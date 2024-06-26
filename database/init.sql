\c grocery_list;

CREATE USER anders WITH PASSWORD 'password';
GRANT ALL PRIVILEGES ON DATABASE grocery_list TO anders;

CREATE TABLE users (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    roles VARCHAR NOT NULL
);

INSERT INTO users (first_name, last_name, username, password, roles) VALUES ('Anders', 'Madsen', 'anderslm@hotmail.com', 'password', 'admin,user');
INSERT INTO users (first_name, last_name, username, password, roles) VALUES ('Nadia', 'Thomsen', 'nadiasophie@hotmail.com', 'password', 'admin,user');

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

INSERT INTO lists (id, title, user_id) VALUES (gen_random_uuid(), 'Personal', (SELECT id FROM users WHERE username = 'anderslm@hotmail.com'));

CREATE TABLE categories (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

INSERT INTO categories (name) VALUES ('dairy'), ('meat'), ('bakery'), ('canned'), ('clothe'), ('snack'), ('vegetable'), ('fruit'), ('beverage');

CREATE TABLE items (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    list_id uuid,
    name VARCHAR(255) NOT NULL,
    done BOOLEAN DEFAULT FALSE NOT NULL,
    deleted BOOLEAN DEFAULT FALSE NOT NULL,
    lat FLOAT,
    lng FLOAT,
    cluster SMALLINT,
    category VARCHAR(255)
);

INSERT INTO items (list_id, name, lat, lng) SELECT DISTINCT id, 'Milk', 55.3530717, 10.3442589 FROM lists;
INSERT INTO items (list_id, name, lat, lng) SELECT DISTINCT id, 'Cheese', 55.3530717, 10.3442589 FROM lists;
INSERT INTO items (list_id, name, lat, lng) SELECT DISTINCT id, 'Steak', 55.3530621, 10.3442499 FROM lists;
INSERT INTO items (list_id, name, lat, lng) SELECT DISTINCT id, 'Chicken', 55.3530621, 10.3442499 FROM lists;
INSERT INTO items (list_id, name, lat, lng) SELECT DISTINCT id, 'Pasta', 55.3530628, 10.3442416 FROM lists;
INSERT INTO items (list_id, name, lat, lng) SELECT DISTINCT id, 'Rice', 55.3530628, 10.3442416 FROM lists;
INSERT INTO items (list_id, name, lat, lng) SELECT DISTINCT id, 'Cola', 55.3531045, 10.3443968 FROM lists;