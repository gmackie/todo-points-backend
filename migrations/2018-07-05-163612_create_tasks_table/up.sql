CREATE TABLE statusLevels (
  id SERIAL PRIMARY KEY,
  statusName VARCHAR NOT NULL,
  pointsRequired INT NOT NULL
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  firstName VARCHAR NOT NULL,
  lastName VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  active BOOLEAN NOT NULL DEFAULT 'f',
  currentPointTotal INT NOT NULL DEFAULT 0,
  currentStatus_id INT NOT NULL REFERENCES statusLevels (id),
  adminLevel INT NOT NULL DEFAULT 0
);

CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  description VARCHAR NOT NULL,
  completed BOOLEAN NOT NULL DEFAULT 'f',
  points INT NOT NULL,
  user_id INT NOT NULL REFERENCES users (id)
);
