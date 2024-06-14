// @generated automatically by Diesel CLI.

diesel::table! {
    peers (id) {
        id -> Uuid,
        name -> Varchar,
        enabled -> Bool,
        persistent_keepalive -> Int4,
        allowed_ips -> Varchar,
        preshared_key -> Nullable<Varchar>,
        private_key -> Varchar,
        public_key -> Varchar,
        if_pubkey -> Varchar,
        address -> Varchar,
        transfer_rx -> Int4,
        transfer_tx -> Int4,
        last_handshake_at -> Nullable<Timestamp>,
        endpoint_addr -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        hashed_password -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(peers, users,);
