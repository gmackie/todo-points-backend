-- Your SQL goes here
CREATE TABLE points (
    id SERIAL PRIMARY KEY NOT NULL,
    user_id INT NOT NULL REFERENCES users (id),
    value INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE points_audit (
    id SERIAL PRIMARY KEY NOT NULL,
    user_id INT NOT NULL REFERENCES users (id),
    value INT NOT NULL DEFAULT 0,
    description VARCHAR,
    deposit_at TIMESTAMP WITH TIME ZONE NOT NULL
);