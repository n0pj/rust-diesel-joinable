table! {
    post (id) {
        id -> Integer,
        content -> Varchar,
        user_id -> Integer,
    }
}

table! {
    user (id) {
        id -> Integer,
        name -> Varchar,
    }
}

joinable!(post -> user (user_id));

allow_tables_to_appear_in_same_query!(
    post,
    user,
);
