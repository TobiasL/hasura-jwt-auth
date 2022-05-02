const axios = require('axios')

const { knexClient, DATABASE_URL } = require('./knexClient')

const databaseLifecycle = require('./databaseLifecycle')
const startAuthServer = require('./startAuthServer')

databaseLifecycle()

const ORG_ID = 'b12fdeae-e998-4772-85a4-35f0cb8575d0'
const USER_ORG_MIGRATION = `
  CREATE TABLE IF NOT EXISTS user_metadata(
    user_id uuid NOT NULL,
    org_id uuid NOT NULL,
    UNIQUE (user_id, org_id)
  );
`

it('Register a user connected to an organisation and login', async () => {
  await knexClient.raw(USER_ORG_MIGRATION)

  const { url, server } = await startAuthServer({
    JWT_SECRET: 'TEST_JWT_VALUE',
    DATABASE_URL,
    JWT_ORG_CUSTOM_CLAIM: 'user_metadata.org_id',
  })

  const { status: registerStatus }  = await axios.post(`${url}/register`, {
    email: "lars@domain.com",
    password: "lars",
    name: "Lars Larsson",
  })

  expect(registerStatus).toEqual(200)

  const { id } = await knexClient('users').select('id').where('email', 'lars@domain.com').first()
  await knexClient('user_metadata').insert({ user_id: id, org_id: ORG_ID })

  const { data: loginResponse }  = await axios.post(`${url}/login`, {
    email: "lars@domain.com",
    password: "lars"
  })

  expect(loginResponse.refresh).toEqual(expect.any(String))
  expect(loginResponse.jwt_token).toEqual(expect.any(String))

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
      'x-hasura-organisation-id': ORG_ID,
    }
  })

  server.kill()
})

it('Register a user connected to an organisation and refresh', async () => {
  await knexClient.raw(USER_ORG_MIGRATION)

  const { url, server } = await startAuthServer({
    JWT_SECRET: 'TEST_JWT_VALUE',
    DATABASE_URL,
    JWT_ORG_CUSTOM_CLAIM: 'user_metadata.org_id',
  })

  const { data: registerResponse, status: registerStatus }  = await axios.post(`${url}/register`, {
    email: "lars@domain.com",
    password: "lars",
    name: "Lars Larsson",
  })

  expect(registerStatus).toEqual(200)

  const { id } = await knexClient('users').select('id').where('email', 'lars@domain.com').first()
  await knexClient('user_metadata').insert({ user_id: id, org_id: ORG_ID })

  const { data: refreshResponse }  = await axios.post(`${url}/refresh`, {
    refresh: registerResponse.refresh,
  })

  expect(refreshResponse.refresh).toEqual(expect.any(String))
  expect(refreshResponse.jwt_token).toEqual(expect.any(String))

  const [header, payload] = refreshResponse.jwt_token.split('.')

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
      'x-hasura-organisation-id': ORG_ID,
    }
  })

  server.kill()
})
