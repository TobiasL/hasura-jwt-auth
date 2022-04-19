# Hasura JWT auth

Auth server that return JWT tokens to be used by Hasura.

## Setup

TODO: Add a whole docker-compose YAML as an example.
TODO: Publish to Docker Hub.

## Environment variables

### Required

* `JWT_SECRET` same string has to be provided to Hasura.
* `PG_URL` url to the Postgres instance.

### Optional

* `PORT` change from the default port 80.
* `NO_ORG` boolean to turn off the organisation JWT token signing.

## Hasura documentation to use JWT tokens

https://hasura.io/docs/latest/graphql/core/auth/authentication/jwt/

## License

MIT
