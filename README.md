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

Running migrations:

DATABASE_URL=mysql://root:my-secret-pw@localhost:3306/testing1 sea-orm-cli migrate up

Docker - task

docker run -p 33060:33060 -p 3306:3306 --name db-mysql -e MYSQL_ROOT_PASSWORD=admin -d mysql:8.2

Start mysql DB in docker
❯ docker run --name some-mysql -e MYSQL_ROOT_PASSWORD=my-secret-pw -d -p 3306:3306 mysql

Go to mysql CLI
❯ docker exec -it some-mysql bash

Apply migrations (SeaORM)
❯ DATABASE_URL=mysql://root:my-secret-pw@localhost:3306/testing1 sea-orm-cli migrate up
