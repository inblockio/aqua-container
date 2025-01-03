// @generated automatically by Diesel CLI.

diesel::table! {
    pages (id) {
        id -> Integer,
        name -> Text,
        extension -> Text,
        page_data -> Text,
        owner -> Text,
        mode -> Text,
        created_at -> Text,
        is_shared -> Bool,
    }
}

diesel::table! {
    share_data (id) {
        id -> Nullable<Integer>,
        file_id -> Integer,
        identifier -> Text,
        created_time -> Text,
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

diesel::table! {
    user_profiles (id) {
        id -> Nullable<Integer>,
        address -> Text,
        chain -> Text,
        theme -> Text,
        contract_address -> Text,
        file_mode -> Text,
        domain_name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    pages,
    share_data,
    siwe_sessions,
    user_profiles,
);
