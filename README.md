# chemist-warehouse-scraper

Scrapes the [Chemist Warehouse website](https://www.chemistwarehouse.com.au)! 🦀

_...because a CSV is so much better than hitting the search bar 764983768954 times lol_ 🙃

## Commands

### 1. Get catalogs (ie. table of contents)

Retrieves taxonomy of product categories.

`cargo run catalogs <max_depth>`:

Inputs:

1. `max-depth`: Max depth of categories to retrieve, `optional`, default: `1`

Outputs: `output/catalogs.csv`

### 2. Get items from a specific catalog

Retrieves products for a given category.

Command: `cargo run catalog <category_id>`

Inputs:

1. `category_id`: Product category ID to scrape from, `mandatory`

Outputs: `output/catalog_<category_id>.csv`

## References

- https://github.com/kadekillary/scraping-with-rust
- https://www.chemistwarehouse.com.au
