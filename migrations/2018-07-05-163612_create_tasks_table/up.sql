CREATE TABLE status_levels (
  id SERIAL PRIMARY KEY,
  status_name VARCHAR NOT NULL,
  points_required INT NOT NULL
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  hash VARCHAR NOT NULL,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  active BOOLEAN NOT NULL DEFAULT 'f',
  current_point_total INT NOT NULL DEFAULT 0,
  current_status_level_id INT NOT NULL REFERENCES status_levels (id),
  admin_level INT NOT NULL DEFAULT 0,
  created_at TIMESTAMP NOT NULL
);

CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  description VARCHAR NOT NULL,
  completed BOOLEAN NOT NULL DEFAULT 'f',
  points INT NOT NULL,
  user_id INT NOT NULL REFERENCES users (id),
  created_at TIMESTAMP NOT NULL,
  due_by TIMESTAMP,
  completed_at TIMESTAMP
);
