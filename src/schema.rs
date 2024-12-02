// @generated automatically by Diesel CLI.

diesel::table! {
    pages (id) {
        id -> Nullable<Integer>,
        name -> Text,
        extension -> Text,
        page_data -> Text,
        owner -> Text,
        mode -> Text,
        created_at -> Text,
    }
}

diesel::table! {
    siwe_sessions (id) {
        id -> Nullable<Integer>,
        address -> Text,
        nonce -> Text,
        issued_at -> Text,
        expiration_time -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    pages,
    siwe_sessions,
);
