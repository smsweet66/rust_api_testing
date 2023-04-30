// @generated automatically by Diesel CLI.

diesel::table! {
    profiles (pid) {
        pid -> Integer,
        uid -> Integer,
        name -> Text,
        body_sizes -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (uid) {
        uid -> Integer,
        name -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(profiles -> users (uid));

diesel::allow_tables_to_appear_in_same_query!(
    profiles,
    users,
);
