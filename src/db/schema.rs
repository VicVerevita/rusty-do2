// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        finished -> Bool,
        created_at -> Nullable<Timestamp>,
        user_id -> Uuid,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
    }
}

diesel::joinable!(tasks -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(tasks, users,);
