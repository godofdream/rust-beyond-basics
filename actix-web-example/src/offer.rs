use diesel::prelude::*;

use crate::product::products;

table! {
    offers (id) {
        id -> BigInt,
        price -> Text,
        product_name -> Text,
    }
}

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = offers)]
#[diesel(primary_key(id))]
pub struct Offer {
    pub id: i64,
    pub price: String,
    pub product_name: String,
}

diesel::joinable!(offers -> products (product_name));
