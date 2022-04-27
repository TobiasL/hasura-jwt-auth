# Hasura JWT auth

Auth server that return JWT tokens to be used by Hasura.

## Setup

TODO: Add a whole docker-compose YAML as an example.

TODO: Publish to Docker Hub.

## Environment variables

### Required

* `JWT_SECRET` same string has to be provided to Hasura.
* `DATABASE_URL` url to the Postgres instance.

### Optional

* `HOST` change from the default host 0.0.0.0.
* `PORT` change from the default port 80.
* `JWT_ORG_CUSTOM_CLAIM` string to indicate which table and column to fetch an organisation ID from.
For example `user_metadata.org_id` fetches the value and adds it to the claim `x-hasura-organisation-id`.
Need a foreign key that points to the uuid columns `user.id`.

## Hasura documentation to use JWT tokens

https://hasura.io/docs/latest/graphql/core/auth/authentication/jwt/

## License

MIT
