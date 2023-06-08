use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "snippets.pest"]
pub struct SnippetParser;

#[cfg(test)]
mod test {
    use pest::Parser;

    use super::*;

    #[test]
    fn parses_simple_snippets() {
        let text = r#"Stop it. You breathe in, you breathe out. Eat, shit, sleep. You take whatever they give you, and you give nothing in return.
// marker-start peaches
let a = 4;
let a = 5;
// marker-end peaches
"#;
        let pair = SnippetParser::parse(super::Rule::File, text)
            .unwrap()
            .into_iter()
            .next()
            .unwrap()
            .into_inner()
            .into_iter()
            .next()
            .unwrap();
        assert_eq!(pair.as_rule(), Rule::Snippet);
        let mut snippet = pair.into_inner().into_iter();
        let identifier = snippet.next().unwrap().as_str();
        let snippet_text = snippet.next().unwrap().as_str();
        assert_eq!(identifier, "peaches");
        assert_eq!(
            snippet_text,
            r#"let a = 4;
let a = 5;
"#
        );
    }
}
