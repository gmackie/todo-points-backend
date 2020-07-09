CREATE TABLE labels (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  color VARCHAR NOT NULL,
  created_by INT NOT NULL REFERENCES users (id),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE task_labels (
  id SERIAL PRIMARY KEY,
  task_id INT NOT NULL REFERENCES tasks (id),
  label_id INT NOT NULL REFERENCES labels (id),
  created_by INT NOT NULL REFERENCES users (id),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE todo_labels (
  id SERIAL PRIMARY KEY,
  todo_id INT NOT NULL REFERENCES todos (id),
  label_id INT NOT NULL REFERENCES labels (id),
  created_by INT NOT NULL REFERENCES users (id),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL
);