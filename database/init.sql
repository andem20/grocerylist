\c grocery_list;

CREATE USER anders WITH PASSWORD 'password';
GRANT ALL PRIVILEGES ON DATABASE grocery_list TO anders;

CREATE TABLE users (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    roles VARCHAR NOT NULL
);

INSERT INTO users (first_name, last_name, email, password, roles) VALUES ('Anders', 'Madsen', 'anderslm@hotmail.com', 'password', 'admin,user');
INSERT INTO users (first_name, last_name, email, password, roles) VALUES ('Nadia', 'Thomsen', 'nadiasophie@hotmail.com', 'password', 'admin,user');

CREATE TABLE lists (
    id uuid NOT NULL,
    title VARCHAR(255) NOT NULL,
    user_id uuid REFERENCES users(id) NOT NULL,
    PRIMARY KEY(id, user_id)
);

WITH list_id (id) AS (
   VALUES (gen_random_uuid())
) 
INSERT INTO lists (id, title, user_id) SELECT (SELECT * FROM list_id), 'Shopping', id FROM users;

INSERT INTO lists (id, title, user_id) VALUES (gen_random_uuid(), 'Personal', (SELECT id FROM users WHERE email = 'anderslm@hotmail.com'));

CREATE TABLE items (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    list_id uuid,
    name VARCHAR(255) NOT NULL,
    done BOOLEAN DEFAULT FALSE NOT NULL 
);

INSERT INTO items (list_id, name) SELECT DISTINCT id, 'Milk' FROM lists;