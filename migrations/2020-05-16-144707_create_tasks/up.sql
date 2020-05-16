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