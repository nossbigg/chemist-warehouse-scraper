use std::fs::create_dir_all;
use std::path::Path;

pub async fn get_page(url: &str) -> Result<String, reqwest::Error> {
    let result = reqwest::get(url).await?.text().await?;
    return Ok(result);
}

pub fn make_search_api_url(category_id: &str, index: &str) -> String {
    return format!("https://www.chemistwarehouse.com.au/searchapi/webapi/search/category?category={}&index={}&sort=", category_id, index);
}

pub fn parse_json(value: &str) -> json::JsonValue {
    json::parse(&value).unwrap()
}

pub fn create_output_dir() {
    create_dir_all(Path::new("output")).unwrap();
}
