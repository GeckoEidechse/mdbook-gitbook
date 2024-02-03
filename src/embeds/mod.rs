use super::Asset;
use once_cell::sync::Lazy;
use regex::Regex;

mod extract;

/// Renders various embeds formatted as
/// `{% embed url="URL_HERE" %}`
/// with special case handling for:
/// - YouTube links
pub fn render(content: &str) -> Result<String, mdbook::errors::Error> {
    // Handle special cases first:
    let content = render_youtube_embeds(content)?;

    // Then render generic links
    let content = render_generic_embeds(&content)?;

    // Finally return
    Ok(content)
}

/// Uses regex to find [website embeds](https://docs.gitbook.com/content-editor/blocks/embed-a-url#git-sync-representation-in-markdown)
/// and replaces them with appropriate HTML rendering
fn render_generic_embeds(content: &str) -> Result<String, mdbook::errors::Error> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"\{%\s*embed\s*url="(?P<url>https?://(?:[a-zA-Z0-9-]+\.)+[a-zA-Z]{2,}(?:/[a-zA-Z0-9-._%&?=/]*)*)"\s*%\}"#)
            .expect("failed to parse regex")
    });
    let embeds_template =
        Asset::get("templates/embeds/generic.html").expect("template not found in assets");
    let hints = std::str::from_utf8(embeds_template.data.as_ref())?;
    let content = RE.replace_all(content, |caps: &regex::Captures| {
        let url = caps.name("url").expect("url not found").as_str();

        let embed_data = extract::get_website_embed_data(url);

        hints
            .replace("{url}", url)
            .replace("{icon_link}", &embed_data.icon_url)
            .replace("{og:title}", &embed_data.title)
            .replace("{og:site_name}", &embed_data.site_name)
    });
    Ok(content.into())
}

/// Uses regex to find [GitBook YouTube embeds](https://docs.gitbook.com/content-editor/blocks/embed-a-url#videos)
/// and replaces them with appropriate HTML rendering
fn render_youtube_embeds(content: &str) -> Result<String, mdbook::errors::Error> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"\{%\s*embed\s*url="https://www\.youtube\.com/watch\?v=(?P<ytid>[a-zA-Z0-9_-]+)"\s*%\}"#)
            .expect("failed to parse regex")
    });
    let embeds_template =
        Asset::get("templates/embeds/youtube.html").expect("template not found in assets");
    let hints = std::str::from_utf8(embeds_template.data.as_ref())?;
    let content = RE.replace_all(content, |caps: &regex::Captures| {
        let yt_id = caps
            .name("ytid")
            .expect("YouTube-ID not found in regex")
            .as_str();
        hints.replace("{ytid}", yt_id)
    });
    Ok(content.into())
}
