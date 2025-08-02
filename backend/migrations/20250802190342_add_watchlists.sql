-- Add migration script here
CREATE TABLE watchlists(
    id uuid PRIMARY KEY,
    name varchar(256),
    description text,
    owner uuid REFERENCES persons(id)
);

CREATE TABLE watchlist_entries(
    movie_id uuid REFERENCES movies(id),
    watchlist_id uuid REFERENCES watchlists(id),
    idx int
);