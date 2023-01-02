// @generated automatically by Diesel CLI.

diesel::table! {
    items (item_id) {
        item_id -> Int8,
        product_id -> Int8,
    }
}

diesel::table! {
    products (product_id) {
        product_id -> Int8,
        name -> Varchar,
        description -> Nullable<Varchar>,
        image_uri -> Varchar,
        is_active -> Bool,
        price -> Float4,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int8,
        first_name -> Varchar,
        surname -> Varchar,
        phone -> Varchar,
        email -> Varchar,
        password -> Varchar,
        is_active -> Bool,
        is_admin -> Bool,
    }
}

diesel::joinable!(items -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(items, products, users,);
