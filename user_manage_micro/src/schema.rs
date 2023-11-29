// @generated automatically by Diesel CLI.
// Schema Definition
table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        full_name -> Text,
        email -> Text,
        phone_number -> BigInt,
        address -> Text,
    }
}
