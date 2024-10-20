// @generated automatically by Diesel CLI.

diesel::table! {
    connections (id) {
        id -> Nullable<Integer>,
        device1_id -> Text,
        device2_id -> Text,
        connection_type -> Nullable<Text>,
        bandwidth -> Nullable<Text>,
        status -> Nullable<Text>,
        exp -> Text,
    }
}

diesel::table! {
    devices (id) {
        id -> Nullable<Integer>,
        name -> Text,
        device_type -> Text,
        ip_address -> Nullable<Text>,
        mac_address -> Nullable<Text>,
        location -> Nullable<Text>,
        description -> Nullable<Text>,
        exp -> Text,
    }
}

diesel::table! {
    networks (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        cidr -> Nullable<Text>,
        description -> Nullable<Text>,
        exp -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    connections,
    devices,
    networks,
);
