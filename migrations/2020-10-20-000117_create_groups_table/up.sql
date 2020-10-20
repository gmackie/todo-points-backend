-- Your SQL goes here
CREATE TABLE groups (
    id SERIAL PRIMARY KEY NOT NULL,
    group_name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_by INT NOT NULL REFERENCES users (id)
);

CREATE TABLE users_groups (
    id SERIAL PRIMARY KEY NOT NULL,
    user_id INT NOT NULL REFERENCES users (id),
    group_id INT NOT NULL REFERENCES groups (id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
);