use regex::Regex;
use scraper::{Html, Selector};

const CHEMIST_WAREHOUSE_URL_HOMEPAGE: &str = "https://www.chemistwarehouse.com.au";

#[tokio::main]
pub async fn handle_catalogs() {
    let homepage_request = get_page(CHEMIST_WAREHOUSE_URL_HOMEPAGE);
    let html: String = match homepage_request.await {
        Ok(v) => v,
        _ => {
            println!("{}", "Error retrieving homepage.");
            return;
        }
    };

    let top_level_category_urls = parse_homepage(html);
}

async fn get_page(url: &str) -> Result<String, reqwest::Error> {
    let result = reqwest::get(url).await?.text().await?;
    return Ok(result);
}

fn parse_homepage(html: String) -> Vec<String> {
    let category_url_matcher = Regex::new(r"shop-online").unwrap();
    let document = Html::parse_document(&html);

    let selector = Selector::parse("ul.menu-items > li > a").unwrap();

    let categories_url = document
        .select(&selector)
        .map(|e| e.value().attr("href").unwrap().replace("\"", ""))
        .filter(|href| category_url_matcher.is_match(&href))
        .collect::<Vec<String>>();

    return categories_url;
}
