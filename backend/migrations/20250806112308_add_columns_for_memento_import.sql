-- Add migration script here
ALTER TABLE movies
ADD trailer_url varchar(512),
ADD wikipedia_url varchar(512),
ADD runtime int,
ADD medium varchar(32),
ADD age_rating int,
ADD disk_bag int,
ADD director text,
ADD producer text,
ADD studio text,
ADD locations text,
ADD actors text,
ADD owner uuid REFERENCES persons(id),
ADD memento_id char(20),
ADD overview text,
ADD tagline text