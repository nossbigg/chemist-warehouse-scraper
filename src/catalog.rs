use regex::Regex;
use scraper::{Html, Selector};
use std::fmt;

const CHEMIST_WAREHOUSE_URL_HOMEPAGE: &str = "https://www.chemistwarehouse.com.au";

#[derive(Clone)]
struct MyNode {
    from: String,
    to: String,
}

impl fmt::Debug for MyNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}

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
    let top_level_category_ids = get_top_level_category_ids(top_level_category_urls);

    let mut all_nodes: Vec<MyNode> = Vec::new();
    let mut current_nodes: Vec<MyNode> = top_level_category_ids;

    while current_nodes.len() > 0 {
        let mut next_nodes: Vec<MyNode> = Vec::new();

        for entry in current_nodes {
            let mut result = parse_category_page(&entry.to).await;
            next_nodes.append(&mut result);
        }

        current_nodes = next_nodes.to_owned();
        all_nodes.append(&mut next_nodes);
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

fn get_top_level_category_ids(category_urls: Vec<String>) -> Vec<MyNode> {
    let category_id_matcher = Regex::new(r"shop-online/(\d+)").unwrap();
    let mut category_ids: Vec<MyNode> = Vec::new();

    for category_url in category_urls {
        if !category_id_matcher.is_match(&category_url) {
            continue;
        }
        let captures = category_id_matcher
            .captures_iter(&category_url)
            .nth(0)
            .unwrap();
        let category_id = &captures[1];

        let entry = MyNode {
            from: "".to_string(),
            to: category_id.into(),
        };
        category_ids.push(entry);
    }

    return category_ids;
}

async fn parse_category_page(category_id: &str) -> Vec<MyNode> {
    let api_category_url = make_search_api_url(category_id, "0");
    let api_request = get_page(&api_category_url);
    let response = match api_request.await {
        Ok(v) => v,
        _ => {
            println!("Error retrieving category id. {}", category_id);
            return Vec::new();
        }
    };

    let json = parse_json(&response);
    let filters = &json["universes"]["universe"][0]["facetmap"][0]["filter"][0]["filtersection"];

    let mut child_category_ids: Vec<MyNode> = Vec::new();
    for filter in filters.members() {
        let child_category_id_value = &filter["value"]["value"];
        let child_category_id = get_category_id(child_category_id_value.as_str().unwrap());

        let entry = MyNode {
            from: category_id.into(),
            to: child_category_id,
        };
        child_category_ids.push(entry)
    }

    return child_category_ids;
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
