CREATE TABLE IF NOT EXISTS users(
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);

INSERT INTO users (username, password_hash) 
VALUES ('Pior', '$argon2id$v=19$m=19456,t=2,p=1$OhbBv8lw46AsJm2YZY/a4g$zvL2cWCwnRKEaPCrSmBL0kol6jWQf9w3nkJ5/QkzwIk');
