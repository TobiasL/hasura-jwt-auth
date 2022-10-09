# Hasura JWT auth

*Authentication server that return JWT tokens to be used by Hasura.*

## Setup

### Docker image

A Docker image for the architectures `linux/amd64` and `linux/arm64` are published to Docker Hub: https://hub.docker.com/r/tobiasli/hasura-jwt-auth

### Docker Compose

Example of a Docker Compose setup:

```yaml
version: '3.8'

services:
  postgres:
    image: postgres
    ports:
    - "5432:5432"
    environment:
      POSTGRES_DB: test_db
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgrespassword

  graphql-engine:
    image: hasura/graphql-engine
    ports:
    - "8080:8080"
    depends_on:
    - "postgres"
    - "auth"
    environment:
      HASURA_GRAPHQL_DATABASE_URL: postgres://postgres:postgrespassword@postgres:5432/test_db
      HASURA_GRAPHQL_ENABLE_CONSOLE: "true"
      HASURA_GRAPHQL_ADMIN_SECRET: PLACEHOLDER_ADMIN_SECRET
      HASURA_GRAPHQL_JWT_SECRET: >-
        { "type": "HS256", "header": {"type": "Cookie", "name": "jwt" }, "key": "PLACEHOLDER_JWT_SECRET_KEY_TO_REPLACE" }

  auth:
    image: tobiasli/hasura-jwt-auth
    ports:
    - "3030:80"
    environment:
      DATABASE_URL: postgres://postgres:postgrespassword@postgres:5432/test_db
      JWT_SECRET: PLACEHOLDER_JWT_SECRET_KEY_TO_REPLACE
```

## Environment variables

### Required

* `JWT_SECRET` same string has to be provided to Hasura.
* `DATABASE_URL` url to the Postgres instance.

### Optional

* `HOST` change from the default host 0.0.0.0.
* `PORT` change from the default port 80.
* `DATABASE_CONNECTIONS` number of connections for the Postgres connection pool.
* `JWT_EXPIRES_IN_MINUTES` how many minutes until a JWT token expires.
* `REFRESH_EXPIRES_IN_DAYS` how many days until a refresh token expires.
* `JWT_ORG_CUSTOM_CLAIM` string to indicate which table and column to fetch an organisation ID from.
For example `user_metadata.org_id` fetches the value and adds it to the claim `X-Hasura-Org-Id`.
Need a foreign key that points to the uuid columns `user.id`.
* `POST_REGISTER_URL` URL that will receive the email and id for newly created user.
* `POST_RESET_PASSWORD_URL` URL that will receive the email and ticket for the password that was reset.
* `POST_SET_PASSWORD_URL` URL that will receive the email for the user that set a new password.

## API documentation

### GET `/livez`

Liveness probe.

### GET `/readyz`

Readiness probe.

### POST `/register`

Register an account and get the JWT token and refresh token.

### Request
```json
{
  "name": "string",
  "email": "string",
  "password": "string"
}
```

### Response

See [cookie responses](#cookie-responses).

### POST `/login`

Login with email and password to get the JWT token and refresh token.

### Request
```json
{
  "email": "string",
  "password": "string"
}
```

### Response

See [cookie responses](#cookie-responses).

### POST `/refresh`

Call with the refresh token to get a new JWT token.

### Request

The `refresh` cookie is used.

### Response

See [cookie responses](#cookie-responses).

### POST `/reset-password`

Gets a ticket to use when setting the new password through `/password`.

### Request
```json
{
  "email": "string",
  "password": "string"
}
```

### Response
```json
{
  "ticket": "string"
}
```

### POST `/password`

Sets the users new password with the ticket received from `/reset-password`.

### Request
```json
{
  "ticket": "string",
  "password": "string"
}
```

### POST `/logout`

Call to clear all cookies.

### Response

See [cookie responses](#cookie-responses).

## Cookie responses

The `jwt_expiry` and `refresh_expiry` cookies exist to be able to be accessed in JavaScript
to check how long the `jwt` and `refresh` cookies are valid.
The `jwt` and `refresh` tokens are HttpOnly and can't be accessed through JavaScript.

### Register, login and refresh `set-cookie` header

  * `jwt={JWT_VALUE}; Max-Age={JWT_EXPIRY}; Path=/; SameSite=strict; HttpOnly;`
  * `refresh={REFRESH_TOKEN}; Max-Age={REFRESH_EXPIRY}; Path=/; SameSite=strict; HttpOnly;`
  * `jwt_expiry={JWT_EXPIRY}; Max-Age={JWT_EXPIRY}; Path=/; SameSite=strict;`
  * `refresh_expiry={REFRESH_EXPIRY}; Max-Age={REFRESH_EXPIRY}; Path=/; SameSite=strict;`

### Logout `set-cookie` header

  * `jwt=; Max-Age=-1; Path=/;`
  * `refresh=; Max-Age=-1; Path=/;`
  * `jwt_expiry=; Max-Age=-1; Path=/;`
  * `refresh_expiry=; Max-Age=-1; Path=/;`

## Hasura documentation on how to use JWT tokens

https://hasura.io/docs/latest/graphql/core/auth/authentication/jwt/

## License

MIT
