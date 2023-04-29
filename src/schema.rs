// @generated automatically by Diesel CLI.

diesel::table! {
    profiles (id) {
        id -> Integer,
        user_id -> Integer,
        name -> Text,
        body_sizes -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(profiles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    profiles,
    users,
);
