-- Add migration script here
ALTER TABLE movies
ADD COLUMN tagline varchar(256),
ADD COLUMN cover_url varchar(256),
ADD COLUMN description text,
ADD COLUMN year_of_publication int