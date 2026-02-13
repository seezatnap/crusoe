//! Dialogue-tag, POV, and character-voice consistency checks with confidence scoring.

use analysis_contracts::{
    AnalysisBinary, AnalysisError, AnalysisInput, AnalysisReport, Finding, Severity, SourceLocation,
};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;

/// Default confidence for an explicit named dialogue tag (`she said, "...`).
pub const DEFAULT_EXPLICIT_NAME_CONFIDENCE: f32 = 0.95;
/// Default confidence for a pronoun-only attribution (`he said`).
pub const DEFAULT_PRONOUN_TAG_CONFIDENCE: f32 = 0.42;
/// Default confidence for continuity-based continuation attribution.
pub const DEFAULT_CONTINUATION_CONFIDENCE: f32 = 0.72;

/// Confidence below this and above warning threshold still creates a warning-level violation.
pub const DEFAULT_WARNING_CONFIDENCE: f32 = 0.70;
/// Confidence below this creates an error-level violation.
pub const DEFAULT_ERROR_CONFIDENCE: f32 = 0.45;
/// Confidence below this creates a blocker-level violation.
pub const DEFAULT_BLOCKER_CONFIDENCE: f32 = 0.25;

/// Lexical similarity threshold for mild voice drift warnings.
pub const DEFAULT_VOICE_WARNING_SIMILARITY: f32 = 0.55;
/// Lexical similarity threshold for error voice drift.
pub const DEFAULT_VOICE_ERROR_SIMILARITY: f32 = 0.40;
/// Lexical similarity threshold for blocker voice drift.
pub const DEFAULT_VOICE_BLOCKER_SIMILARITY: f32 = 0.25;

/// If two candidates are within this confidence margin, report ambiguity.
pub const DEFAULT_AMBIGUITY_MARGIN: f32 = 0.12;

/// Require at least this many words of historical profile before running voice checks.
pub const DEFAULT_VOICE_MIN_SAMPLE_WORDS: usize = 24;

/// Number of top words used in each character profile comparison.
pub const DEFAULT_VOICE_TOP_WORDS: usize = 8;

#[derive(Debug, Clone)]
pub struct VoiceConsistencyConfig {
    pub explicit_name_confidence: f32,
    pub pronoun_tag_confidence: f32,
    pub continuation_confidence: f32,
    pub warning_confidence: f32,
    pub error_confidence: f32,
    pub blocker_confidence: f32,
    pub ambiguity_margin: f32,
    pub voice_warning_similarity: f32,
    pub voice_error_similarity: f32,
    pub voice_blocker_similarity: f32,
    pub voice_min_sample_words: usize,
    pub voice_top_words: usize,
    pub artifact_name_prefix: &'static str,
}

impl Default for VoiceConsistencyConfig {
    fn default() -> Self {
        Self {
            explicit_name_confidence: DEFAULT_EXPLICIT_NAME_CONFIDENCE,
            pronoun_tag_confidence: DEFAULT_PRONOUN_TAG_CONFIDENCE,
            continuation_confidence: DEFAULT_CONTINUATION_CONFIDENCE,
            warning_confidence: DEFAULT_WARNING_CONFIDENCE,
            error_confidence: DEFAULT_ERROR_CONFIDENCE,
            blocker_confidence: DEFAULT_BLOCKER_CONFIDENCE,
            ambiguity_margin: DEFAULT_AMBIGUITY_MARGIN,
            voice_warning_similarity: DEFAULT_VOICE_WARNING_SIMILARITY,
            voice_error_similarity: DEFAULT_VOICE_ERROR_SIMILARITY,
            voice_blocker_similarity: DEFAULT_VOICE_BLOCKER_SIMILARITY,
            voice_min_sample_words: DEFAULT_VOICE_MIN_SAMPLE_WORDS,
            voice_top_words: DEFAULT_VOICE_TOP_WORDS,
            artifact_name_prefix: "voice-consistency",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum AttributionSource {
    ExplicitName,
    ExplicitPronoun,
    Continuation,
}

impl AttributionSource {
    fn label(&self) -> &'static str {
        match self {
            Self::ExplicitName => "explicit named tag",
            Self::ExplicitPronoun => "pronoun-based tag",
            Self::Continuation => "continuation inference",
        }
    }
}

#[derive(Debug, Clone)]
struct SpeakerCandidate {
    name: String,
    confidence: f32,
    source: AttributionSource,
}

impl SpeakerCandidate {
    fn new(name: impl Into<String>, source: AttributionSource, confidence: f32) -> Self {
        Self {
            name: normalize_name(name),
            confidence: confidence.clamp(0.0, 1.0),
            source,
        }
    }
}

#[derive(Debug, Clone)]
struct CharacterProfile {
    utterance_count: usize,
    words_seen: usize,
    exclamation_count: usize,
    question_count: usize,
    confidence_sum: f32,
    word_frequencies: HashMap<String, usize>,
}

impl CharacterProfile {
    fn new() -> Self {
        Self {
            utterance_count: 0,
            words_seen: 0,
            exclamation_count: 0,
            question_count: 0,
            confidence_sum: 0.0,
            word_frequencies: HashMap::new(),
        }
    }

    fn record_utterance(&mut self, words: &[String], exclamations: u32, questions: u32, confidence: f32) {
        self.utterance_count += 1;
        self.words_seen += words.len();
        self.exclamation_count += exclamations as usize;
        self.question_count += questions as usize;
        self.confidence_sum += confidence;

        for word in words {
            if is_stop_word(word) {
                continue;
            }
            *self.word_frequencies.entry(word.to_ascii_lowercase()).or_insert(0) += 1;
        }
    }

    fn top_words(&self, limit: usize) -> Vec<String> {
        let mut top: Vec<(String, usize)> = self
            .word_frequencies
            .iter()
            .map(|(word, count)| (word.clone(), *count))
            .collect();
        top.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
        top.into_iter().take(limit).map(|(word, _)| word).collect()
    }

    fn average_confidence(&self) -> f32 {
        if self.utterance_count == 0 {
            0.0
        } else {
            self.confidence_sum / self.utterance_count as f32
        }
    }
}

#[derive(Debug)]
struct NamedState {
    last_seen_line: u32,
    profile: CharacterProfile,
}

#[derive(Debug)]
struct AnalyzerState {
    characters: HashMap<String, NamedState>,
    last_named_speaker: Option<String>,
    total_dialogue_blocks: usize,
    untagged_dialogue_blocks: usize,
    ambiguous_dialogue_blocks: usize,
}

impl AnalyzerState {
    fn new() -> Self {
        Self {
            characters: HashMap::new(),
            last_named_speaker: None,
            total_dialogue_blocks: 0,
            untagged_dialogue_blocks: 0,
            ambiguous_dialogue_blocks: 0,
        }
    }

    fn touch_character(
        &mut self,
        name: &str,
        line: u32,
        words: &[String],
        exclamations: u32,
        questions: u32,
        confidence: f32,
    ) {
        let state = self
            .characters
            .entry(name.to_string())
            .or_insert_with(|| NamedState {
                last_seen_line: line,
                profile: CharacterProfile::new(),
            });
        state.last_seen_line = line;
        state.profile.record_utterance(words, exclamations, questions, confidence);
    }

    fn character_profile(&self, name: &str) -> Option<&CharacterProfile> {
        self.characters.get(name).map(|state| &state.profile)
    }
}

pub struct VoiceConsistencyAnalyzer {
    config: VoiceConsistencyConfig,
}

impl Default for VoiceConsistencyAnalyzer {
    fn default() -> Self {
        Self {
            config: VoiceConsistencyConfig::default(),
        }
    }
}

impl VoiceConsistencyAnalyzer {
    pub fn new(config: VoiceConsistencyConfig) -> Self {
        Self { config }
    }

    pub fn analyze_path(&self, input: &AnalysisInput) -> Result<AnalysisReport, AnalysisError> {
        input.validate_target_exists()?;

        let target = input.resolve_target_path();
        let content = fs::read_to_string(&target).map_err(|err| AnalysisError::Io {
            path: target.display().to_string(),
            reason: err.to_string(),
        })?;

        let mut report = AnalysisReport::new("voice-consistency", &target)
            .add_metadata("analyzer", "voice-consistency")
            .add_metadata("target", target.display().to_string())
            .add_metadata(
                "explicit_name_confidence",
                format!("{:.2}", self.config.explicit_name_confidence),
            )
            .add_metadata(
                "pronoun_tag_confidence",
                format!("{:.2}", self.config.pronoun_tag_confidence),
            )
            .add_metadata(
                "continuation_confidence",
                format!("{:.2}", self.config.continuation_confidence),
            )
            .add_metadata(
                "warning_confidence",
                format!("{:.2}", self.config.warning_confidence),
            )
            .add_metadata("error_confidence", format!("{:.2}", self.config.error_confidence))
            .add_metadata(
                "blocker_confidence",
                format!("{:.2}", self.config.blocker_confidence),
            )
            .add_metadata(
                "ambiguous_margin",
                format!("{:.2}", self.config.ambiguity_margin),
            )
            .add_metadata(
                "voice_warning_similarity",
                format!("{:.2}", self.config.voice_warning_similarity),
            )
            .add_metadata(
                "voice_error_similarity",
                format!("{:.2}", self.config.voice_error_similarity),
            )
            .add_metadata(
                "voice_blocker_similarity",
                format!("{:.2}", self.config.voice_blocker_similarity),
            )
            .add_metadata("voice_min_sample_words", self.config.voice_min_sample_words.to_string())
            .add_metadata("voice_top_words", self.config.voice_top_words.to_string());

        for (key, value) in &input.options {
            report = report.add_metadata(format!("option:{key}"), value.clone());
        }

        let (paragraphs, marker_lines) = split_paragraphs_with_markers(&content);
        let mut state = AnalyzerState::new();
        let mut active_pov: Option<String> = None;

        for (index, paragraph) in paragraphs.iter().enumerate() {
            let line = paragraph.0;
            let text = paragraph.1.trim();
            if text.is_empty() {
                continue;
            }

            if let Some(marker) = marker_lines.get(index).and_then(|name| name.clone()) {
                active_pov = Some(marker.clone());
            }

            if !is_dialogue_paragraph(text) {
                if active_pov.is_none() && has_first_person_cues(text) {
                    report.findings.push(
                        Finding::new(
                            "POV-MARKER-001".to_string(),
                            "First-person narration has no visible POV marker (`<!-- pov: <name> -->`).".to_string(),
                            Severity::Info,
                        )
                        .with_location(SourceLocation::new(&target, Some(line), Some(1)))
                        .with_suggestion(
                            "Add a POV marker before this paragraph or confirm expected focalization shift in your chapter metadata.".to_string(),
                        ),
                    );
                }
                continue;
            }

            state.total_dialogue_blocks += 1;
            let mut candidates = extract_speakers(text, &self.config);

            if candidates.is_empty() {
                if let Some(last) = state.last_named_speaker.clone() {
                    candidates.push(SpeakerCandidate::new(
                        last,
                        AttributionSource::Continuation,
                        self.config.continuation_confidence,
                    ));
                }
            }

            let Some((selected, ambiguous)) = pick_top_candidate(&candidates, self.config.ambiguity_margin) else {
                state.untagged_dialogue_blocks += 1;
                report.findings.push(
                    Finding::new(
                        "DIAL-TAG-001".to_string(),
                        "Dialogue paragraph has no candidate speaker attribution; confidence could not be established."
                            .to_string(),
                        Severity::Warning,
                    )
                    .with_location(SourceLocation::new(&target, Some(line), Some(1)))
                    .with_suggestion(
                        "Add a dialogue tag (named speaker or clear attribution) or keep character state explicit nearby."
                            .to_string(),
                    ),
                );
                continue;
            };

            if ambiguous {
                state.ambiguous_dialogue_blocks += 1;
                report.findings.push(
                    Finding::new(
                        "DIAL-AMBIG-001".to_string(),
                        format!(
                            "Ambiguous dialogue attribution: multiple candidates around line {}, likely '{}' with confidence {:.2}.",
                            line, selected.name, selected.confidence
                        ),
                        severity_for_confidence(
                            selected.confidence,
                            self.config.warning_confidence,
                            self.config.error_confidence,
                            self.config.blocker_confidence,
                        ),
                    )
                    .with_location(SourceLocation::new(&target, Some(line), Some(1)))
                    .with_suggestion(
                        "Use a stronger tag so named-character tracking is unambiguous."
                            .to_string(),
                    ),
                );
            }

            if selected.confidence < self.config.warning_confidence
                && selected.source != AttributionSource::ExplicitName
            {
                report.findings.push(
                    Finding::new(
                        "DIAL-TAG-001".to_string(),
                        format!(
                            "Speaker attribution is weak ({:.2}) for '{}'; source is {}.",
                            selected.confidence, selected.name, selected.source.label()
                        ),
                        severity_for_confidence(
                            selected.confidence,
                            self.config.warning_confidence,
                            self.config.error_confidence,
                            self.config.blocker_confidence,
                        ),
                    )
                    .with_location(SourceLocation::new(&target, Some(line), Some(1)))
                    .with_suggestion(format!(
                        "Replace weak or pronoun-only tag with a named character tag for stronger continuity tracking."
                    )),
                );
            } else if selected.source == AttributionSource::Continuation {
                report.findings.push(
                    Finding::new(
                        "DIAL-TAG-002".to_string(),
                        format!(
                            "Dialogue uses continuation inference for '{}'; confidence {:.2}.",
                            selected.name, selected.confidence
                        ),
                        Severity::Info,
                    )
                    .with_location(SourceLocation::new(&target, Some(line), Some(1)))
                    .with_suggestion(
                        "Add an explicit dialogue tag when the speaker switches or scene changes."
                            .to_string(),
                    ),
                );
            }

            let raw_dialogue = extract_dialogue_body(text);
            let tokens = tokenize_content(&raw_dialogue);
            let (excl, q) = count_question_like_words(text);

            if let Some(previous) = state.character_profile(&selected.name).cloned() {
                if previous.utterance_count > 1 && previous.words_seen >= self.config.voice_min_sample_words {
                    let signature_sim = voice_signature_similarity(
                        &previous,
                        &tokens,
                        self.config.voice_top_words,
                        excl,
                        q,
                    );
                    if let Some(similarity) = signature_sim {
                        if similarity < self.config.voice_warning_similarity {
                            report.findings.push(
                                Finding::new(
                                    "VOICE-STYLE-001".to_string(),
                                    format!(
                                        "Voice drift risk for '{}' (similarity {:.2}, weighted confidence {:.2}).",
                                        selected.name,
                                        similarity,
                                        selected.confidence
                                    ),
                                    severity_from_similarity(
                                        similarity,
                                        self.config.voice_warning_similarity,
                                        self.config.voice_error_similarity,
                                        self.config.voice_blocker_similarity,
                                    ),
                                )
                                .with_location(SourceLocation::new(&target, Some(line), Some(1)))
                                .with_suggestion(
                                    "Reconcile diction and rhythm with prior dialogue from this character."
                                        .to_string(),
                                ),
                            );
                        }
                    }
                }
            }

            state.touch_character(
                &selected.name,
                line,
                &tokens,
                excl,
                q,
                selected.confidence,
            );
            state.last_named_speaker = Some(selected.name.clone());

            if let Some(pov) = active_pov.as_ref() {
                if pov.to_ascii_lowercase() != selected.name {
                    let score = selected.confidence;
                    if score < 0.80 {
                        report.findings.push(
                            Finding::new(
                                "POV-DRIFT-001".to_string(),
                                format!(
                                    "Dialogue speaker '{}' does not match active POV marker '{}'.",
                                    selected.name,
                                    pov
                                ),
                                if score >= self.config.error_confidence {
                                    Severity::Warning
                                } else {
                                    severity_for_confidence(
                                        score,
                                        self.config.warning_confidence,
                                        self.config.error_confidence,
                                        self.config.blocker_confidence,
                                    )
                                },
                            )
                            .with_location(SourceLocation::new(&target, Some(line), Some(1)))
                            .with_suggestion(
                                "If the POV changed, insert a POV marker before this dialogue block."
                                    .to_string(),
                            ),
                        );
                    }
                }
            }

            // Use profile average confidence as a lightweight continuity quality metric.
            if state
                .character_profile(&selected.name)
                .is_some_and(|profile| profile.average_confidence() < 0.80)
            {
                let profile = state.character_profile(&selected.name).expect("profile must exist");
                report.metadata.insert(
                    format!("{}:avg_confidence", selected.name),
                    format!("{:.2}", profile.average_confidence()),
                );
            }
        }

        report = report
            .add_metadata("total_dialogue_blocks", state.total_dialogue_blocks.to_string())
            .add_metadata("untagged_dialogue_blocks", state.untagged_dialogue_blocks.to_string())
            .add_metadata("ambiguous_dialogue_blocks", state.ambiguous_dialogue_blocks.to_string())
            .add_metadata("tracked_characters", state.characters.len().to_string());

        let characters: Vec<_> = state.characters.keys().cloned().collect();
        report = report.add_metadata("character_list", characters.join(", "));
        Ok(report)
    }

    pub fn analyze(&self, input: &AnalysisInput) -> Result<AnalysisReport, AnalysisError> {
        self.analyze_path(input)
    }
}

impl AnalysisBinary for VoiceConsistencyAnalyzer {
    fn analyzer_name(&self) -> &'static str {
        self.config.artifact_name_prefix
    }

    fn run(&self, input: &AnalysisInput) -> Result<AnalysisReport, AnalysisError> {
        self.analyze_path(input)
    }
}

fn severity_for_confidence(
    confidence: f32,
    warning: f32,
    error: f32,
    blocker: f32,
) -> Severity {
    if confidence <= blocker {
        Severity::Blocker
    } else if confidence <= error {
        Severity::Error
    } else if confidence <= warning {
        Severity::Warning
    } else {
        Severity::Info
    }
}

fn severity_from_similarity(
    similarity: f32,
    warning: f32,
    error: f32,
    blocker: f32,
) -> Severity {
    if similarity <= blocker {
        Severity::Blocker
    } else if similarity <= error {
        Severity::Error
    } else if similarity <= warning {
        Severity::Warning
    } else {
        Severity::Info
    }
}

fn split_paragraphs_with_markers(content: &str) -> (Vec<(u32, String)>, Vec<Option<String>>) {
    let normalized = content.replace("\r\n", "\n").replace('\r', "\n");
    let mut out = Vec::new();
    let mut markers = Vec::new();
    let mut current_lines = Vec::new();
    let mut cursor = 1u32;
    let mut current_start_line = None::<u32>;

    for line in normalized.lines() {
        if line.trim().is_empty() {
            if !current_lines.is_empty() {
                let text = compact_ws(&current_lines.join("\n"));
                if let Some(start_line) = current_start_line {
                    out.push((start_line, text));
                    let lines: Vec<&str> = current_lines.iter().map(|line: &String| line.as_str()).collect();
                    markers.push(pov_marker_for_paragraph(lines));
                }
                current_lines.clear();
                current_start_line = None;
            }
            cursor = cursor.saturating_add(1);
            continue;
        }

        if current_start_line.is_none() {
            current_start_line = Some(cursor);
        }

        current_lines.push(line.to_string());
        cursor = cursor.saturating_add(1);
    }

    if !current_lines.is_empty() {
        let start_line = current_start_line.unwrap_or(cursor);
        let text = compact_ws(&current_lines.join("\n"));
        out.push((start_line, text));
        let lines: Vec<&str> = current_lines.iter().map(|line: &String| line.as_str()).collect();
        markers.push(pov_marker_for_paragraph(lines));
    } else {
        markers.push(None);
    }

    if out.is_empty() {
        return (Vec::new(), Vec::new());
    }

    while markers.len() < out.len() {
        markers.push(None);
    }

    (out, markers)
}

fn compact_ws(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn pov_marker_for_paragraph(lines: Vec<&str>) -> Option<String> {
    let mut found = None;
    for raw in lines {
        if let Some(marker) = parse_pov_marker(raw) {
            found = Some(marker);
        }
    }
    found
}

fn parse_pov_marker(line: &str) -> Option<String> {
    let line = line.trim();
    let lower = line.to_ascii_lowercase();
    let prefixes = ["<!-- pov:", "<!-- pov =", "<!-- pov ="];
    for prefix in prefixes {
        if lower.starts_with(prefix) {
            let remainder = &lower[prefix.len()..];
            let remainder = remainder.split("-->").next().unwrap_or(remainder).trim();
            if remainder.is_empty() {
                return None;
            }
            let remainder = remainder
                .trim_matches('-')
                .trim_matches('/')
                .trim_matches('<')
                .trim_matches('>');
            if remainder.is_empty() {
                return None;
            }
            return Some(remainder.to_string());
        }
    }
    None
}

fn is_dialogue_paragraph(text: &str) -> bool {
    text.contains('"') || text.contains('“') || text.contains('”')
}

fn tokenize_content(text: &str) -> Vec<String> {
    text.to_ascii_lowercase()
        .split(|ch: char| {
            !ch.is_ascii_alphabetic() && ch != '\'' && ch != '-'
        })
        .filter(|token| !token.trim().is_empty())
        .map(|token| token.to_string())
        .collect()
}

fn extract_dialogue_body(text: &str) -> String {
    let open = text.find('"').or_else(|| text.find('“'));
    let close = text.rfind('"').or_else(|| text.rfind('”')).or_else(|| text.rfind('\"'));
    if let (Some(open), Some(close)) = (open, close) {
        if close > open + 1 {
            return text[open + 1..close].to_string();
        }
    }
    text.to_string()
}

fn count_question_like_words(text: &str) -> (u32, u32) {
    let exclamation_count = u32::try_from(text.matches('!').count()).unwrap_or(u32::MAX);
    let question_count = u32::try_from(text.matches('?').count()).unwrap_or(u32::MAX);
    (exclamation_count, question_count)
}

fn normalize_name(name: impl Into<String>) -> String {
    let name = name.into();
    name.split_whitespace()
        .filter(|part| !part.trim().is_empty())
        .map(|part| part.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join(" ")
}

fn is_quote_verb(candidate: &str) -> bool {
    matches!(
        candidate,
        "said"
            | "asked"
            | "whispered"
            | "murmured"
            | "breathed"
            | "replied"
            | "answered"
            | "observed"
            | "called"
            | "yelled"
            | "shouted"
            | "interrupted"
            | "inquired"
            | "mused"
            | "sighed"
    )
}

fn is_name_word(word: &str) -> bool {
    let first = match word.chars().next() {
        Some(ch) => ch,
        None => return false,
    };
    if !first.is_ascii_uppercase() {
        return false;
    }
    if word == "I" {
        return false;
    }
    !matches!(
        word.to_ascii_lowercase().as_str(),
        "he" | "she" | "i" | "we" | "they" | "you" | "him" | "her" | "it" | "us" | "me"
    )
}

fn extract_name_sequence_before(tokens: &[&str], verb_index: usize) -> Option<String> {
    if verb_index == 0 {
        return None;
    }
    let mut start = verb_index;
    while start > 0 && is_name_word(tokens[start - 1]) {
        start -= 1;
    }
    if start == verb_index {
        return None;
    }
    let end = verb_index;
    Some(tokens[start..end].join(" "))
}

fn extract_name_sequence_after(tokens: &[&str], verb_index: usize) -> Option<String> {
    let mut index = verb_index + 1;
    while index < tokens.len() && tokens[index].is_empty() {
        index += 1;
    }
    if index >= tokens.len() {
        return None;
    }

    let first = tokens[index];
    if is_name_word(first) {
        let mut end = index + 1;
        while end < tokens.len() && is_name_word(tokens[end]) {
            end += 1;
        }
        return Some(tokens[index..end].join(" "));
    }

    if is_pronoun(first) {
        return Some(first.to_string());
    }

    None
}

fn is_pronoun(word: &str) -> bool {
    matches!(
        word.to_ascii_lowercase().as_str(),
        "i" | "he" | "she" | "they" | "we" | "you" | "it" | "hmm"
    )
}

fn has_first_person_cues(text: &str) -> bool {
    let tokens = tokenize_content(text);
    tokens.iter().any(|token| {
        matches!(
            token.as_str(),
            "i" | "me" | "my" | "mine" | "myself" | "i'm" | "i’ve" | "i’d" | "i’ll" | "i've" | "i'd"
        )
    })
}

fn is_stop_word(word: &str) -> bool {
    matches!(
        word,
        "the" | "a" | "an" | "and" | "or" | "to" | "of"
            | "in" | "on" | "at" | "for" | "with" | "as" | "by" | "that"
            | "this" | "it" | "his" | "her" | "they" | "them" | "we"
            | "us" | "i" | "me" | "my" | "you" | "was" | "were" | "is" | "are"
            | "am" | "be" | "been" | "from" | "not" | "but"
    )
}

fn extract_speakers(text: &str, config: &VoiceConsistencyConfig) -> Vec<SpeakerCandidate> {
    let tokens_raw: Vec<&str> = text
        .split(|ch: char| {
            !ch.is_ascii_alphabetic() && ch != '\'' && ch != '-'
        })
        .filter(|token| !token.is_empty())
        .collect();

    let mut candidates = Vec::new();
    for (index, token) in tokens_raw.iter().enumerate() {
        if !is_quote_verb(&token.to_ascii_lowercase()) {
            continue;
        }

        if let Some(before) = extract_name_sequence_before(&tokens_raw, index) {
            candidates.push(SpeakerCandidate::new(
                before,
                AttributionSource::ExplicitName,
                config.explicit_name_confidence,
            ));
        }

        if let Some(after) = extract_name_sequence_after(&tokens_raw, index) {
            if is_pronoun(&after) {
                candidates.push(SpeakerCandidate::new(
                    after,
                    AttributionSource::ExplicitPronoun,
                    config.pronoun_tag_confidence,
                ));
            } else {
                candidates.push(SpeakerCandidate::new(
                    after,
                    AttributionSource::ExplicitName,
                    config.explicit_name_confidence,
                ));
            }
        }
    }

    deduplicate_candidates(candidates)
}

fn deduplicate_candidates(candidates: Vec<SpeakerCandidate>) -> Vec<SpeakerCandidate> {
    let mut best = BTreeMap::<String, SpeakerCandidate>::new();
    for candidate in candidates {
        best.entry(candidate.name.clone())
            .and_modify(|existing| {
                if candidate.confidence > existing.confidence {
                    *existing = candidate.clone();
                }
            })
            .or_insert(candidate);
    }
    best.into_values().collect()
}

fn pick_top_candidate(
    candidates: &[SpeakerCandidate],
    ambiguity_margin: f32,
) -> Option<(SpeakerCandidate, bool)> {
    if candidates.is_empty() {
        return None;
    }

    let mut sorted = candidates.to_vec();
    sorted.sort_by(|left, right| {
        right
            .confidence
            .partial_cmp(&left.confidence)
            .unwrap_or(Ordering::Equal)
            .then_with(|| left.name.cmp(&right.name))
    });

    let top = sorted[0].clone();
    let ambiguous = sorted
        .get(1)
        .is_some_and(|second| (top.confidence - second.confidence).abs() <= ambiguity_margin);

    Some((top, ambiguous))
}

fn voice_signature_similarity(
    profile: &CharacterProfile,
    current_words: &[String],
    top_word_count: usize,
    exclamation_count: u32,
    question_count: u32,
) -> Option<f32> {
    if profile.utterance_count < 2 || top_word_count == 0 {
        return None;
    }

    let mut current_set: HashSet<String> = HashSet::new();
    for word in current_words {
        if is_stop_word(word) {
            continue;
        }
        current_set.insert(word.to_ascii_lowercase());
    }

    let profile_set: HashSet<String> = profile
        .top_words(top_word_count)
        .into_iter()
        .collect();

    if current_set.is_empty() || profile_set.is_empty() {
        return None;
    }

    let intersection = current_set.intersection(&profile_set).count() as f32;
    let union = current_set.union(&profile_set).count() as f32;
    let lexical = intersection / union;

    let profile_utterances = u32::try_from(profile.utterance_count).unwrap_or(1);
    let profile_excl_share = if profile_utterances == 0 {
        0.0
    } else {
        profile.exclamation_count as f32 / profile_utterances as f32
    };
    let profile_q_share = if profile_utterances == 0 {
        0.0
    } else {
        profile.question_count as f32 / profile_utterances as f32
    };

    let utterance_count = if current_words.is_empty() {
        1.0
    } else {
        current_words.len() as f32
    };
    let current_excl_share = exclamation_count as f32 / utterance_count;
    let current_q_share = question_count as f32 / utterance_count;

    let punctuation_penalty =
        ((profile_excl_share - current_excl_share).abs() + (profile_q_share - current_q_share).abs())
            / 2.0;
    let punctuation_similarity = (1.0 - punctuation_penalty).max(0.0);

    Some((0.75 * lexical) + (0.25 * punctuation_similarity))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_file(contents: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        path.push(format!("voice-consistency-tests-{nanos}.md"));
        std::fs::write(&path, contents).expect("write test file");
        path
    }

    fn run_analysis(contents: &str, config: VoiceConsistencyConfig) -> AnalysisReport {
        let analyzer = VoiceConsistencyAnalyzer::new(config);
        let path = temp_file(contents);
        let input = AnalysisInput::new(path);
        analyzer.run(&input).expect("analysis")
    }

    #[test]
    fn extracts_named_speaker_candidates() {
        let config = VoiceConsistencyConfig::default();
        let candidates = extract_speakers("\"I will see it soon,\" Lena said.", &config);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].name, "lena");
        assert_eq!(candidates[0].source, AttributionSource::ExplicitName);
        assert_eq!(candidates[0].confidence, config.explicit_name_confidence);
    }

    #[test]
    fn detects_weak_dialogue_tag() {
        let report = run_analysis(
            "\"You are sure?\" she whispered.",
            VoiceConsistencyConfig::default(),
        );
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.code == "DIAL-TAG-001"));
    }

    #[test]
    fn detects_untagged_dialogue_blocks() {
        let report = run_analysis(
            "\"It happens each cycle.\"

Narrative continues here without tag.

The sentence turns.",
            VoiceConsistencyConfig::default(),
        );
        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.code == "DIAL-TAG-001")
        );
    }

    #[test]
    fn tracks_character_voice_similarity() {
        let mut profile = CharacterProfile::new();
        profile.record_utterance(
            &tokenize_content("keep the lantern close and check the door"),
            0,
            0,
            0.95,
        );
        profile.record_utterance(
            &tokenize_content("keep the lantern close and check the door"),
            0,
            0,
            0.95,
        );
        profile.record_utterance(
            &tokenize_content("quantum architecture drifts through the glassy matrix"),
            0,
            0,
            0.95,
        );

        let stable = voice_signature_similarity(&profile, &tokenize_content("check the lantern and keep it warm"), 4, 0, 0)
            .expect("stable speech should produce a score");
        let drift = voice_signature_similarity(
            &profile,
            &tokenize_content("quantum architecture drifts through the glassy matrix"),
            4,
            0,
            0,
        )
        .expect("drift speech should produce a score");

        assert!(stable >= drift);
    }

    #[test]
    fn resolves_relative_target_with_working_directory_and_reads_content() {
        let mut workdir = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        workdir.push(format!("voice-consistency-tests-{nanos}"));
        std::fs::create_dir_all(&workdir).unwrap();
        let chapter_dir = workdir.join("chapter-files");
        std::fs::create_dir_all(&chapter_dir).unwrap();

        let expected = chapter_dir.join("chapter-01.md");
        std::fs::write(
            &expected,
            "\"It happens each cycle.\"\n\nNarrative continues here without tag.\n\nThe sentence turns.",
        )
        .unwrap();

        let analyzer = VoiceConsistencyAnalyzer::new(VoiceConsistencyConfig::default());
        let input = AnalysisInput::new("chapter-files/chapter-01.md").with_working_directory(&workdir);
        let report = analyzer.run(&input).expect("analysis should resolve working-directory target");

        assert_eq!(report.target, expected);
        assert_eq!(
            report
                .metadata
                .get("target")
                .expect("target metadata should be present"),
            &expected.display().to_string()
        );
        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.code == "DIAL-TAG-001"),
            "expected untagged dialogue finding from the chapter content"
        );

        let _ = std::fs::remove_dir_all(&workdir);
    }
}
