CREATE TABLE completed_tasks (
  id SERIAL PRIMARY KEY,
  task_id INT NOT NULL REFERENCES tasks (id),
  completed_at TIMESTAMP WITH TIME ZONE NOT NULL,
  user_id INT NOT NULL REFERENCES users (id),
  points INT NOT NULL DEFAULT 0
);

CREATE TABLE balances (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL REFERENCES users (id),
  label_id INT NOT NULL REFERENCES labels (id),
  points INT NOT NULL DEFAULT 0,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);