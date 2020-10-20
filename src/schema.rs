table! {
    balances (id) {
        id -> Int4,
        user_id -> Int4,
        label_id -> Int4,
        points -> Int4,
        updated_at -> Timestamptz,
    }
}

table! {
    completed_tasks (id) {
        id -> Int4,
        task_id -> Int4,
        completed_at -> Timestamptz,
        user_id -> Int4,
        points -> Int4,
    }
}

table! {
    groups (id) {
        id -> Int4,
        group_name -> Varchar,
        description -> Varchar,
        created_at -> Timestamptz,
        created_by -> Int4,
    }
}

table! {
    labels (id) {
        id -> Int4,
        name -> Varchar,
        color -> Varchar,
        created_by -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    login_history (id) {
        id -> Int4,
        user_id -> Int4,
        login_timestamp -> Timestamptz,
    }
}

table! {
    points (id) {
        id -> Int4,
        user_id -> Int4,
        value -> Int4,
    }
}

table! {
    points_audit (id) {
        id -> Int4,
        user_id -> Int4,
        value -> Int4,
        description -> Nullable<Varchar>,
        deposit_at -> Timestamptz,
    }
}

table! {
    task_labels (id) {
        id -> Int4,
        task_id -> Int4,
        label_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamptz,
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
    todo_labels (id) {
        id -> Int4,
        todo_id -> Int4,
        label_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    todos (id) {
        id -> Int4,
        description -> Varchar,
        points -> Int4,
        user_id -> Int4,
        created_at -> Timestamptz,
        due_by -> Nullable<Timestamptz>,
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

table! {
    users_groups (id) {
        id -> Int4,
        user_id -> Int4,
        group_id -> Int4,
        created_at -> Timestamptz,
    }
}

joinable!(balances -> labels (label_id));
joinable!(balances -> users (user_id));
joinable!(completed_tasks -> tasks (task_id));
joinable!(completed_tasks -> users (user_id));
joinable!(groups -> users (created_by));
joinable!(labels -> users (created_by));
joinable!(login_history -> users (user_id));
joinable!(points -> users (user_id));
joinable!(points_audit -> users (user_id));
joinable!(task_labels -> labels (label_id));
joinable!(task_labels -> tasks (task_id));
joinable!(task_labels -> users (created_by));
joinable!(tasks -> users (user_id));
joinable!(todo_labels -> labels (label_id));
joinable!(todo_labels -> todos (todo_id));
joinable!(todo_labels -> users (created_by));
joinable!(todos -> users (user_id));
joinable!(users_groups -> groups (group_id));
joinable!(users_groups -> users (user_id));

allow_tables_to_appear_in_same_query!(
    balances,
    completed_tasks,
    groups,
    labels,
    login_history,
    points,
    points_audit,
    task_labels,
    tasks,
    todo_labels,
    todos,
    users,
    users_groups,
);
