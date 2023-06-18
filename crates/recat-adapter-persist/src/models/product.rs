use chrono::{DateTime, Utc};
use diesel::prelude::*;
use recat_core::entities::Product;
use reddd::domain::{Key, ValueType};
use bigdecimal::BigDecimal;

use crate::schema::products;

#[derive(Debug, Clone, Identifiable, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = products)]
pub(crate) struct ProductModel {
    #[primary_key]
    pub(crate) id: uuid::Uuid,
    pub(crate) title: String,
    pub(crate) quantity: i32,
    pub(crate) description: Option<String>,
    pub(crate) owner_id: uuid::Uuid,
    pub(crate) price_token_id: uuid::Uuid,
    pub(crate) price_amount: BigDecimal,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

impl super::Model for ProductModel {
    type Entity = Product;

    fn from_entity(entity: Self::Entity) -> Self {
        Self {
            id: entity.id.into_inner(),
            title: entity.title,
            quantity: entity.quantity,
            description: entity.description,
            owner_id: entity.owner_id,
            price_token_id: entity.price_token_id,
            price_amount: entity.price_amount,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }

    fn into_entity(self) -> Self::Entity {
        Self::Entity {
            id: Key::new(self.id),
            title: self.title,
            quantity: self.quantity,
            description: self.description,
            owner_id: self.owner_id,
            price_token_id: self.price_token_id,
            price_amount: self.price_amount,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
