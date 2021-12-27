use std::fmt;

#[derive(Clone)]
struct MyItem {
    name: String,
    brand: String,
    price: String,
    price_rrp: String,
    product_url: String,
}

impl fmt::Debug for MyItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}, {}",
            self.name, self.brand, self.price, self.price_rrp, self.product_url
        )
    }
}

const API_INDEX_INCREMENT: i32 = 45;

#[tokio::main]
pub async fn handle_catalog(category_id: String) {
    let mut items: Vec<MyItem> = Vec::new();

    let mut current_index: i32 = 0;
    loop {
        let url = make_search_api_url(&category_id, &current_index.to_string());
        let response = get_page(&url).await;
        let json = parse_json(response.unwrap().as_str());
        let repsonse_items = &json["universes"]["universe"][0]["items-section"]["items"]["item"];

        let mut count: i32 = 0;
        for repsonse_item in repsonse_items.members() {
            count = count + 1;
            let attributes = &repsonse_item["attribute"];

            let mut my_item = MyItem {
                name: "".to_string(),
                brand: "".to_string(),
                price: "".to_string(),
                price_rrp: "".to_string(),
                product_url: "".to_string(),
            };

            for attribute in attributes.members() {
                let attribute_name = attribute["name"].as_str().unwrap();
                let attribute_value = attribute["value"][0]["value"].as_str().unwrap().to_string();

                match attribute_name {
                    "name" => my_item.name = attribute_value,
                    "brand" => my_item.brand = attribute_value,
                    "price_cw_au" => my_item.price = attribute_value,
                    "rrp_cw_au" => my_item.price_rrp = attribute_value,
                    "producturl" => my_item.product_url = attribute_value,
                    _ => (),
                };
            }

            items.push(my_item);
        }

        if count < API_INDEX_INCREMENT {
            break;
        }

        current_index = current_index + API_INDEX_INCREMENT;
    }
}

async fn get_page(url: &str) -> Result<String, reqwest::Error> {
    let result = reqwest::get(url).await?.text().await?;
    return Ok(result);
}

fn parse_json(value: &str) -> json::JsonValue {
    json::parse(&value).unwrap()
}

fn make_search_api_url(category_id: &str, index: &str) -> String {
    return format!("https://www.chemistwarehouse.com.au/searchapi/webapi/search/category?category={}&index={}&sort=", category_id, index);
}
