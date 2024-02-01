use super::Asset;
use once_cell::sync::Lazy;
use regex::Regex;

/// Uses regex to find [GitBook YouTube embeds](https://docs.gitbook.com/content-editor/blocks/embed-a-url#videos)
/// and replaces them with appropriate HTML rendering
pub fn render_youtube_embeds(content: &str) -> Result<String, mdbook::errors::Error> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"\{%\s*embed\s*url="https://www\.youtube\.com/watch\?v=(?P<ytid>[a-zA-Z0-9_-]+)"\s*%\}"#)
            .expect("failed to parse regex")
    });
    let embeds_template = Asset::get("youtube-embed-template.html")
        .expect("youtube-embed-template.html not found in assets");
    let hints = std::str::from_utf8(embeds_template.data.as_ref())?;
    let content = RE.replace_all(content, |caps: &regex::Captures| {
        let yt_id = caps
            .name("ytid")
            .expect("YouTube-ID not found in regex")
            .as_str();
        hints.replace("{ytid}", &yt_id)
    });
    Ok(content.into())
}
