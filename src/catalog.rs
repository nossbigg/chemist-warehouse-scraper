const CHEMIST_WAREHOUSE_URL_HOMEPAGE: &str = "https://www.chemistwarehouse.com.au/";

#[tokio::main]
pub async fn handle_catalogs() {
    let homepage_request = get_homepage();
    let body: String = match homepage_request.await {
        Ok(v) => v,
        _ => String::from(""),
    };

    println!("{}", body);
}

async fn get_homepage() -> Result<String, reqwest::Error> {
    let result = reqwest::get(CHEMIST_WAREHOUSE_URL_HOMEPAGE)
        .await?
        .text()
        .await?;
    return Ok(result);
}
