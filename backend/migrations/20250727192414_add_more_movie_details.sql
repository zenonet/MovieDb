-- Add migration script here
ALTER TABLE movies
ADD COLUMN cover_url text,
ADD COLUMN description text,
ADD COLUMN year_of_publication int