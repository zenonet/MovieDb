ALTER TABLE nights
ADD movie_id uuid;

ALTER TABLE movie_views
DROP COLUMN movie_id