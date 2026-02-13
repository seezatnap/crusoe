use std::error::Error;
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use rand::seq::SliceRandom;

#[derive(Clone, Copy)]
pub enum ExtractMode {
    Narrative,
    Dialogue,
    Page,
    ChapterStart,
    ChapterEnd,
}

pub const STYLE_REFERENCE_URLS: &[&str] = &["https://dgoldberg.sdsu.edu/515/harrypotter.txt"];

const DEFAULT_PAGE_WORDS: usize = 250;
const PAGE_LINES: usize = 30;
const MIN_PAGE_WORDS: usize = 140;
const MAX_PAGE_WORDS: usize = 420;
const MIN_CHAPTER_EDGE_WORDS: usize = 50;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LengthTier {
    Short,
    Medium,
    Long,
}

#[derive(Debug)]
pub enum ExtractError {
    Io(std::io::Error),
    NoMatchingFiles(PathBuf),
    DownloadFailed { url: String, reason: String },
}

impl fmt::Display for ExtractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::NoMatchingFiles(path) => {
                write!(f, "No readable text files found in {}", path.display())
            }
            Self::DownloadFailed { url, reason } => {
                write!(f, "Failed to download {url}: {reason}")
            }
        }
    }
}

impl Error for ExtractError {}

impl From<std::io::Error> for ExtractError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

fn is_text_file(path: &Path) -> bool {
    match path.extension().and_then(OsStr::to_str) {
        Some(ext) => matches!(ext.to_ascii_lowercase().as_str(), "txt" | "md" | "markdown"),
        None => false,
    }
}

fn normalized_lines(content: &str) -> String {
    content.replace("\r\n", "\n").replace('\r', "\n")
}

fn compact_paragraph(raw: &str) -> String {
    raw.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn split_paragraphs(content: &str) -> Vec<String> {
    let normalized = normalized_lines(content);
    let mut paragraphs = Vec::new();
    let mut current = Vec::new();

    for line in normalized.lines() {
        if line.trim().is_empty() {
            if !current.is_empty() {
                paragraphs.push(compact_paragraph(&current.join("\n")));
                current.clear();
            }
        } else {
            current.push(line.trim_end_matches('\r').to_string());
        }
    }

    if !current.is_empty() {
        paragraphs.push(compact_paragraph(&current.join("\n")));
    }

    paragraphs.into_iter().filter(|p| !p.is_empty()).collect()
}

fn is_chapter_heading_line(line: &str) -> bool {
    let trimmed = line.trim();
    if !trimmed.starts_with("CHAPTER ") {
        return false;
    }
    !trimmed.chars().any(|ch| ch.is_ascii_lowercase())
}

fn is_chapter_heading_paragraph(paragraph: &str) -> bool {
    is_chapter_heading_line(paragraph)
}

fn split_chapters(content: &str) -> Vec<String> {
    let normalized = normalized_lines(content);
    let mut chapters = Vec::new();
    let mut current = Vec::new();
    let mut saw_heading = false;

    for line in normalized.lines() {
        if is_chapter_heading_line(line) {
            if saw_heading && !current.is_empty() {
                chapters.push(current.join("\n"));
            }
            current.clear();
            saw_heading = true;
            continue;
        }

        current.push(line.to_string());
    }

    if !current.is_empty() {
        chapters.push(current.join("\n"));
    }

    if saw_heading {
        chapters
            .into_iter()
            .filter(|chapter| !chapter.trim().is_empty())
            .collect()
    } else if normalized.trim().is_empty() {
        Vec::new()
    } else {
        vec![normalized]
    }
}

fn chapter_edge_paragraphs(
    paragraphs: &[String],
    from_start: bool,
    min_words: usize,
) -> Option<String> {
    if paragraphs.is_empty() {
        return None;
    }

    let mut chosen = Vec::new();
    let mut words = 0usize;

    if from_start {
        for paragraph in paragraphs {
            if is_chapter_heading_paragraph(paragraph) {
                continue;
            }
            words += word_count(paragraph);
            chosen.push(paragraph.clone());
            if words >= min_words {
                break;
            }
        }
    } else {
        for paragraph in paragraphs.iter().rev() {
            if is_chapter_heading_paragraph(paragraph) {
                continue;
            }
            words += word_count(paragraph);
            chosen.push(paragraph.clone());
            if words >= min_words {
                break;
            }
        }
        chosen.reverse();
    }

    if chosen.is_empty() {
        None
    } else {
        Some(chosen.join("\n\n"))
    }
}

fn extract_chapter_edge_units_from_text(text: &str, from_start: bool) -> Vec<String> {
    let mut units = Vec::new();
    for chapter in split_chapters(text) {
        let paragraphs = split_paragraphs(&chapter);
        if let Some(unit) = chapter_edge_paragraphs(&paragraphs, from_start, MIN_CHAPTER_EDGE_WORDS)
        {
            units.push(unit);
        }
    }
    units
}

fn extract_page_units_from_text(text: &str, page_word_target: usize) -> Vec<String> {
    let paragraphs = split_paragraphs(text);
    let mut units = Vec::new();

    if paragraphs.is_empty() {
        return units;
    }

    for start in 0..paragraphs.len() {
        if is_chapter_heading_paragraph(&paragraphs[start]) {
            continue;
        }

        let mut words = 0usize;
        let mut chunk = Vec::new();
        let mut index = start;

        while index < paragraphs.len() {
            let paragraph = &paragraphs[index];
            if is_chapter_heading_paragraph(paragraph) {
                if !chunk.is_empty() {
                    break;
                }
                index += 1;
                continue;
            }

            words += word_count(paragraph);
            chunk.push(paragraph.clone());
            index += 1;

            if words >= page_word_target {
                break;
            }
        }

        if !chunk.is_empty() {
            units.push(chunk.join("\n\n"));
        }
    }

    units
}

fn estimate_average_page_words(texts: &[String]) -> usize {
    let mut total_words = 0usize;
    let mut total_lines = 0usize;

    for text in texts {
        let normalized = normalized_lines(text);
        for line in normalized.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || is_chapter_heading_line(trimmed) {
                continue;
            }

            let words = word_count(trimmed);
            if words == 0 {
                continue;
            }

            total_words += words;
            total_lines += 1;
        }
    }

    if total_lines == 0 {
        return DEFAULT_PAGE_WORDS;
    }

    let average_line_words = total_words as f64 / total_lines as f64;
    let page_words = (average_line_words * PAGE_LINES as f64).round() as usize;
    page_words.clamp(MIN_PAGE_WORDS, MAX_PAGE_WORDS)
}

fn source_filename(url: &str, index: usize) -> String {
    let mut trimmed = url.trim_end_matches('/').to_string();
    if let Some(fragment_index) = trimmed.find('#') {
        trimmed.truncate(fragment_index);
    }
    if let Some(query_index) = trimmed.find('?') {
        trimmed.truncate(query_index);
    }

    let base = trimmed
        .rsplit('/')
        .next()
        .unwrap_or("style-reference-source")
        .to_string();

    let base = base.chars().map(|ch| {
        if ch.is_ascii_alphanumeric() || ch == '.' || ch == '-' || ch == '_' {
            ch
        } else {
            '_'
        }
    });
    let mut base: String = base.collect();
    if base.is_empty() {
        base.push_str("style-reference-source");
    }

    if !base.ends_with(".txt") && !base.contains('.') {
        base.push_str(".txt");
    }

    format!("{index:02}_{base}")
}

fn download_text_file(url: &str, destination: &Path) -> Result<(), ExtractError> {
    if let Ok(metadata) = fs::metadata(destination) {
        if metadata.len() > 0 {
            return Ok(());
        }
        fs::remove_file(destination)?;
    }

    let output = Command::new("curl")
        .arg("-L")
        .arg("-sS")
        .arg("--fail")
        .arg(url)
        .arg("--output")
        .arg(destination)
        .output()
        .map_err(|err| ExtractError::DownloadFailed {
            url: url.to_string(),
            reason: format!("unable to execute curl: {err}"),
        })?;

    if !output.status.success() {
        let status = output.status.code().unwrap_or(-1);
        let message = String::from_utf8_lossy(&output.stderr).into_owned();
        let reason = if message.trim().is_empty() {
            format!("curl exited with status code {status}")
        } else {
            message
        };
        fs::remove_file(destination).ok();
        return Err(ExtractError::DownloadFailed {
            url: url.to_string(),
            reason,
        });
    }

    Ok(())
}

pub fn sync_style_reference_sources(source_dir: &Path) -> Result<(), ExtractError> {
    fs::create_dir_all(source_dir)?;
    for (index, url) in STYLE_REFERENCE_URLS.iter().enumerate() {
        let filename = source_filename(url, index + 1);
        let destination = source_dir.join(filename);
        download_text_file(url, &destination)?;
    }
    Ok(())
}

fn is_dialogue_paragraph(paragraph: &str) -> bool {
    let paragraph = paragraph.trim();
    if paragraph.is_empty() {
        return false;
    }

    let has_straight_quotes = paragraph.matches('"').count() > 0;
    let has_smart_quotes = paragraph.contains('“') || paragraph.contains('”');
    if !has_straight_quotes && !has_smart_quotes {
        return false;
    }

    let starts_with_dialogue_quote =
        paragraph.starts_with('"') || paragraph.starts_with('“') || paragraph.starts_with('`');
    let has_dialogue_tag = has_dialogue_tag_words(paragraph);

    starts_with_dialogue_quote || has_dialogue_tag || has_smart_quotes
}

fn has_dialogue_tag_words(paragraph: &str) -> bool {
    let lower = paragraph.to_lowercase();
    let words = lower
        .split(|c: char| !c.is_ascii_alphabetic())
        .filter(|segment| !segment.is_empty());
    words.into_iter().any(|segment| {
        matches!(
            segment,
            "said"
                | "asked"
                | "answered"
                | "replied"
                | "cried"
                | "murmured"
                | "observed"
                | "breathed"
                | "shouted"
                | "whispered"
                | "added"
                | "interrupted"
                | "inquired"
        )
    })
}

fn has_narrative_punctuation(paragraph: &str) -> bool {
    paragraph
        .chars()
        .any(|ch| matches!(ch, '.' | '!' | '?' | ':' | ';' | ',' | '"' | '”' | '’'))
}

fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

fn length_tier(text: &str) -> LengthTier {
    let count = word_count(text);
    if count <= 75 {
        LengthTier::Short
    } else if count <= 150 {
        LengthTier::Medium
    } else {
        LengthTier::Long
    }
}

fn take_long(long: &mut Vec<String>, require_under_500: bool) -> Option<String> {
    if !require_under_500 {
        return long.pop();
    }

    long.iter()
        .position(|candidate| word_count(candidate) < 500)
        .map(|index| long.swap_remove(index))
}

fn collect_text_files(root: &Path, out: &mut Vec<PathBuf>) -> Result<(), ExtractError> {
    if root.is_file() {
        if is_text_file(root) {
            out.push(root.to_path_buf());
        }
        return Ok(());
    }

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_text_files(&path, out)?;
            continue;
        }

        if is_text_file(&path) {
            out.push(path);
        }
    }

    Ok(())
}

pub fn extract_units_from_text(text: &str, mode: ExtractMode) -> Vec<String> {
    let page_words = DEFAULT_PAGE_WORDS;
    extract_units_with_page_words(text, mode, page_words)
}

fn extract_units_with_page_words(
    text: &str,
    mode: ExtractMode,
    page_word_target: usize,
) -> Vec<String> {
    if matches!(mode, ExtractMode::Page) {
        return extract_page_units_from_text(text, page_word_target);
    }

    if matches!(mode, ExtractMode::ChapterStart) {
        return extract_chapter_edge_units_from_text(text, true);
    }

    if matches!(mode, ExtractMode::ChapterEnd) {
        return extract_chapter_edge_units_from_text(text, false);
    }

    let paragraphs = split_paragraphs(text);
    let mut result = Vec::new();

    if paragraphs.is_empty() {
        return result;
    }

    let mut index = 0;
    while index < paragraphs.len() {
        let is_dialogue = is_dialogue_paragraph(&paragraphs[index]);

        if matches!(mode, ExtractMode::Dialogue) && is_dialogue {
            let start = index;
            let mut end = index + 1;
            while end < paragraphs.len() && is_dialogue_paragraph(&paragraphs[end]) {
                end += 1;
            }
            let unit = paragraphs[start..end].join("\n\n");
            result.push(unit);
            index = end;
            continue;
        }

        if matches!(mode, ExtractMode::Narrative)
            && !is_dialogue
            && has_narrative_punctuation(&paragraphs[index])
        {
            result.push(paragraphs[index].clone());
        }

        index += 1;
    }

    result
}

pub fn collect_examples(source_dir: &Path, mode: ExtractMode) -> Result<Vec<String>, ExtractError> {
    let mut files = Vec::new();
    collect_text_files(source_dir, &mut files)?;

    if files.is_empty() {
        return Err(ExtractError::NoMatchingFiles(source_dir.to_path_buf()));
    }

    files.sort();

    let mut texts = Vec::with_capacity(files.len());
    for file in files {
        let bytes = fs::read(file)?;
        texts.push(String::from_utf8_lossy(&bytes).into_owned());
    }

    let page_word_target = if matches!(mode, ExtractMode::Page) {
        estimate_average_page_words(&texts)
    } else {
        DEFAULT_PAGE_WORDS
    };

    let mut all_examples = Vec::new();
    for text in &texts {
        all_examples.extend(extract_units_with_page_words(text, mode, page_word_target));
    }

    Ok(all_examples)
}

pub fn collect_cached_examples(
    source_dir: &Path,
    mode: ExtractMode,
) -> Result<Vec<String>, ExtractError> {
    sync_style_reference_sources(source_dir)?;
    collect_examples(source_dir, mode)
}

pub fn pick_examples(mut examples: Vec<String>, count: usize) -> Vec<String> {
    if examples.is_empty() || count == 0 {
        return Vec::new();
    }

    let mut short = Vec::new();
    let mut medium = Vec::new();
    let mut long = Vec::new();

    for example in examples.drain(..) {
        match length_tier(&example) {
            LengthTier::Short => short.push(example),
            LengthTier::Medium => medium.push(example),
            LengthTier::Long => long.push(example),
        }
    }

    let mut rng = rand::thread_rng();
    short.shuffle(&mut rng);
    medium.shuffle(&mut rng);
    long.shuffle(&mut rng);

    let mut desired_tiers = Vec::with_capacity(count);
    while desired_tiers.len() < count {
        let mut batch = [LengthTier::Short, LengthTier::Medium, LengthTier::Long];
        batch.shuffle(&mut rng);
        for tier in batch {
            if desired_tiers.len() == count {
                break;
            }
            desired_tiers.push(tier);
        }
    }

    let mut selected = Vec::new();
    let mut require_under_500_for_next_long = false;
    for desired in desired_tiers {
        let candidate = match desired {
            LengthTier::Short => short.pop().or_else(|| medium.pop()).or_else(|| {
                take_long(&mut long, require_under_500_for_next_long).or_else(|| long.pop())
            }),
            LengthTier::Medium => medium.pop().or_else(|| short.pop()).or_else(|| {
                take_long(&mut long, require_under_500_for_next_long).or_else(|| long.pop())
            }),
            LengthTier::Long => take_long(&mut long, require_under_500_for_next_long)
                .or_else(|| medium.pop())
                .or_else(|| short.pop())
                .or_else(|| long.pop()),
        };

        match candidate {
            Some(example) => {
                let words = word_count(&example);
                if words > 150 {
                    if require_under_500_for_next_long {
                        require_under_500_for_next_long = words >= 500;
                    } else if words > 1000 {
                        require_under_500_for_next_long = true;
                    }
                }
                selected.push(example);
            }
            None => break,
        }
    }

    selected
}

pub fn pick_random_examples(mut examples: Vec<String>, count: usize) -> Vec<String> {
    if examples.is_empty() || count == 0 {
        return Vec::new();
    }

    let mut rng = rand::thread_rng();
    examples.shuffle(&mut rng);
    examples.into_iter().take(count).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn write_temp_files(files: &[(&str, &str)]) -> std::path::PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        path.push(format!("style-reference-tests-{nanos}"));

        fs::create_dir_all(&path).unwrap();

        for (name, content) in files {
            let mut file = fs::File::create(path.join(name)).unwrap();
            file.write_all(content.as_bytes()).unwrap();
        }

        path
    }

    #[test]
    fn returns_dialogue_blocks_as_connected_units() {
        let content =
            "Gregson rubbed his hands in a self-satisfied way. \"I think we have\n\tdone all that can be done,\" he answered; \"it's a queer case though,\n\tand I knew your taste for such things.\"\n\n\"You did not come here in a cab?\" asked Sherlock Holmes.\n\n\"No, sir.\"\n\n\"Then let us go and look at the room.\"";

        let units = extract_units_from_text(content, ExtractMode::Dialogue);
        assert_eq!(units.len(), 1);
        assert!(units[0].contains("You did not come here in a cab?"));
        assert!(units[0].contains("Gregson"));
    }

    #[test]
    fn returns_only_narrative_paragraphs() {
        let content =
            "A short passage, bare planked and dusty, led to the kitchen and\n\toffices.\n\nThe door opened.\n\n\"No, sir.\"";

        let units = extract_units_from_text(content, ExtractMode::Narrative);
        assert_eq!(units.len(), 2);
        assert!(units[0].starts_with("A short passage"));
        assert!(units[1].starts_with("The door opened"));
    }

    #[test]
    fn narrative_excludes_dialogue_like_paragraphs() {
        let content =
            "\"You did not come here in a cab?\" asked Sherlock Holmes.\n\nThe hallway remained quiet and empty.\n\n\"You are close to the King's training stables,\" said she.";

        let units = extract_units_from_text(content, ExtractMode::Narrative);
        assert_eq!(
            units,
            vec!["The hallway remained quiet and empty.".to_string()]
        );
    }

    #[test]
    fn narrative_requires_punctuation() {
        let content = "A short passage with no punctuation\n\nThis one has punctuation.";

        let units = extract_units_from_text(content, ExtractMode::Narrative);
        assert_eq!(units, vec!["This one has punctuation.".to_string()]);
    }

    #[test]
    fn page_units_preserve_whole_paragraphs() {
        let first = format!("{}.", words(20));
        let second = format!("{}.", words(21));
        let third = format!("{}.", words(22));
        let content = format!("{first}\n\n{second}\n\n{third}");

        let units = extract_page_units_from_text(&content, 35);
        assert!(!units.is_empty());
        assert_eq!(units[0], format!("{first}\n\n{second}"));
    }

    #[test]
    fn chapter_start_grows_to_minimum_words() {
        let chapter_one_a = words(20);
        let chapter_one_b = words(35);
        let chapter_two = words(70);
        let content = format!(
            "CHAPTER ONE\n\n{chapter_one_a}\n\n{chapter_one_b}\n\nCHAPTER TWO\n\n{chapter_two}"
        );

        let units = extract_units_from_text(&content, ExtractMode::ChapterStart);
        assert_eq!(units.len(), 2);
        assert!(word_count(&units[0]) >= 50);
        assert!(units[0].contains(&chapter_one_a));
        assert!(units[0].contains(&chapter_one_b));
    }

    #[test]
    fn chapter_end_grows_to_minimum_words() {
        let chapter_one_a = words(20);
        let chapter_one_b = words(35);
        let chapter_two = words(55);
        let content = format!(
            "CHAPTER ONE\n\n{chapter_one_a}\n\n{chapter_one_b}\n\nCHAPTER TWO\n\n{chapter_two}"
        );

        let units = extract_units_from_text(&content, ExtractMode::ChapterEnd);
        assert_eq!(units.len(), 2);
        assert!(word_count(&units[0]) >= 50);
        assert!(units[0].contains(&chapter_one_a));
        assert!(units[0].contains(&chapter_one_b));
    }

    #[test]
    fn pick_examples_limits_to_available_count() {
        let examples = vec!["A".to_string(), "B".to_string()];
        let sampled = pick_examples(examples, 5);
        assert_eq!(sampled.len(), 2);
    }

    fn words(count: usize) -> String {
        (0..count)
            .map(|idx| format!("w{idx}"))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn tiers_for(excerpts: &[String]) -> Vec<LengthTier> {
        excerpts
            .iter()
            .map(|excerpt| length_tier(excerpt))
            .collect()
    }

    fn long_counts_for(excerpts: &[String]) -> Vec<usize> {
        excerpts
            .iter()
            .map(|excerpt| word_count(excerpt))
            .filter(|count| *count > 150)
            .collect()
    }

    #[test]
    fn picks_one_of_each_tier_for_three() {
        let examples = vec![words(12), words(110), words(220)];
        let sampled = pick_examples(examples, 3);
        let mut tiers = tiers_for(&sampled);
        tiers.sort_by_key(|tier| match tier {
            LengthTier::Short => 0,
            LengthTier::Medium => 1,
            LengthTier::Long => 2,
        });

        assert_eq!(sampled.len(), 3);
        assert_eq!(
            tiers,
            vec![LengthTier::Short, LengthTier::Medium, LengthTier::Long]
        );
    }

    #[test]
    fn each_full_batch_of_three_contains_one_of_each_tier() {
        let examples = vec![
            words(10),
            words(11),
            words(100),
            words(101),
            words(180),
            words(181),
        ];
        let sampled = pick_examples(examples, 6);
        let mut first = tiers_for(&sampled[0..3]);
        let mut second = tiers_for(&sampled[3..6]);
        first.sort_by_key(|tier| match tier {
            LengthTier::Short => 0,
            LengthTier::Medium => 1,
            LengthTier::Long => 2,
        });
        second.sort_by_key(|tier| match tier {
            LengthTier::Short => 0,
            LengthTier::Medium => 1,
            LengthTier::Long => 2,
        });

        assert_eq!(sampled.len(), 6);
        assert_eq!(
            first,
            vec![LengthTier::Short, LengthTier::Medium, LengthTier::Long]
        );
        assert_eq!(
            second,
            vec![LengthTier::Short, LengthTier::Medium, LengthTier::Long]
        );
    }

    #[test]
    fn under_three_returns_distinct_tiers() {
        let examples = vec![words(20), words(80), words(200)];
        let sampled = pick_examples(examples, 2);
        let tiers = tiers_for(&sampled);

        assert_eq!(sampled.len(), 2);
        assert_ne!(tiers[0], tiers[1]);
    }

    #[test]
    fn next_long_after_over_1000_is_under_500_when_available() {
        let examples = vec![
            words(10),
            words(11),
            words(12),
            words(80),
            words(90),
            words(100),
            words(1201),
            words(1300),
            words(300),
        ];

        for _ in 0..50 {
            let sampled = pick_examples(examples.clone(), 6);
            let longs = long_counts_for(&sampled);
            for window in longs.windows(2) {
                if window[0] > 1000 {
                    assert!(
                        window[1] < 500,
                        "expected next long under 500, got {}",
                        window[1]
                    );
                }
            }
        }
    }

    #[test]
    fn avoids_second_long_when_only_over_1000_longs_exist() {
        let examples = vec![
            words(10),
            words(11),
            words(12),
            words(80),
            words(90),
            words(100),
            words(1201),
            words(1300),
        ];

        let sampled = pick_examples(examples, 6);
        let longs = long_counts_for(&sampled);
        assert_eq!(longs.len(), 1);
        assert!(longs[0] > 1000);
    }

    #[test]
    fn count_zero_returns_empty() {
        let content = "\"Dialogue\"\n\nNarrative paragraph.";
        let units = extract_units_from_text(content, ExtractMode::Narrative);
        assert_eq!(pick_examples(units, 0).len(), 0);
    }

    #[test]
    fn collects_examples_from_directory() {
        let fixture = "\"A\"\n\nA narrative paragraph.";
        let root = write_temp_files(&[("a.txt", fixture)]);
        let examples = collect_examples(&root, ExtractMode::Narrative).unwrap();
        assert_eq!(examples, vec!["A narrative paragraph.".to_string()]);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn tolerates_non_utf8_files() {
        let root = std::env::temp_dir().join("style-reference-tests-non-utf8");
        fs::create_dir_all(&root).unwrap();
        let mut file = fs::File::create(root.join("a.txt")).unwrap();
        file.write_all(&[0x41, 0x20, 0xff, 0x42, 0x2e]).unwrap();

        let examples = collect_examples(&root, ExtractMode::Narrative).unwrap();

        assert_eq!(examples.len(), 1);
        assert_eq!(examples[0], "A �B.".to_string());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn ignores_non_text_files() {
        let root = write_temp_files(&[("a.bin", "\"Not text\""), ("b.txt", "A narrative.")]);
        let examples = collect_examples(&root, ExtractMode::Narrative).unwrap();
        assert_eq!(examples, vec!["A narrative.".to_string()]);
        let _ = fs::remove_dir_all(root);
    }
}
