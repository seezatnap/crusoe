use analysis_contracts::{
    AnalysisBinary, AnalysisError, AnalysisInput, AnalysisReport, Finding, Severity, SourceLocation,
};
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Write};
use std::fs;
use std::path::{Path, PathBuf};

const MARKER_PREFIX: &str = "<!-- continuity:scene";
const DEFAULT_LORE_STATES: &[&str] = &["unknown", "hint", "evidence", "confirmed", "public"];

#[derive(Clone, Debug)]
struct Scene {
    id: String,
    path: PathBuf,
    line: u32,
    timeline: Option<i32>,
    drift: Option<i32>,
    reveals: Vec<String>,
    requires: Vec<String>,
    lore: Vec<LoreMention>,
}

#[derive(Clone, Debug)]
struct LoreMention {
    id: String,
    state: String,
}

#[derive(Clone, Debug)]
struct SceneRef {
    line: u32,
    path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ContinuityOptions {
    pub max_timeline_backstep: i32,
    pub max_drift_jump: i32,
    pub max_drift_backstep: i32,
    pub reveal_order: Vec<String>,
    pub lore_states: Vec<String>,
}

impl Default for ContinuityOptions {
    fn default() -> Self {
        Self {
            max_timeline_backstep: 0,
            max_drift_jump: 2,
            max_drift_backstep: 0,
            reveal_order: Vec::new(),
            lore_states: DEFAULT_LORE_STATES.iter().map(|item| item.to_string()).collect(),
        }
    }
}

#[derive(Debug)]
struct ReportState {
    findings: Vec<Finding>,
}

impl ReportState {
    fn push(
        &mut self,
        code: &str,
        message: impl Into<String>,
        severity: Severity,
        location: SourceLocation,
        expected: impl Into<String>,
        observed: impl Into<String>,
    ) {
        let expected = expected.into();
        let observed = observed.into();
        self.findings.push(
            Finding::new(code, message, severity)
                .with_location(location.clone())
                .with_suggestion(make_diff_block(
                    &location.path.display().to_string(),
                    location.line.unwrap_or(0),
                    &expected,
                    &observed,
                )),
        );
    }
}

fn make_diff_block(path: &str, line: u32, expected: &str, observed: &str) -> String {
    format!(
        "--- expected\n+++ observed\n@@ {path}:{line}\n- {expected}\n+ {observed}\n"
    )
}

fn split_csv(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_lowercase())
        .collect()
}

fn parse_lore(value: &str) -> Vec<LoreMention> {
    value
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| {
            if let Some((id, state)) = value.split_once(':') {
                LoreMention {
                    id: id.to_lowercase(),
                    state: state.to_lowercase(),
                }
            } else {
                LoreMention {
                    id: value.to_lowercase(),
                    state: "evidence".to_string(),
                }
            }
        })
        .collect()
}

fn parse_scene_line(path: &Path, line_no: u32, line: &str) -> Option<Scene> {
    let trimmed = line.trim();
    if !trimmed.starts_with(MARKER_PREFIX) || !trimmed.ends_with("-->") {
        return None;
    }

    let mut inner = trimmed.trim_start_matches("<!--").trim();
    inner = inner.trim_start_matches("continuity:scene").trim();
    inner = inner.trim_end_matches("-->").trim();
    if inner.is_empty() {
        return None;
    }

    let mut id = String::new();
    let mut timeline = None;
    let mut drift = None;
    let mut reveals = Vec::new();
    let mut requires = Vec::new();
    let mut lore = Vec::new();

    for token in inner.split_whitespace() {
        let Some((key, value)) = token.split_once('=') else {
            continue;
        };
        match key.to_lowercase().as_str() {
            "id" => id = value.to_lowercase(),
            "timeline" => {
                if let Ok(parsed) = value.parse::<i32>() {
                    timeline = Some(parsed);
                }
            }
            "drift" => {
                if let Ok(parsed) = value.parse::<i32>() {
                    drift = Some(parsed);
                }
            }
            "reveals" => reveals = split_csv(value),
            "requires" => requires = split_csv(value),
            "lore" => lore = parse_lore(value),
            _ => {}
        }
    }

    if id.is_empty() {
        id = format!("line-{line_no}");
    }

    Some(Scene {
        id,
        path: path.to_path_buf(),
        line: line_no,
        timeline,
        drift,
        reveals,
        requires,
        lore,
    })
}

fn chapter_number_from_path(path: &Path) -> Option<u32> {
    let stem = path.file_stem()?.to_str()?;
    let stem = stem.to_lowercase();
    let suffix = stem.strip_prefix("chapter-")?;
    suffix.parse().ok()
}

fn collect_scenes(path: &Path) -> Result<Vec<Scene>, AnalysisError> {
    let content = fs::read_to_string(path).map_err(|err| AnalysisError::Io {
        path: path.display().to_string(),
        reason: err.to_string(),
    })?;
    let mut scenes = Vec::new();
    for (index, line) in content.lines().enumerate() {
        let line_no = u32::try_from(index + 1).unwrap_or(0);
        if let Some(scene) = parse_scene_line(path, line_no, line) {
            scenes.push(scene);
        }
    }
    Ok(scenes)
}

fn discover_files(path: &Path, out: &mut Vec<PathBuf>) -> Result<(), AnalysisError> {
    if path.is_file() {
        if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            out.push(path.to_path_buf());
        }
        return Ok(());
    }

    let mut entries = Vec::new();
    for entry in fs::read_dir(path).map_err(|err| AnalysisError::Io {
        path: path.display().to_string(),
        reason: err.to_string(),
    })? {
        entries.push(entry.map_err(|err| AnalysisError::Io {
            path: path.display().to_string(),
            reason: err.to_string(),
        })?.path());
    }

    entries.sort_unstable();
    for entry in entries {
        if entry.is_dir() {
            discover_files(&entry, out)?;
        } else if entry.extension().and_then(|ext| ext.to_str()) == Some("md") {
            out.push(entry);
        }
    }

    Ok(())
}

fn collect_scene_sequence(target: &Path) -> Result<Vec<Scene>, AnalysisError> {
    let mut files = Vec::new();
    discover_files(target, &mut files)?;
    if files.is_empty() {
        return Ok(Vec::new());
    }

    files.sort_unstable_by(|left, right| {
        let left_chapter = chapter_number_from_path(left).unwrap_or(u32::MAX);
        let right_chapter = chapter_number_from_path(right).unwrap_or(u32::MAX);
        match left_chapter.cmp(&right_chapter) {
            std::cmp::Ordering::Equal => left.cmp(right),
            other => other,
        }
    });

    let mut scenes = Vec::new();
    for file in files {
        scenes.extend(collect_scenes(&file)?);
    }

    Ok(scenes)
}

fn check_timeline(scenes: &[Scene], options: &ContinuityOptions, state: &mut ReportState) {
    for window in scenes.windows(2) {
        let prev = &window[0];
        let curr = &window[1];
        let (Some(prev_timeline), Some(curr_timeline)) = (prev.timeline, curr.timeline) else {
            continue;
        };

        if curr_timeline + options.max_timeline_backstep < prev_timeline {
            state.push(
                "CONT-TL-001",
                format!(
                    "Timeline regression: scene '{}' moved from {prev_timeline} to {curr_timeline}.",
                    curr.id
                ),
                Severity::Error,
                SourceLocation::new(&curr.path, Some(curr.line), None),
                format!("timeline >= {}", prev_timeline - options.max_timeline_backstep),
                format!("timeline = {curr_timeline}"),
            );
        }
    }
}

fn check_drift(scenes: &[Scene], options: &ContinuityOptions, state: &mut ReportState) {
    for window in scenes.windows(2) {
        let prev = &window[0];
        let curr = &window[1];
        let (Some(prev_drift), Some(curr_drift)) = (prev.drift, curr.drift) else {
            continue;
        };

        if curr_drift + options.max_drift_backstep < prev_drift {
            state.push(
                "CONT-DRIFT-001",
                format!(
                    "Drift level regressed in scene '{}' from {prev_drift} to {curr_drift}.",
                    curr.id
                ),
                Severity::Error,
                SourceLocation::new(&curr.path, Some(curr.line), None),
                format!("drift >= {}", prev_drift - options.max_drift_backstep),
                format!("drift = {curr_drift}"),
            );
        }

        let jump = (curr_drift - prev_drift).abs();
        if jump > options.max_drift_jump {
            state.push(
                "CONT-DRIFT-002",
                format!(
                    "Drift jump too large in scene '{}' from {prev_drift} to {curr_drift} ({jump}).",
                    curr.id
                ),
                Severity::Warning,
                SourceLocation::new(&curr.path, Some(curr.line), None),
                format!("|drift - {prev_drift}| <= {}", options.max_drift_jump),
                format!("|drift - {prev_drift}| = {jump}"),
            );
        }
    }
}

fn check_lore(scenes: &[Scene], options: &ContinuityOptions, state: &mut ReportState) {
    let mut state_rank = HashMap::new();
    for (index, value) in options.lore_states.iter().enumerate() {
        state_rank.insert(value.to_lowercase(), index as i32);
    }

    let mut last_seen: HashMap<String, (i32, String)> = HashMap::new();

    for scene in scenes {
        for mention in &scene.lore {
            let current_rank = match state_rank.get(&mention.state) {
                Some(rank) => *rank,
                None => {
                    state.push(
                        "CONT-LORE-000",
                        format!(
                            "Lore '{}' in scene '{}' uses unknown state '{}'.",
                            mention.id, scene.id, mention.state
                        ),
                        Severity::Warning,
                        SourceLocation::new(&scene.path, Some(scene.line), None),
                        "known lore state",
                        format!("state '{}'", mention.state),
                    );
                    0
                }
            };

            if let Some((previous_rank, previous_id)) = last_seen.get(&mention.id) {
                let previous_rank = *previous_rank;
                let previous_id = previous_id.clone();
                if current_rank < previous_rank {
                    let expected = format!(
                        "{} has already been seen at state '{}'",
                        mention.id,
                        options
                            .lore_states
                            .get(previous_rank as usize)
                            .map(|item| item.as_str())
                            .unwrap_or("unknown")
                    );
                    state.push(
                        "CONT-LORE-001",
                        format!(
                            "Lore state regressed for '{}' from {} to {}.",
                            mention.id,
                            previous_id,
                            mention.state
                        ),
                        Severity::Error,
                        SourceLocation::new(&scene.path, Some(scene.line), None),
                        expected,
                        format!("{} in '{}'", mention.state, scene.id),
                    );
                }
            }
            last_seen.insert(mention.id.clone(), (current_rank, scene.id.clone()));
        }
    }
}

fn check_scene_dependencies(scenes: &[Scene], state: &mut ReportState) {
    let mut seen: HashMap<String, SceneRef> = HashMap::new();
    let mut ids = HashSet::new();

    for (index, scene) in scenes.iter().enumerate() {
        let current = scene.id.to_lowercase();
        if !ids.insert(current.clone()) {
                state.push(
                    "CONT-SCENE-002",
                    format!("Scene '{current}' is duplicated."),
                    Severity::Warning,
                    SourceLocation::new(&scene.path, Some(scene.line), None),
                    "unique scene ids",
                    format!("duplicate scene id '{current}'"),
                );
            }

        for dependency in &scene.requires {
            let dependency = dependency.to_lowercase();
            if !seen.contains_key(&dependency) {
                state.push(
                    "CONT-SCENE-001",
                    format!(
                        "Scene '{}' depends on missing scene '{}'.",
                        scene.id, dependency
                    ),
                    Severity::Error,
                    SourceLocation::new(&scene.path, Some(scene.line), None),
                    format!("dependency '{dependency}' is declared before usage"),
                    format!("first occurrence of '{dependency}' not found earlier"),
                );
            }
        }

        seen.insert(
            current,
            SceneRef {
                line: scene.line,
                path: scene.path.clone(),
            },
        );
        let _ = index;
    }
}

fn check_reveal_order(scenes: &[Scene], options: &ContinuityOptions, state: &mut ReportState) {
    if options.reveal_order.is_empty() {
        return;
    }

    let mut first_seen: HashMap<String, (usize, SceneRef)> = HashMap::new();
    for (index, scene) in scenes.iter().enumerate() {
        for reveal in &scene.reveals {
            first_seen.entry(reveal.to_lowercase()).or_insert_with(|| {
                (
                    index,
                    SceneRef {
                        line: scene.line,
                        path: scene.path.clone(),
                    },
                )
            });
        }
    }

    for order in options.reveal_order.windows(2) {
        let first = order[0].to_lowercase();
        let second = order[1].to_lowercase();
        let Some((first_pos, first_scene)) = first_seen.get(&first).map(|(index, scene)| (*index, scene)) else {
            continue;
        };
        let Some((second_pos, second_scene)) = first_seen.get(&second).map(|(index, scene)| (*index, scene)) else {
            continue;
        };

        if first_pos > second_pos {
            state.push(
                "CONT-REVEAL-001",
                format!("Reveal order violation: '{first}' appears after '{second}'."),
                Severity::Error,
                SourceLocation::new(&second_scene.path, Some(second_scene.line), None),
                format!("'{first}' before '{second}'"),
                format!(
                    "'{first}' at {}:{} is after '{second}' at {}:{}",
                    first_scene.path.display(),
                    first_scene.line,
                    second_scene.path.display(),
                    second_scene.line
                ),
            );
        }
    }
}

fn populate_report_metadata(report: &mut AnalysisReport, scenes: &[Scene]) {
    report
        .metadata
        .insert("scene_count".to_string(), scenes.len().to_string());
    if let Some(first) = scenes.first() {
        report.metadata.insert(
            "first_scene_file".to_string(),
            first.path.display().to_string(),
        );
        if let Some(last) = scenes.last() {
            report
                .metadata
                .insert("last_scene_file".to_string(), last.path.display().to_string());
        }
    } else {
        report
            .metadata
            .insert("status".to_string(), "no_scenes_found".to_string());
    }
}

pub fn run_continuity_check(
    input: &AnalysisInput,
    options: ContinuityOptions,
) -> Result<AnalysisReport, AnalysisError> {
    input.validate_target_exists()?;
    let target = input.resolve_target_path();
    let scenes = collect_scene_sequence(&target)?;

    let mut report = AnalysisReport::new("continuity-check", &target)
        .add_metadata("analyzer", "continuity-check")
        .add_metadata("target", target.display().to_string());

    let mut state = ReportState { findings: Vec::new() };
    check_timeline(&scenes, &options, &mut state);
    check_drift(&scenes, &options, &mut state);
    check_lore(&scenes, &options, &mut state);
    check_scene_dependencies(&scenes, &mut state);
    check_reveal_order(&scenes, &options, &mut state);

    populate_report_metadata(&mut report, &scenes);
    for (name, value) in &input.options {
        report = report.add_metadata(format!("option:{name}"), value.clone());
    }

    for finding in state.findings {
        report.findings.push(finding);
    }

    Ok(report)
}

fn render_text_line(location: &SourceLocation, width: usize, output: &mut String) -> fmt::Result {
    write!(
        output,
        "{:>w$}:{}",
        location.path.display(),
        location.line.unwrap_or(0),
        w = width
    )
}

pub fn render_report_text_diff(report: &AnalysisReport) -> String {
    let mut out = String::new();
    if report.findings.is_empty() {
        out.push_str("No continuity violations.\n");
        return out;
    }

    let mut findings = report.findings.clone();
    findings.sort_by(|left, right| left.code.cmp(&right.code));

    let mut code_count: HashMap<String, usize> = HashMap::new();
    for finding in findings {
        let count = code_count.entry(finding.code.clone()).or_insert(0);
        *count += 1;
        let _ = writeln!(out, "---");
        let _ = writeln!(out, "violation {}: {}", finding.code, count);
        let _ = writeln!(out, "{}", finding.message);
        if let Some(location) = finding.location.as_ref() {
            let mut line = String::new();
            let _ = render_text_line(location, 20, &mut line);
            let _ = writeln!(out, "@@ {} @@ ", line.trim());
        }
        if let Some(diff) = finding.suggestion.as_ref() {
            let _ = writeln!(out, "{}", diff);
        }
        let _ = writeln!(out);
    }

    out
}

pub struct ContinuityAnalyzer;

impl AnalysisBinary for ContinuityAnalyzer {
    fn analyzer_name(&self) -> &'static str {
        "continuity-check"
    }

    fn run(&self, input: &AnalysisInput) -> Result<AnalysisReport, AnalysisError> {
        run_continuity_check(input, ContinuityOptions::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TempPath {
        path: PathBuf,
    }

    impl TempPath {
        fn new() -> Self {
            let mut base = env::temp_dir();
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_or(0, |d| d.as_nanos());
            base.push(format!("continuity-check-tests-{now}"));
            fs::create_dir_all(&base).expect("create temporary dir");
            Self { path: base }
        }

        fn write(&self, name: &str, content: &str) -> PathBuf {
            let path = self.path.join(name);
            fs::write(&path, content).expect("write temporary chapter");
            path
        }
    }

    impl Drop for TempPath {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn default_options() -> ContinuityOptions {
        let mut options = ContinuityOptions::default();
        options.reveal_order = Vec::new();
        options
    }

    fn run_on_file(name: &str, content: &str, options: ContinuityOptions) -> AnalysisReport {
        let temp = TempPath::new();
        let file = temp.write(name, content);
        run_continuity_check(&AnalysisInput::new(file), options).expect("run continuity check")
    }

    #[test]
    fn parses_scene_markers_with_fields() {
        let content = r#"
<!-- continuity:scene id=intro timeline=2030 drift=1 reveals=atlas,lattice lore=lattice:hint -->
some prose
<!-- continuity:scene id=jump timeline=2040 drift=2 reveals=rosetta lore=rosetta:evidence -->
"#;
        let report = run_on_file("chapter-01.md", content, default_options());
        assert_eq!(report.finding_count(), 0);
    }

    #[test]
    fn detects_timeline_regression() {
        let content = r#"
<!-- continuity:scene id=a timeline=2050 drift=1 -->
noise
<!-- continuity:scene id=b timeline=2040 drift=1 requires=a -->
"#;
        let report = run_on_file("chapter-01.md", content, default_options());
        assert!(report.findings.iter().any(|finding| finding.code == "CONT-TL-001"));
    }

    #[test]
    fn detects_drift_jump() {
        let content = r#"
<!-- continuity:scene id=a timeline=2050 drift=1 -->
noise
<!-- continuity:scene id=b timeline=2051 drift=10 -->
"#;
        let mut options = default_options();
        options.max_drift_jump = 2;
        let report = run_on_file("chapter-01.md", content, options);
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.code == "CONT-DRIFT-002"));
    }

    #[test]
    fn detects_scene_dependency_violation() {
        let content = r#"
<!-- continuity:scene id=jump timeline=2050 drift=1 requires=unknown -->
"#;
        let report = run_on_file("chapter-01.md", content, default_options());
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.code == "CONT-SCENE-001"));
    }

    #[test]
    fn detects_lore_regression() {
        let content = r#"
<!-- continuity:scene id=alpha timeline=2050 drift=1 lore=rosetta:confirmed -->
noise
<!-- continuity:scene id=beta timeline=2051 drift=1 lore=rosetta:hint -->
"#;
        let report = run_on_file("chapter-01.md", content, default_options());
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.code == "CONT-LORE-001"));
    }

    #[test]
    fn detects_reveal_order_violation() {
        let content = r#"
<!-- continuity:scene id=scene-b timeline=2050 drift=1 reveals=beta -->
noise
<!-- continuity:scene id=scene-a timeline=2051 drift=1 reveals=alpha -->
"#;
        let mut options = default_options();
        options.reveal_order = vec!["alpha".into(), "beta".into()];
        let report = run_on_file("chapter-01.md", content, options);
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.code == "CONT-REVEAL-001"));
    }

    #[test]
    fn resolves_relative_target_with_working_directory() {
        let mut workdir = env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        workdir.push(format!("continuity-check-tests-{nanos}"));
        std::fs::create_dir_all(&workdir).unwrap();

        let expected = workdir.join("chapter-01.md");
        std::fs::write(
            &expected,
            "<!-- continuity:scene id=scene timeline=2050 drift=1 -->\nText.\n",
        )
        .unwrap();

        let input = AnalysisInput::new("chapter-01.md").with_working_directory(&workdir);
        let report = run_continuity_check(&input, default_options()).unwrap();

        assert_eq!(report.target, expected);
        assert_eq!(
            report
                .metadata
                .get("target")
                .expect("target metadata should be present"),
            &workdir.join("chapter-01.md").display().to_string()
        );

        let _ = std::fs::remove_file(&expected);
        let _ = std::fs::remove_dir(&workdir);
    }
}
