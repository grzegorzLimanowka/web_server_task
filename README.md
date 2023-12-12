# Actix Simple Crud

Goal of this project was to create a web server (prefferably in actix-web)
that will have endpoint for sending requests to external API, retrieve this data
and store it in local database (and provide REST API for manipulating each request)

## Running project

1. Run MySQL in docker

    ```shell
    docker run --name some-mysql -e MYSQL_ROOT_PASSWORD=my-secret-pw -d -p 3306:3306 mysql
    ```

2. Get inside container

    ```shell
    docker exec -it some-mysql bash
    ```

3. Create DB

    ```shell
    create database testing1; 
    ```

4. Run migrations

    ```shell
    DATABASE_URL=mysql://root:my-secret-pw@localhost:3306/testing1 sea-orm-cli migrate up --migration-dir crates/migration
    ```

5. Generate entities (codegen based on migrations)

    ```shell
    sea-orm-cli generate entity --database-url mysql://root:my-secret-pw@localhost:3306/testing1 --output-dir crates/entity/src
    ```

6. Run server:

    ```shell
    DATABASE_URL=mysql://root:my-secret-pw@localhost:3306/testing1 HOST=localhost PORT=8000 cargo run --bin server
    ```

7. GET /run endpoint (go to browser and write):

    ```path
    http://localhost:8000/run
    ```

## Migrations

Before running project MYSQL database has to be running.
When DB is running, we can run migrations

```shell
sea-orm-cli migrate COMMAND -d ./crates/migration
```

## Tests

For now this project has only unit tests, integration tests should be added

### Unit tests

```shell
cargo test
```

### Integration tests

TODO
