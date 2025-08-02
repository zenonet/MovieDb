-- Add migration script here
ALTER TABLE watchlists
ALTER COLUMN name SET not null;

ALTER TAbLE watchlist_entries
ALTER COLUMN idx set not null,
ALTER COLUMN movie_id set not null,
ALTER column watchlist_id set not null;