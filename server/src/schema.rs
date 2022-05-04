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
    solved_metas (post_id) {
        post_id -> Int4,
        comment_id -> Int4,
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
joinable!(solved_metas -> comments (comment_id));
joinable!(solved_metas -> posts (post_id));

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    solved_metas,
    users,
);
