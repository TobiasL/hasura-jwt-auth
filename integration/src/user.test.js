const got = require('got')
const jwt = require('jsonwebtoken')
const toughCookie = require('tough-cookie')

const { DATABASE_URL } = require('./helpers/knexClient')
const databaseLifecycle = require('./helpers/databaseLifecycle')
const startAuthServer = require('./helpers/startAuthServer')
const getCookieValues = require('./helpers/getCookieValues')

databaseLifecycle()

it('Register a user and login', async () => {
  const { url, server } = await startAuthServer({
    JWT_SECRET: 'TEST_JWT_VALUE',
    DATABASE_URL,
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
    },
  })

  server.kill()
})

it('Register a user and use the refresh token', async () => {
  const { url, server } = await startAuthServer({
    JWT_SECRET: 'TEST_JWT_VALUE',
    DATABASE_URL,
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

  const { statusCode: refreshStatus } = await got.post(`${url}/refresh`, { cookieJar })

  expect(refreshStatus).toEqual(200)

  server.kill()
})
