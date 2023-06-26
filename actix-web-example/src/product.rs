use diesel::prelude::*;
table! {
    products (product_name) {
        product_name -> Text,
        title -> Text,
        gtin -> Nullable<Text>,
    }
}
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = products)]
#[diesel(primary_key(product_name))]
pub struct Product {
    pub product_name: String,
    pub title: String,
    pub gtin: Option<String>,
}
