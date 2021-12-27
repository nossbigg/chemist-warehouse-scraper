# chemist-warehouse-scraper

Scrapes the [Chemist Warehouse website](https://www.chemistwarehouse.com.au)! 🦀

_...because a CSV is so much better than hitting the search bar 764983768954 times lol_ 🙃

## Commands

1. `cargo run catalogs <max_depth>`:

   Retrieves taxonomy of product categories.

   `max-depth`: Max depth of categories to retrieve, `optional`, default: `1`

2. `cargo run catalog <category_id>`

   Retrieves products for a given category.

   Writes csv output (eg. `catalog_<category_id>.csv`)

   `category_id`: Product category ID to scrape from, `mandatory`

## References

- https://github.com/kadekillary/scraping-with-rust
- https://www.chemistwarehouse.com.au
