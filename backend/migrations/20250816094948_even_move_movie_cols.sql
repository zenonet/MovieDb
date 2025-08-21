-- Add migration script here
ALTER TABLE movies
ADD aspect_ratio text,
ADD sound_formats text,
ADD added_to_archive TIMESTAMP,
ADD status text