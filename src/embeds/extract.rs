use reqwest::blocking::Client;
use scraper::Selector;

pub struct WebsiteEmbedProperties {
    pub icon_url: String,
    pub title: String,
    pub site_name: String,
}

fn extract_fluid_icon(document: scraper::Html) -> Option<String> {
    let fluid_icon_selector = match Selector::parse("link[rel='fluid-icon']") {
        Ok(res) => res,
        Err(_) => return None,
    };

    let fluid_icon = document
        .select(&fluid_icon_selector)
        .next()
        .and_then(|e| e.value().attr("href"))?;
    Some(fluid_icon.to_string())
}

fn extract_favicon(document: scraper::Html) -> Option<String> {
    let favicon_selector = match Selector::parse("link[rel='icon']") {
        Ok(res) => res,
        Err(_) => return None,
    };
    let favicon = document
        .select(&favicon_selector)
        .next()
        .and_then(|e| e.value().attr("href"))?;
    Some(favicon.to_string())
}

fn extract_og_title(document: scraper::Html) -> Option<String> {
    let og_title_selector = match Selector::parse("meta[property='og:title']") {
        Ok(res) => res,
        Err(_) => return None,
    };

    let og_title = document
        .select(&og_title_selector)
        .next()
        .and_then(|e| e.value().attr("content"))?;

    Some(og_title.to_string())
}

fn extract_title(document: scraper::Html) -> Option<String> {
    let title_selector = match Selector::parse("title") {
        Ok(res) => res,
        Err(_) => return None,
    };

    let title = document
        .select(&title_selector)
        .next()
        .map(|e| e.text().collect::<Vec<_>>().join(""))?;

    Some(title)
}

fn extract_og_name(document: scraper::Html) -> Option<String> {
    let og_site_name_selector = match Selector::parse("meta[property='og:site_name']") {
        Ok(res) => res,
        Err(_) => return None,
    };

    let og_site_name = document
        .select(&og_site_name_selector)
        .next()
        .and_then(|e| e.value().attr("content"))?;

    Some(og_site_name.to_string())
}

/// Gets embed data from website
pub fn get_website_embed_data(url: &str) -> WebsiteEmbedProperties {
    // Svg of a globe as a generic internet icon
    let icon_url = "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxZW0iIGhlaWdodD0iMWVtIiB2aWV3Qm94PSIwIDAgMjQgMjQiPjxwYXRoIGZpbGw9ImN1cnJlbnRDb2xvciIgZD0iTTE2LjM2IDE0Yy4wOC0uNjYuMTQtMS4zMi4xNC0yYzAtLjY4LS4wNi0xLjM0LS4xNC0yaDMuMzhjLjE2LjY0LjI2IDEuMzEuMjYgMnMtLjEgMS4zNi0uMjYgMm0tNS4xNSA1LjU2Yy42LTEuMTEgMS4wNi0yLjMxIDEuMzgtMy41NmgyLjk1YTguMDMgOC4wMyAwIDAgMS00LjMzIDMuNTZNMTQuMzQgMTRIOS42NmMtLjEtLjY2LS4xNi0xLjMyLS4xNi0yYzAtLjY4LjA2LTEuMzUuMTYtMmg0LjY4Yy4wOS42NS4xNiAxLjMyLjE2IDJjMCAuNjgtLjA3IDEuMzQtLjE2IDJNMTIgMTkuOTZjLS44My0xLjItMS41LTIuNTMtMS45MS0zLjk2aDMuODJjLS40MSAxLjQzLTEuMDggMi43Ni0xLjkxIDMuOTZNOCA4SDUuMDhBNy45MjMgNy45MjMgMCAwIDEgOS40IDQuNDRDOC44IDUuNTUgOC4zNSA2Ljc1IDggOG0tMi45MiA4SDhjLjM1IDEuMjUuOCAyLjQ1IDEuNCAzLjU2QTguMDA4IDguMDA4IDAgMCAxIDUuMDggMTZtLS44Mi0yQzQuMSAxMy4zNiA0IDEyLjY5IDQgMTJzLjEtMS4zNi4yNi0yaDMuMzhjLS4wOC42Ni0uMTQgMS4zMi0uMTQgMmMwIC42OC4wNiAxLjM0LjE0IDJNMTIgNC4wM2MuODMgMS4yIDEuNSAyLjU0IDEuOTEgMy45N2gtMy44MmMuNDEtMS40MyAxLjA4LTIuNzcgMS45MS0zLjk3TTE4LjkyIDhoLTIuOTVhMTUuNjUgMTUuNjUgMCAwIDAtMS4zOC0zLjU2YzEuODQuNjMgMy4zNyAxLjkgNC4zMyAzLjU2TTEyIDJDNi40NyAyIDIgNi41IDIgMTJhMTAgMTAgMCAwIDAgMTAgMTBhMTAgMTAgMCAwIDAgMTAtMTBBMTAgMTAgMCAwIDAgMTIgMiIgLz48L3N2Zz4="
        .to_string();
    // Use URL as fallback
    let title = url.to_string();
    // Use basedomain as fallback
    let site_name = url::Url::parse(url)
        .unwrap() // TODO: don't panic but just fall back to url
        .host_str()
        .unwrap_or(url)
        .to_string();

    // Load webpage and parse it
    let response_body = match Client::new().get(url).send() {
        Ok(response) => response.text().unwrap(),
        Err(e) => {
            eprintln!("Error: {}", e);
            return WebsiteEmbedProperties {
                icon_url,
                title,
                site_name,
            };
        }
    };
    let document = scraper::Html::parse_document(&response_body);

    // Get icon
    // Try fluid icon first, if not, fallback to favicon
    let icon_url = extract_fluid_icon(document.clone())
        .or_else(|| extract_favicon(document.clone()))
        .unwrap_or(icon_url);

    // Get title
    // Try og:title first, fall back to <title>, or URL otherwise
    let title = extract_og_title(document.clone())
        .or_else(|| extract_title(document.clone()))
        .unwrap_or(title);

    // Get sitename
    // Try og:site_name first, otherwise use base domain
    let site_name = extract_og_name(document.clone()).unwrap_or(site_name);

    WebsiteEmbedProperties {
        icon_url,
        title,
        site_name,
    }
}
