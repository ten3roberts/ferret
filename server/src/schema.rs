table! {
    comments (comment_id) {
        comment_id -> Int4,
        post_id -> Int4,
        user_id -> Varchar,
        body -> Text,
        created_at -> Timestamp,
    }
}

table! {
    posts (post_id) {
        post_id -> Int4,
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

joinable!(comments -> posts (post_id));
joinable!(comments -> users (user_id));
joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(comments, posts, users,);
