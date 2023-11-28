# Actix Simple Crud

Goal of this project was to create a web server (prefferably in actix-web)
that will have endpoint for sending requests to external API, retrieve this data
and store it in local database (and provide REST API for manipulating each request)

## Migrations

Before running project MYSQL database has to be running.
When DB is running, we can run migrations

```shell
sea-orm-cli migrate COMMAND -d ./crates/migration
```

## Running project

..

## Tests

...

### Unit tests

...

### Integration tests

...
