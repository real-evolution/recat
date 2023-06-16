// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Uuid,
        title -> Varchar,
        quantity -> Int4,
        description -> Nullable<Varchar>,
        owner_id -> Uuid,
        price_token_id -> Uuid,
        price_amount -> Numeric,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
