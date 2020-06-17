table! {
    login_history (id) {
        id -> Int4,
        user_id -> Int4,
        login_timestamp -> Timestamptz,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        description -> Varchar,
        points -> Int4,
        user_id -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    todos (id) {
        id -> Int4,
        description -> Varchar,
        completed -> Bool,
        points -> Int4,
        user_id -> Int4,
        created_at -> Timestamptz,
        due_by -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamptz,
        login_session -> Varchar,
    }
}

joinable!(login_history -> users (user_id));
joinable!(tasks -> users (user_id));
joinable!(todos -> users (user_id));

allow_tables_to_appear_in_same_query!(
    login_history,
    tasks,
    todos,
    users,
);
