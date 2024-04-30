use pulldown_cmark::{Event, LinkType};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{link::Link, LinkTransformer};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Substitution {
    #[serde(with = "serde_regex")]
    tail: Regex,
    tag: String,
    replacement: String,

    #[serde(default = "one")]
    limit: usize,

    #[serde(default)]
    code: bool,

    #[serde(default)]
    tail_only: bool,

    #[serde(default)]
    replace_text: bool,
}

/// Workaround for <https://github.com/serde-rs/serde/issues/368>
const fn one() -> usize {
    1
}

impl Substitution {
    #[must_use]
    pub fn example() -> Self {
        Self {
            tail: regex::Regex::new(r"\d+").expect("Invalid example regex"),
            tag: String::from("PS-"),
            replacement: "mycompany.jira.com/issues/PS-$text".to_string(),
            limit: 1,
            code: false,
            tail_only: false,
            replace_text: false,
        }
    }
}

/// A [`Substitution`] is a link transformer of sorts, too.
impl LinkTransformer for Substitution {
    fn tag(&self) -> String {
        self.tag.clone()
    }

    fn pattern(&self) -> Regex {
        Regex::new(format!("(?<text>{}{})", self.tag(), self.tail).as_str()).expect("Invalid regex")
    }

    fn strip_tag(&self) -> bool {
        self.tail_only
    }

    /// Perform the replacement.
    fn apply(&self, link: &mut Link) -> anyhow::Result<()> {
        if link.link_type == LinkType::ShortcutUnknown {
            return Ok(());
        }
        let snippet = &self
            .pattern()
            .replacen(&link.destination, self.limit, &self.replacement);

        let new_text = link.destination.clone();
        let text = if let Some(caps) = self.tail.captures(&link.destination) {
            caps.name(self.tail.as_str())
                .map(|m: regex::Match<'_>| m.as_str())
                .unwrap_or(&new_text)
                .to_string()
        } else {
            snippet.to_string()
        };

        let text = if self.strip_tag() && text.starts_with(&self.tag()) {
            text.replace(&self.tag(), "")
        } else {
            text
        };
        let event = if self.code {
            Event::Code(text.into())
        } else {
            Event::Text(text.into())
        };

        link.destination = snippet.to_string().into();
        if link.text.is_empty() || self.replace_text {
            link.text = vec![event];
        }
        if link.title.is_empty() {
            link.title = link.destination.clone();
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn grtp_replacement() {
        // let input = "[](GTPR-12355)";
        let sub = Substitution {
            tail: Regex::new(r"\d+").unwrap(),
            tag: "GTPR-".into(),
            replacement: "http://www.grtp.de/issue/$text".to_string(),
            limit: 0,
            code: true,
            tail_only: false,
            replace_text: false,
        };
        let link = &mut Link {
            link_type: LinkType::Reference,
            destination: "GTPR-12355".into(),
            title: "".into(),
            text: vec![],
            id: "".into(),
        };
        sub.apply(link).unwrap();
        dbg!(link);
    }

    #[test]
    fn struct_keyword_replacement() {
        let sub = Substitution {
            tail: Regex::new(r"(?<word>\w+)").unwrap(),
            tag: "keyword".into(),
            replacement: "https://doc.rust-lang.org/std/keyword.$word.html".to_string(),
            limit: 1,
            code: true,
            tail_only: true,
            replace_text: false,
        };
        let link = &mut Link {
            link_type: LinkType::Autolink,
            destination: "keyword:struct".into(),
            title: "".into(),
            text: vec![],
            id: "".into(),
        };
        sub.apply(link).unwrap();
        dbg!(link);
    }
}
