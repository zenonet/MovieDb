# Move DB

This is a fullstack webapp for managing movie collections.

## Background

My family has quite a big collection of movies on blu-ray at home. We've sorted them in disk-bags, numbered them and managed them in a DB made in the android app "Memento DB". Until now!
While MementoDB was incredibly useful over the years, especially because of how easy it is to configure, the movie collection is slowly outgrowing it. Memento only has limited synchronization functionality and to access the database, it has to be completely downloaded and running locally on your device. Memento is also obviously not as extensible as a custom fullstack app. So I decided to create just that.

## Tech Stack

The application has 3 layers

- Frontend (typescript webfrontent using the Svelte framework)
- API (built in Rust with axum, sql interaction using sqlx)
- DB (postgres)