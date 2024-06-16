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
        interface_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        hashed_password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    wg_if (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        pubkey -> Varchar,
        #[max_length = 255]
        privkey -> Varchar,
        #[max_length = 255]
        address -> Varchar,
        port -> Int4,
        mtu -> Nullable<Int4>,
        fwmark -> Nullable<Int4>,
    }
}

diesel::joinable!(peers -> wg_if (interface_id));

diesel::allow_tables_to_appear_in_same_query!(
    peers,
    users,
    wg_if,
);
