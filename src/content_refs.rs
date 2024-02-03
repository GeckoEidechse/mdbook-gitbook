use super::Asset;
use once_cell::sync::Lazy;
use regex::Regex;

/// Uses regex to find [GitBook page links](https://docs.gitbook.com/content-editor/blocks/page-link#representation-in-markdown)
/// and replaces them with appropriate HTML rendering
pub fn render(content: &str) -> Result<String, mdbook::errors::Error> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r#"\{% content-ref url="(?P<refurl>[^"]+)" %\}\s\[(?P<text>[^]]+)\]\((?P<texturl>[^)]+)\)\s\{% endcontent-ref %\}"#,
        )
        .expect("failed to parse regex")
    });
    let content_refs =
        Asset::get("templates/content-refs.html").expect("template not found in assets");
    let content_refs = std::str::from_utf8(content_refs.data.as_ref())?;
    let content = RE.replace_all(content, |caps: &regex::Captures| {
        let ref_url = caps
            .name("refurl")
            .expect("kind not found in regex")
            .as_str()
            .to_lowercase();
        let text = caps.name("text").expect("body not found in regex").as_str();
        let text_url = caps
            .name("texturl")
            .expect("body not found in regex")
            .as_str();

        assert_eq!(ref_url, text_url); // TODO warn instead of assert

        content_refs
            .replace("{ref_url}", &ref_url)
            .replace("{text}", text)
            .replace("{text_url}", text_url)
    });
    Ok(content.into())
}
