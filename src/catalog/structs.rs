use serde::Serialize;
use std::fmt;

#[derive(Clone, Serialize)]
pub struct MyItem {
    pub id: String,
    pub name: String,
    pub brand: String,
    pub price: String,
    pub price_rrp: String,
    pub product_url: String,
}

impl fmt::Debug for MyItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}, {}, {}",
            self.id, self.name, self.brand, self.price, self.price_rrp, self.product_url
        )
    }
}
