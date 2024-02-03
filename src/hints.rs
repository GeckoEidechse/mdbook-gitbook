use super::Asset;
use once_cell::sync::Lazy;
use regex::Regex;

/// Uses regex to find [GitBook hints](https://docs.gitbook.com/content-editor/blocks/hint)
/// and replaces them with appropriate HTML rendering
pub fn render_hints(content: &str) -> Result<String, mdbook::errors::Error> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r#"\{% hint style="(?P<kind>[^"]+)" %\}\s*\n(?P<body>(?:.*\n)*?)\s*\{% endhint %\}"#,
        )
        .expect("failed to parse regex")
    });
    let hints = Asset::get("templates/hints.html").expect("template not found");
    let hints = std::str::from_utf8(hints.data.as_ref())?;
    let content = RE.replace_all(content, |caps: &regex::Captures| {
        let kind = caps
            .name("kind")
            .expect("kind not found in regex")
            .as_str()
            .to_lowercase();
        let body = caps
            .name("body")
            .expect("body not found in regex")
            .as_str()
            .replace("\n>\n", "\n\n")
            .replace("\n> ", "\n");
        hints.replace("{kind}", &kind).replace("{body}", &body)
    });
    Ok(content.into())
}
