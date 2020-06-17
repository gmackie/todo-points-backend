CREATE TABLE completed_tasks (
  id SERIAL PRIMARY KEY,
  task_id INT NOT NULL REFERENCES tasks (id),
  completed_at TIMESTAMP WITH TIME ZONE NOT NULL,
  user_id INT NOT NULL REFERENCES users (id),
  points INT NOT NULL DEFAULT 0
);

CREATE TABLE completed_todos (
  id SERIAL PRIMARY KEY,
  todo_id INT NOT NULL REFERENCES todos (id),
  completed_at TIMESTAMP WITH TIME ZONE NOT NULL,
  user_id INT NOT NULL REFERENCES users (id),
  points INT NOT NULL DEFAULT 0
);