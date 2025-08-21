-- Add migration script here
CREATE TABLE movies(
    id uuid PRIMARY KEY,
    name varchar(256) NOT NULL
);


CREATE TABLE persons(
    id uuid PRIMARY KEY,
    name VARCHAR(128) UNIQUE
);

CREATE TABLE nights(
    id uuid PRIMARY KEY,
    time TIMESTAMPTZ NOT NULL,
    description text
);

CREATE TABLE movie_views(
    id uuid PRIMARY KEY,
    movie_id uuid REFERENCES movies(id) NOT NULL,
    night_id uuid REFERENCES nights(id) NOT NULL,
    person_id uuid REFERENCES persons(id) NOT NULL
);

CREATE TABLE ratings(
    id uuid PRIMARY KEY,
    movie_view_id uuid REFERENCES movie_views(id) NOT NULL,
    value float
);