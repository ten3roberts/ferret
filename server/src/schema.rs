table! {
    posts (id) {
        id -> Int4,
        user_id -> Varchar,
        title -> Varchar,
        body -> Text,
        created_at -> Timestamp,
    }
}

table! {
    users (user_id) {
        user_id -> Varchar,
        picture -> Nullable<Varchar>,
        name -> Varchar,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
