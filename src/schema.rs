table! {
    status_levels (id) {
        id -> Int4,
        status_name -> Varchar,
        points_required -> Int4,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        description -> Varchar,
        completed -> Bool,
        points -> Int4,
        user_id -> Int4,
        created_at -> Timestamp,
        due_by -> Nullable<Timestamp>,
        completed_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        hash -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        active -> Bool,
        current_point_total -> Int4,
        current_status_level_id -> Int4,
        admin_level -> Int4,
        created_at -> Timestamp,
    }
}

joinable!(tasks -> users (user_id));
joinable!(users -> status_levels (current_status_level_id));

allow_tables_to_appear_in_same_query!(
    status_levels,
    tasks,
    users,
);
