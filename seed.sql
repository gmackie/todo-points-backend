INSERT INTO statusLevels (statusname, pointsrequired)
VALUES 
    ('None', 0),
    ('Bronze', 250),
    ('Silver', 500),
    ('Gold', 1000);

INSERT INTO users (username, firstname, lastname, email, active, currentpointtotal, currentstatus_id, adminlevel)
VALUES 
    ('mackieg', 'Graham', 'Mackie', 'graham.mackie@gmail.com', 't', 1337, 1, 101);

INSERT INTO tasks (description, points, user_id)
VALUES 
    ('my first task', 100, 1),
    ('another one', 150, 1);
