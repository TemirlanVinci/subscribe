CREATE TABLE categories (
    id   INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

INSERT INTO categories VALUES (1, 'Подписки');
INSERT INTO categories VALUES (2, 'Звёзды');

CREATE TABLE products (
    id          INTEGER PRIMARY KEY,
    category_id INTEGER REFERENCES categories(id),
    name        TEXT NOT NULL,
    description TEXT,
    price       INTEGER NOT NULL, 
    stock_type  INTEGER NOT NULL      
);

INSERT INTO products VALUES (1, 1, 'Telegram Premium 1 месяц', 'премиум хуня для лохов покупай', 199, 1);
INSERT INTO products VALUES (2, 1, 'Telegram Premium 3 месяца', 'премиум хуня для лохов покупай', 549, 1);
INSERT INTO products VALUES (3, 1, 'Telegram Premium 6 месяцев', 'премиум хуня для лохов покупай', 899, 1);
INSERT INTO products VALUES (4, 1, 'Telegram Premium 1 год', 'премиум хуня для лохов покупай', 1599, 1);

INSERT INTO products VALUES (5, 2, 'Telegram Stars 100 штук', 'звезды хуня покупай', 99, 2);
INSERT INTO products VALUES (6, 2, 'Telegram Stars 200 штук', 'звезды хуня покупай', 189, 2);
INSERT INTO products VALUES (7, 2, 'Telegram Stars 500 штук', 'звезды хуня покупай', 399, 2);
INSERT INTO products VALUES (8, 2, 'Telegram Stars 1000 штук', 'звезды хуня покупай', 799, 2);

CREATE TABLE treasury (
    id       INTEGER PRIMARY KEY,
    quantity BIGINT NOT NULL
);

INSERT INTO treasury VALUES (1, 10000);
INSERT INTO treasury VALUES (2, 1000);

CREATE TABLE users (
    id            INTEGER PRIMARY KEY,
    telegram_id   BIGINT UNIQUE NOT NULL
);

CREATE TABLE orders (
    id              INTEGER PRIMARY KEY,
    user_id         BIGINT REFERENCES users(telegram_id),
    product_id      INTEGER REFERENCES products(id), 
    quantity        INTEGER NOT NULL DEFAULT 1, 
    status          TEXT DEFAULT 'pending'
);