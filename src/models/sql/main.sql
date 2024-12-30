-- польователи
CREATE TABLE users (
    id         VARCHAR(50) NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name  VARCHAR(100) NOT NULL,
    email      VARCHAR(100) NOT NULL,
    password   VARCHAR(1000) NOT NULL,
    perm       SMALLINT NOT NULL,      -- 1 обычный, 10 партнер (заказчик), 60 админ
    level      SMALLINT NOT NULL,
    image      VARCHAR(500),
    UNIQUE(email)
); 

-- обратная связь
CREATE TABLE feedbacks (
    id       VARCHAR(50) NOT NULL,
    username VARCHAR(100) NOT NULL,
    email    VARCHAR(200) NOT NULL,
    message  VARCHAR(1000) NOT NULL
);

-- типы объекта с модулями
CREATE TABLE place_types (
    id    VARCHAR(100) NOT NULL,
    title VARCHAR(100) NOT NULL
);

-- типы объекта с модулями
CREATE TABLE places (
    id       VARCHAR(100) NOT NULL,
    title    VARCHAR(100) NOT NULL,
    types    SMALLINT NOT NULL,     -- 1 открыто, 2 редактирование, 3 закрыто
    created  TIMESTAMP NOT NULL,
    user_id  VARCHAR(50) NOT NULL,
    type_id  VARCHAR(50) NOT NULL,
    image    VARCHAR(500)
);

-- типы модуля
CREATE TABLE module_types (
    id       VARCHAR(100) NOT NULL,
    title    VARCHAR(100) NOT NULL,
    types    SMALLINT NOT NULL,     -- 1 открыто, 2 редактирование, 3 закрыто
    image    VARCHAR(500)
);

-- временная метка
CREATE TABLE times (
    id   VARCHAR(100) NOT NULL,
    time TIMESTAMP NOT NULL
);

-- модули для конструктора
CREATE TABLE modules (
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(100) NOT NULL,
    types       SMALLINT NOT NULL,
    place_id    VARCHAR(50) NOT NULL,
    type_id     VARCHAR(50) NOT NULL,
    price       INT NOT NULL,
    width       SMALLINT NOT NULL,
    height      SMALLINT NOT NULL,
    left        FLOAT NOT NULL,
    top         FLOAT NOT NULL,
    angle       FLOAT NOT NULL,
    font_color  VARCHAR(10) NOT NULL,
    font_size   VARCHAR(10) NOT NULL,
    back_color  VARCHAR(10) NOT NULL,
    image       VARCHAR(500),
);

-- заказы / бронирование
CREATE TABLE orders (
    id         VARCHAR(50) NOT NULL,
    title      VARCHAR(100) NOT NULL,
    types      SMALLINT NOT NULL,    -- 1 услуга, 2 товар, 3 работа
    place_id   VARCHAR(50) NOT NULL,
    object_id  VARCHAR(50) NOT NULL,
    created    TIMESTAMP NOT NULL,
    user_id    VARCHAR(50) NOT NULL,
    price      INT NOT NULL,
    time_start VARCHAR(50) NOT NULL, -- связь на times
    time_end   VARCHAR(50) NOT NULL  -- связь на times
);

-- логи
CREATE TABLE logs (
    id        VARCHAR(100) NOT NULL,
    user_id   VARCHAR(50) NOT NULL,
    text      VARCHAR(100) NOT NULL,
    order_id  VARCHAR(50) NOT NULL,
    place_id  VARCHAR(50) NOT NULL,
    created   TIMESTAMP NOT NULL
);