// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Bpchar,
        display_name -> Varchar,
        provider -> Int4,
        provider_id -> Varchar,
        last_login_date -> Timestamp,
    }
}
