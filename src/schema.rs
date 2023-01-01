// @generated automatically by Diesel CLI.

diesel::table! {
    item (item_id) {
        item_id -> Unsigned<Bigint>,
        product_id -> Unsigned<Bigint>,
    }
}

diesel::table! {
    product (product_id) {
        product_id -> Unsigned<Bigint>,
        name -> Varchar,
        description -> Nullable<Varchar>,
        image_uri -> Varchar,
        is_active -> Bool,
        price -> Unsigned<Float>,
    }
}

diesel::table! {
    user (user_id) {
        user_id -> Unsigned<Bigint>,
        first_name -> Varchar,
        surname -> Varchar,
        phone -> Varchar,
        email -> Varchar,
        password -> Varchar,
        is_active -> Bool,
        is_admin -> Bool,
    }
}

diesel::joinable!(item -> product (product_id));

diesel::allow_tables_to_appear_in_same_query!(item, product, user,);
