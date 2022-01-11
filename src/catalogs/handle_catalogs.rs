use regex::Regex;
use scraper::{Html, Selector};
use serde::Serialize;
use std::fmt;
use std::fs::File;

const CHEMIST_WAREHOUSE_URL_HOMEPAGE: &str = "https://www.chemistwarehouse.com.au";

#[derive(Clone, Serialize)]
struct MyNode {
    name: String,
    from: String,
    to: String,
}

impl fmt::Debug for MyNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} -> {}", self.name, self.from, self.to)
    }
}

#[tokio::main]
pub async fn handle_catalogs(max_catalogs_depth_str: String) {
    let max_catalogs_depth = max_catalogs_depth_str.parse().unwrap_or(1);

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

    let mut current_depth = 0;
    let mut all_nodes: Vec<MyNode> = Vec::new();
    all_nodes.append(&mut top_level_category_ids.to_owned());

    let mut current_nodes: Vec<MyNode> = top_level_category_ids;

    while current_nodes.len() > 0 && current_depth < max_catalogs_depth {
        let mut next_nodes: Vec<MyNode> = Vec::new();

        for entry in current_nodes {
            let mut result = parse_category_page(&entry.to).await;
            next_nodes.append(&mut result);
        }

        current_nodes = next_nodes.to_owned();
        all_nodes.append(&mut next_nodes);

        current_depth = current_depth + 1;
    }

    write_csv(all_nodes);
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

    fn get_category_url(e: &scraper::ElementRef) -> String {
        return e.value().attr("href").unwrap().replace("\"", "");
    }

    let categories_url_elements = document.select(&selector).filter(|e| {
        let href = get_category_url(&e);
        return category_url_matcher.is_match(&href);
    });

    let categories_url = categories_url_elements
        .map(|e| get_category_url(&e))
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
            name: "".to_string(),
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
        let child_category_name_value = &filter["link"]["name"];
        let child_category_name = child_category_name_value.as_str().unwrap();

        let child_category_id_value = &filter["value"]["value"];
        let child_category_id = match get_category_id(child_category_id_value.as_str().unwrap()) {
            Some(v) => v,
            None => continue,
        };

        let entry = MyNode {
            name: child_category_name.into(),
            from: category_id.into(),
            to: child_category_id,
        };
        child_category_ids.push(entry)
    }

    return child_category_ids;
}

fn get_category_id(value: &str) -> Option<String> {
    let category_id_matcher = Regex::new(r"chemau(\d+)$").unwrap();

    let captures = match category_id_matcher.captures_iter(&value).nth(0) {
        Some(v) => v,
        None => return None,
    };

    let category_id = &captures[1];
    return Some(category_id.into());
}

fn make_search_api_url(category_id: &str, index: &str) -> String {
    return format!("https://www.chemistwarehouse.com.au/searchapi/webapi/search/category?category={}&index={}&sort=", category_id, index);
}

fn write_csv(nodes: Vec<MyNode>) {
    let filename = format!("catalogs.csv");
    let file = File::create(filename).unwrap();

    let mut wtr = csv::Writer::from_writer(file);

    for node in nodes {
        match wtr.serialize(&node) {
            _ => (),
        };
    }
    match wtr.flush() {
        _ => (),
    };
}
