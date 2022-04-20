table! {
    posts (id) {
        id -> Int4,
        username -> Varchar,
        title -> Varchar,
        body -> Text,
        created_at -> Timestamp,
    }
}

table! {
    users (user_id) {
        user_id -> Varchar,
        username -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
