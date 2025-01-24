// @generated automatically by Diesel CLI.

diesel::table! {
    AquaForms (hash) {
        hash -> Text,
        key -> Nullable<Text>,
        value -> Nullable<Text>,
        value_type -> Nullable<Text>,
    }
}

diesel::table! {
    Content (hash) {
        hash -> Text,
        content -> Nullable<Text>,
        reference_count -> Nullable<Int4>,
    }
}

diesel::table! {
    Contract (hash) {
        hash -> Text,
        latest -> Nullable<Array<Nullable<Text>>>,
        sender -> Nullable<Text>,
        receiver -> Nullable<Text>,
        option -> Nullable<Text>,
        reference_count -> Nullable<Int4>,
    }
}

diesel::table! {
    FileHash (hash) {
        hash -> Text,
        file_hash -> Nullable<Text>,
        reference_count -> Nullable<Int4>,
    }
}

diesel::table! {
    Index (hash, file_hash) {
        hash -> Text,
        file_hash -> Text,
        uri -> Nullable<Text>,
    }
}

diesel::table! {
    Latest (hash) {
        hash -> Text,
        owner -> Nullable<Text>,
    }
}

diesel::table! {
    Link (hash) {
        hash -> Text,
        link_type -> Nullable<Text>,
        link_require_indepth_verification -> Nullable<Bool>,
        link_verification_hash -> Nullable<Text>,
        reference_count -> Nullable<Int4>,
    }
}

diesel::table! {
    MerkleNodes (id) {
        id -> Int4,
        node_hash -> Nullable<Text>,
        parent_hash -> Nullable<Text>,
        height -> Nullable<Int4>,
        is_leaf -> Nullable<Bool>,
        left_child_hash -> Nullable<Text>,
        right_child_hash -> Nullable<Text>,
    }
}

diesel::table! {
    Revision (hash) {
        hash -> Text,
        owner -> Text,
        nonce -> Text,
        shared -> Nullable<Array<Nullable<Text>>>,
        contract -> Nullable<Array<Nullable<Text>>>,
        previous -> Nullable<Varchar>,
        children -> Nullable<Text>,
        local_timestamp -> Nullable<Timestamp>,
        revision_type -> Nullable<Text>,
        verification_leaves -> Nullable<Text>,
    }
}

diesel::table! {
    Settings (user_pub_key) {
        user_pub_key -> Text,
        cli_pub_key -> Nullable<Text>,
        cli_priv_key -> Nullable<Text>,
        witness_network -> Nullable<Text>,
        witness_contract_address -> Nullable<Text>,
        theme -> Nullable<Text>,
    }
}

diesel::table! {
    Signature (hash) {
        hash -> Text,
        signature_digest -> Nullable<Text>,
        signature_wallet_address -> Nullable<Varchar>,
        signature_type -> Nullable<Text>,
    }
}

diesel::table! {
    User (user) {
        user -> Text,
    }
}

diesel::table! {
    Witness (hash) {
        hash -> Text,
        witness_merkle_root -> Nullable<Text>,
    }
}

diesel::table! {
    WitnessEvent (witness_merkle_root) {
        witness_merkle_root -> Text,
        witness_timestamp -> Nullable<Timestamp>,
        witness_network -> Nullable<Text>,
        witness_smart_contract_address -> Nullable<Text>,
        witness_transaction_hash -> Nullable<Text>,
        witness_sender_account_address -> Nullable<Text>,
    }
}

diesel::table! {
    siwe_sessions (id) {
        id -> Int4,
        address -> Text,
        nonce -> Text,
        issued_at -> Timestamptz,
        expiration_time -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(AquaForms -> Revision (hash));
diesel::joinable!(Content -> Revision (hash));
diesel::joinable!(Contract -> Revision (hash));
diesel::joinable!(Contract -> User (hash));
diesel::joinable!(FileHash -> Revision (hash));
diesel::joinable!(Latest -> User (hash));
diesel::joinable!(Link -> Revision (hash));
diesel::joinable!(MerkleNodes -> Witness (node_hash));
diesel::joinable!(Revision -> Latest (hash));
diesel::joinable!(Settings -> User (user_pub_key));
diesel::joinable!(Signature -> Revision (hash));
diesel::joinable!(Witness -> Revision (hash));

diesel::allow_tables_to_appear_in_same_query!(
    AquaForms,
    Content,
    Contract,
    FileHash,
    Index,
    Latest,
    Link,
    MerkleNodes,
    Revision,
    Settings,
    Signature,
    User,
    Witness,
    WitnessEvent,
    siwe_sessions,
);
