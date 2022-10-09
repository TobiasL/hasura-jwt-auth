const got = require('got')
const jwt = require('jsonwebtoken')
const toughCookie = require('tough-cookie')

const { knexClient, DATABASE_URL } = require('./helpers/knexClient')
const databaseLifecycle = require('./helpers/databaseLifecycle')
const startAuthServer = require('./helpers/startAuthServer')
const getCookieValues = require('./helpers/getCookieValues')

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

  const cookieJar = new toughCookie.CookieJar()

  const { statusCode: registerStatus } = await got.post(`${url}/register`, {
    cookieJar,
    json: {
      email: 'lars@domain.com',
      password: 'lars',
      name: 'Lars Larsson',
    },
  })

  expect(registerStatus).toEqual(200)

  const { id } = await knexClient('users').select('id').where('email', 'lars@domain.com').first()
  await knexClient('user_metadata').insert({ user_id: id, org_id: ORG_ID })

  await got.post(`${url}/login`, {
    cookieJar,
    json: {
      email: 'lars@domain.com',
      password: 'lars',
    },
  })

  const cookieString = await cookieJar.getCookieString(url)

  const {
    refresh, refreshExpiry, jwtToken, jwtExpiry,
  } = getCookieValues(cookieString)

  expect(refresh).toEqual(expect.any(String))
  expect(refresh).toHaveLength(36)
  expect(jwtToken).toEqual(expect.any(String))
  expect(jwtExpiry).toBeGreaterThan(Date.now())
  expect(refreshExpiry).toBeGreaterThan(Date.now())

  // Throws if the signature doesn't match.
  jwt.verify(jwtToken, 'TEST_JWT_VALUE')

  const [header, payload] = jwtToken.split('.')

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
      'x-hasura-org-id': ORG_ID,
    },
  })

  server.kill()
})

it('Register a user connected to an organisation and use the refresh token', async () => {
  await knexClient.raw(USER_ORG_MIGRATION)

  const { url, server } = await startAuthServer({
    JWT_SECRET: 'TEST_JWT_VALUE',
    DATABASE_URL,
    JWT_ORG_CUSTOM_CLAIM: 'user_metadata.org_id',
  })

  const cookieJar = new toughCookie.CookieJar()

  const { statusCode: registerStatus } = await got.post(`${url}/register`, {
    cookieJar,
    json: {
      email: 'lars@domain.com',
      password: 'lars',
      name: 'Lars Larsson',
    },
  })

  expect(registerStatus).toEqual(200)

  const { id } = await knexClient('users').select('id').where('email', 'lars@domain.com').first()
  await knexClient('user_metadata').insert({ user_id: id, org_id: ORG_ID })

  const { statusCode: refreshStatus } = await got.post(`${url}/refresh`, { cookieJar })

  expect(refreshStatus).toBe(200)

  server.kill()
})
