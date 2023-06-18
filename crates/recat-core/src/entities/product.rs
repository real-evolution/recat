use chrono::{DateTime, Utc};
use reddd::domain::{Entity, Key, MutableEntity};
use reddd_macros::MutableEntity;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

/// A struct representing a product.
#[derive(Debug, MutableEntity, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    /// The unique identifier of the product.
    pub id: Key<Self, uuid::Uuid>,

    /// The title of the product.
    pub title: String,

    /// The quantity of the product.
    pub quantity: u32,

    /// The description of the product.
    pub description: Option<String>,

    /// The unique identifier of the owner of the product.
    pub owner_id: uuid::Uuid,

    /// The unique identifier of the price's token/currency.
    pub price_token_id: uuid::Uuid,

    /// The amount of the price's token/currency.
    pub price_amount: BigDecimal,

    /// The timestamp at which the product was created.
    pub created_at: DateTime<Utc>,

    /// The timestamp that the latest modification was made.
    pub updated_at: DateTime<Utc>,
}
