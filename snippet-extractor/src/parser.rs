use std::{collections::BTreeMap, path::Path};

use itertools::Itertools;
use snippet_extractor::Snippet;

use {once_cell::sync::Lazy, regex::Regex};

static MARKER_START: Lazy<Regex> = Lazy::new(|| Regex::new(r"[//|#] marker-start (\w+)").unwrap());

static MARKER_END: Lazy<Regex> = Lazy::new(|| Regex::new(r"[//|#] marker-end (\w+)").unwrap());

pub fn parse(text: &str, file: &Path) -> BTreeMap<String, Snippet> {
    let mut snippets = BTreeMap::default();
    let mut starts: BTreeMap<&str, Vec<(usize, usize)>> = BTreeMap::new();
    let mut ends: BTreeMap<&str, Vec<(usize, usize)>> = BTreeMap::new();
    for (line_number, line) in text.lines().enumerate() {
        for capture in MARKER_START.captures_iter(line) {
            let id = capture.get(1).expect("Start regex must have group 1");
            let col = id.start();
            starts
                .entry(id.as_str())
                .or_default()
                .push((line_number, col));
        }
        for capture in MARKER_END.captures_iter(line) {
            let id = capture.get(1).expect("End regex must have group 1");
            let col = id.start();
            ends.entry(id.as_str())
                .or_default()
                .push((line_number, col));
        }
    }
    for (id, positions) in starts {
        if positions.len() > 1 {
            eprintln!("{file:?}: Warning: identifier '{id}' used for start marker on multiple positions: {positions:?}.");
        }
        let ends = ends.remove(id).unwrap_or_default();
        if ends.len() != positions.len() {
            eprintln!(
                "{file:?}: Warning: start marker {id} occurs {} times, but there are {} end markers for it.",
                positions.len(),
                ends.len()
            );
        }
        positions.into_iter().zip(ends).for_each(|(start, end)| {
            if start.0 == end.0 {
                eprintln!("{file:?}: Ignoring multiple markers for {id} on same line");
            } else {
                if start >= end {
                    eprintln!("{file:?}: Warning: start >= end (start: {start:?}, end: {end:?}.");
                }
                let content = text
                    .lines()
                    .skip(start.0 + 1)
                    .take(end.0 - start.0 - 1)
                    .join("\n");
                let snippet = Snippet {
                    content,
                    file: file.to_path_buf(),
                    line: start.0,
                    col: 0,
                };
                snippets.insert(id.to_string(), snippet);
            }
        });
    }
    snippets
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn handles_nested_markers() {
        let input = include_str!("../tests/nested");
        let snippets = parse(input, PathBuf::from("test.txt").as_path());
        assert!(snippets
            .get("begin")
            .unwrap()
            .content
            .contains("// marker-start alsobegin"));
        assert!(snippets
            .get("begin")
            .unwrap()
            .content
            .contains("// marker-end alsobegin"));
        assert!(!snippets
            .get("begin")
            .unwrap()
            .content
            .contains("// marker-start begin"));
    }

    #[test]
    fn handles_overlapping_markers() {
        let input = include_str!("../tests/overlapping");
        let snippets = parse(input, PathBuf::from("test.txt").as_path());
        assert!(snippets
            .get("begin")
            .unwrap()
            .content
            .contains("# marker-start alsobegin"));
        assert!(snippets
            .get("alsobegin")
            .unwrap()
            .content
            .contains("# marker-end begin"));
    }
}
