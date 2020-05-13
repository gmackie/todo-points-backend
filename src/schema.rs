table! {
    statuslevels (id) {
        id -> Int4,
        statusname -> Varchar,
        pointsrequired -> Int4,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        description -> Varchar,
        completed -> Bool,
        points -> Int4,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        firstname -> Varchar,
        lastname -> Varchar,
        email -> Varchar,
        active -> Bool,
        currentpointtotal -> Int4,
        currentstatus_id -> Int4,
        adminlevel -> Int4,
    }
}

joinable!(tasks -> users (user_id));
joinable!(users -> statuslevels (currentstatus_id));

allow_tables_to_appear_in_same_query!(
    statuslevels,
    tasks,
    users,
);
