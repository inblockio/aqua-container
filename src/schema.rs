// @generated automatically by Diesel CLI.

diesel::table! {
    aqua_chain (id) {
        id -> Nullable<Integer>,
        file_hash -> Text,
        file_name -> Text,
        revision -> Text,
        owner -> Text,
        mode -> Text,
        share_code -> Nullable<Text>,
        is_shared -> Bool,
        updated_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    pages (id) {
        id -> Nullable<Integer>,
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
    revisions (id) {
        id -> Nullable<Integer>,
        revision_hash -> Text,
        previous_verification_hash -> Text,
        nonce -> Text,
        local_timestamp -> Text,
        revision_type -> Text,
        file_hash -> Nullable<Text>,
        content -> Nullable<Text>,
        link_type -> Nullable<Text>,
        link_require_indepth_verification -> Nullable<Text>,
        link_verification_hash -> Nullable<Text>,
        link_uri -> Nullable<Text>,
        signature_data -> Nullable<Text>,
        signature_public_key -> Nullable<Text>,
        signature_wallet_address -> Nullable<Text>,
        signature_type -> Nullable<Text>,
        witness_merkle_root -> Nullable<Text>,
        witness_timestamp -> Nullable<Text>,
        witness_network -> Nullable<Text>,
        witness_smart_contract_address -> Nullable<Text>,
        witness_transaction_hash -> Nullable<Text>,
        witness_sender_account_address -> Nullable<Text>,
        leaves -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
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
    aqua_chain,
    pages,
    revisions,
    share_data,
    siwe_sessions,
    user_profiles,
);
