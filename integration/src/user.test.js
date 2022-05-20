const axios = require('axios')
const jwt = require('jsonwebtoken')

const { DATABASE_URL } = require('./helpers/knexClient')
const databaseLifecycle = require('./helpers/databaseLifecycle')
const startAuthServer = require('./helpers/startAuthServer')

databaseLifecycle()

it('Register a user and login', async () => {
  const { url, server } = await startAuthServer({
    JWT_SECRET: 'TEST_JWT_VALUE',
    DATABASE_URL,
  })

  const { status: registerStatus } = await axios.post(`${url}/register`, {
    email: 'lars@domain.com',
    password: 'lars',
    name: 'Lars Larsson',
  })

  expect(registerStatus).toEqual(200)

  const { data: loginResponse } = await axios.post(`${url}/login`, {
    email: 'lars@domain.com',
    password: 'lars',
  })

  expect(loginResponse.refresh).toEqual(expect.any(String))
  expect(loginResponse.jwt_token).toEqual(expect.any(String))

  // Throws if the signature doesn't match.
  jwt.verify(loginResponse.jwt_token, 'TEST_JWT_VALUE')

  const [header, payload] = loginResponse.jwt_token.split('.')

  const decodedJwtHeader = Buffer.from(header, 'base64').toString('utf-8')
  const decodedJwtPayload = Buffer.from(payload, 'base64').toString('utf-8')

  expect(JSON.parse(decodedJwtHeader)).toEqual({
    alg: 'HS256',
    typ: 'JWT',
  })

  expect(JSON.parse(decodedJwtPayload)).toEqual({
    iat: expect.any(Number),
    exp: expect.any(Number),
    nbf: expect.any(Number),
    'https://hasura.io/jwt/claims': {
      'x-hasura-allowed-roles': ['user'],
      'x-hasura-default-role': 'user',
      'x-hasura-user-id': expect.any(String),
      'x-hasura-organisation-id': null,
    },
  })

  server.kill()
})
