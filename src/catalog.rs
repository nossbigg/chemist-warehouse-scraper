use regex::Regex;
use scraper::{Html, Selector};

const CHEMIST_WAREHOUSE_URL_HOMEPAGE: &str = "https://www.chemistwarehouse.com.au";

#[tokio::main]
pub async fn handle_catalogs() {
    let homepage_request = get_page(CHEMIST_WAREHOUSE_URL_HOMEPAGE);
    let html: String = match homepage_request.await {
        Ok(v) => v,
        _ => {
            println!("Error retrieving homepage.");
            return;
        }
    };

    let top_level_category_urls = parse_homepage(html);

    let mut category_ids: Vec<String> = Vec::new();
    let category_id_matcher = Regex::new(r"shop-online/(\d+)").unwrap();
    for category_url in top_level_category_urls {
        parse_category_page(category_url, &category_id_matcher, &mut category_ids).await
    }
}

async fn get_page(url: &str) -> Result<String, reqwest::Error> {
    let result = reqwest::get(url).await?.text().await?;
    return Ok(result);
}

fn parse_json(value: &str) -> json::JsonValue {
    json::parse(&value).unwrap()
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

async fn parse_category_page(
    category_url: String,
    category_id_matcher: &Regex,
    category_ids: &mut Vec<String>,
) {
    if !category_id_matcher.is_match(&category_url) {
        return;
    }
    let captures = category_id_matcher
        .captures_iter(&category_url)
        .nth(0)
        .unwrap();
    let category_id = &captures[1];
    category_ids.push(category_id.into());

    let api_category_url = make_search_api_url(category_id.into(), "0");
    let api_request = get_page(&api_category_url);
    let response = match api_request.await {
        Ok(v) => v,
        _ => {
            println!("Error retrieving category url. {}", category_url);
            return;
        }
    };

    let json = parse_json(&response);
    let filters = &json["universes"]["universe"][0]["facetmap"][0]["filter"][0]["filtersection"];

    let mut child_category_ids: Vec<String> = Vec::new();
    for filter in filters.members() {
        let child_category_id_value = &filter["value"]["value"];
        let child_category_id = get_category_id(child_category_id_value.as_str().unwrap());
        child_category_ids.push(child_category_id)
    }
}

fn get_category_id(value: &str) -> String {
    let category_id_matcher = Regex::new(r"chemau(\d+)$").unwrap();
    let captures = category_id_matcher.captures_iter(&value).nth(0).unwrap();
    let category_id = &captures[1];
    return category_id.into();
}

fn make_search_api_url(category_id: &str, index: &str) -> String {
    return format!("https://www.chemistwarehouse.com.au/searchapi/webapi/search/category?category={}&index={}&sort=", category_id, index);
}
