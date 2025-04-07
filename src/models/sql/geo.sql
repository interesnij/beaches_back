CREATE TABLE regions (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    geo_id      INT,
    country_id  INT NOT NULL,
    timezone_id INT,
    cord        VARCHAR(100)
);
CREATE INDEX regions_country_idx ON regions (country_id);
----------------------------

CREATE TABLE cities (
    id         SERIAL PRIMARY KEY,
    name       VARCHAR(100) NOT NULL,
    geo_id     INT,
    region_id  INT, 
    country_id INT NOT NULL,
    cord       VARCHAR(100)
);
CREATE INDEX cities_country_idx ON cities (country_id);
CREATE INDEX cities_region_idx ON cities (region_id);