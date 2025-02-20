/*
perm - статус пользователя
1 обычный
2 менеджер объекта
3 подал заявку на создание объектов
4 владелец объекта
5 модератор сайта
10 суперпользователь

21 заблокированный
*/
CREATE TABLE users (
    id         TEXT PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name  VARCHAR(100) NOT NULL,
    email      VARCHAR(100) NOT NULL,
    password   VARCHAR(1000) NOT NULL,
    perm       SMALLINT NOT NULL,
    level      SMALLINT NOT NULL,
    image      VARCHAR(500),
    uuid       BYTEA NOT NULL,
    UNIQUE(email)
);

CREATE TABLE partners (
    id       TEXT PRIMARY KEY,
    title    VARCHAR(100) NOT NULL,
    inn      VARCHAR(100) NOT NULL,
    types    SMALLINT NOT NULL,
    created  TIMESTAMP NOT NULL,
    user_id  VARCHAR(100) NOT NULL
);

-- обратная связь
CREATE TABLE feedbacks (
    id       TEXT PRIMARY KEY,
    username VARCHAR(100) NOT NULL,
    email    VARCHAR(200) NOT NULL,
    message  VARCHAR(1000) NOT NULL
); 

-- типы объекта с модулями
CREATE TABLE place_types (
    id    TEXT PRIMARY KEY,
    title VARCHAR(100) NOT NULL
);

/*
типы объекта с модулями
types 
0 подана заявка
1 открыто 
2 редактирование
3 закрыто
*/
CREATE TABLE places (
    id       TEXT PRIMARY KEY,
    title    VARCHAR(100) NOT NULL,
    types    SMALLINT NOT NULL,
    created  TIMESTAMP NOT NULL,
    user_id  VARCHAR(100) NOT NULL,
    type_id  VARCHAR(100) NOT NULL,
    image    VARCHAR(500),
    cord     VARCHAR(100)
);

/* */
CREATE TABLE place_managers (
    id        TEXT PRIMARY KEY,
    user_id   VARCHAR(100) NOT NULL,
    place_id  VARCHAR(100) NOT NULL
); 


/*
типы модуля
types 
1 открыто
2 редактирование
3 закрыто
*/
CREATE TABLE module_types (
    id       TEXT PRIMARY KEY,
    title    VARCHAR(100) NOT NULL,
    types    SMALLINT NOT NULL,     -- 1 открыто, 2 редактирование, 3 закрыто
    image    VARCHAR(500)
);

-- временная метка
CREATE TABLE times (
    id   TEXT PRIMARY KEY,
    time TIMESTAMP NOT NULL
);

-- модули для конструктора
CREATE TABLE modules (
    id          TEXT PRIMARY KEY,
    title       VARCHAR(100) NOT NULL,
    types       SMALLINT NOT NULL,
    place_id    VARCHAR(100) NOT NULL,
    type_id     VARCHAR(100) NOT NULL,
    price       INT NOT NULL,
    _width      SMALLINT NOT NULL,
    _height     SMALLINT NOT NULL,
    _left       FLOAT NOT NULL,
    _top        FLOAT NOT NULL,
    _angle      FLOAT NOT NULL,
    font_color  VARCHAR(10) NOT NULL,
    font_size   VARCHAR(10) NOT NULL,
    back_color  VARCHAR(10) NOT NULL,
    image       VARCHAR(500)
);

-- заказы / бронирование
CREATE TABLE orders (
    id         TEXT PRIMARY KEY,
    title      VARCHAR(100) NOT NULL,
    types      SMALLINT NOT NULL,    -- 1 услуга, 2 товар, 3 работа
    place_id   VARCHAR(100) NOT NULL,
    object_id  VARCHAR(100) NOT NULL,
    created    TIMESTAMP NOT NULL,
    user_id    VARCHAR(100) NOT NULL,
    price      INT NOT NULL,
    time_start VARCHAR(100) NOT NULL, -- связь на times
    time_end   VARCHAR(100) NOT NULL  -- связь на times
);

-- логи
CREATE TABLE logs (
    id        TEXT PRIMARY KEY,
    user_id   VARCHAR(100) NOT NULL,
    text      VARCHAR(100) NOT NULL,
    order_id  VARCHAR(100) NOT NULL,
    place_id  VARCHAR(100) NOT NULL,
    created   TIMESTAMP NOT NULL
);

CREATE TABLE email_verification_token (
    id         BYTEA PRIMARY KEY,
    email      TEXT UNIQUE NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL
);