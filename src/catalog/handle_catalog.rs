use super::structs::MyItem;
use super::write_csv::write_csv;
use crate::common::utils::{get_page, make_search_api_url, parse_json};

const API_INDEX_INCREMENT: i32 = 45;

#[tokio::main]
pub async fn handle_catalog(category_id: String) {
    let items = get_items(&category_id).await;
    write_csv(items, &category_id);
}

async fn get_items(category_id: &str) -> Vec<MyItem> {
    let mut items: Vec<MyItem> = Vec::new();

    let mut current_index: i32 = 0;
    loop {
        let url = make_search_api_url(category_id, &current_index.to_string());
        let response = get_page(&url).await;
        let json = parse_json(response.unwrap().as_str());
        let repsonse_items = &json["universes"]["universe"][0]["items-section"]["items"]["item"];

        let mut count: i32 = 0;
        for repsonse_item in repsonse_items.members() {
            count = count + 1;
            let attributes = &repsonse_item["attribute"];

            let mut my_item = MyItem {
                id: "".to_string(),
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
                    "secondid" => my_item.id = attribute_value,
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

    items
}
