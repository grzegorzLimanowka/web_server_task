# Actix Simple Crud

Goal of this project was to create a web server (prefferably in actix-web)
that will have endpoint for sending requests to external API, retrieve this data
and store it in local database.

## Endpoint /run

Calling this endpoint (`POST`? `GET`? )should cause application to:

- [ ] Send 30 POST requests to <https://httpbin.org/post>
  - [ ] Each request should have JSON body in format {"value": N}, where N - randomly generated number in range od [0,10]
  - [ ] Server should parse the responses and extract json.value from each JSON response.
- [ ] It should calculate and return collection containing the numbers that appear more than once (most frequent numbers) in asc. order
- [ ] If any request fails, server should log error, but continue procesing the remaining requests
- [ ] Server should save each request into MySql database. Each stored request should have a unique identifier and generated value

Optional:

- [ ] REST-like CRUD API
  - [ ] should be implemented for managing the stored data in the MySql database.
  - [ ] API should include endpoints for creating, reading, updating, deleting records.
- [ ] User authentication
  - [ ] basic authentication system should be implemented, where users can auth with a username and password before using CRUD API
  - [ ] hardcoded values in DB are sufficient

Other Requirements:

- [ ] add proper error hanlding for HTTP requests, db interactions, auth
- [ ] test suite for `/run` endpoint
- [ ] (Optional) test suite for authentication
- [ ] add docker support

Extra points for:

- clean, readable code
- performance of `/run` endpoint
- good practises, SOLID, DDD
- tests (unit + integration)

### Examples

[3, 2, 5, 1, 5, 7, 2, 1] -> [1, 2, 5]
[5, 7, 7] -> [7]

## Running project

..

## Tests

...

### Unit tests

...

### Integration tests

...
