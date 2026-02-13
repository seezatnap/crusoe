use std::error::Error;
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use rand::seq::SliceRandom;

#[derive(Clone, Copy)]
pub enum ExtractMode {
    Narrative,
    Dialogue,
}

#[derive(Debug)]
pub enum ExtractError {
    Io(std::io::Error),
    NoMatchingFiles(PathBuf),
}

impl fmt::Display for ExtractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::NoMatchingFiles(path) => {
                write!(f, "No readable text files found in {}", path.display())
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
        Some(ext) => matches!(
            ext.to_ascii_lowercase().as_str(),
            "txt" | "md" | "markdown"
        ),
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

    let mut all_examples = Vec::new();
    for file in files {
        let contents = fs::read_to_string(file)?;
        all_examples.extend(extract_units_from_text(&contents, mode));
    }

    Ok(all_examples)
}

pub fn pick_examples(mut examples: Vec<String>, count: usize) -> Vec<String> {
    if examples.is_empty() || count == 0 {
        return Vec::new();
    }

    let count = count.min(examples.len());
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
        assert_eq!(units, vec!["The hallway remained quiet and empty.".to_string()]);
    }

    #[test]
    fn narrative_requires_punctuation() {
        let content = "A short passage with no punctuation\n\nThis one has punctuation.";

        let units = extract_units_from_text(content, ExtractMode::Narrative);
        assert_eq!(units, vec!["This one has punctuation.".to_string()]);
    }

    #[test]
    fn pick_examples_limits_to_available_count() {
        let examples = vec!["A".to_string(), "B".to_string()];
        let sampled = pick_examples(examples, 5);
        assert_eq!(sampled.len(), 2);
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
    fn ignores_non_text_files() {
        let root = write_temp_files(&[("a.bin", "\"Not text\""), ("b.txt", "A narrative.")]);
        let examples = collect_examples(&root, ExtractMode::Narrative).unwrap();
        assert_eq!(examples, vec!["A narrative.".to_string()]);
        let _ = fs::remove_dir_all(root);
    }
}
