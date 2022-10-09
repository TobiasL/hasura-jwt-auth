const got = require('got')

const { DATABASE_URL } = require('./helpers/knexClient')
const databaseLifecycle = require('./helpers/databaseLifecycle')
const startAuthServer = require('./helpers/startAuthServer')

databaseLifecycle()

it('Reset password and login with the new password', async () => {
  const { url, server } = await startAuthServer({
    JWT_SECRET: 'TEST_JWT_VALUE',
    DATABASE_URL,
  })

  const { statusCode: registerStatus } = await got.post(`${url}/register`, {
    json: {
      email: 'lars@domain.com',
      password: 'lars',
      name: 'Lars Larsson',
    },
  })

  expect(registerStatus).toEqual(200)

  const resetResponse = await got.post(`${url}/reset-password`, {
    json: {
      email: 'lars@domain.com',
    },
  }).json()

  expect(resetResponse.ticket).toEqual(expect.any(String))

  const { statusCode: setPasswordStatus } = await got.post(`${url}/password`, {
    json: {
      ticket: resetResponse.ticket,
      password: 'new-magic-password',
    },
  })

  expect(setPasswordStatus).toEqual(200)

  const { statusCode: loginStatus } = await got.post(`${url}/login`, {
    json: {
      email: 'lars@domain.com',
      password: 'new-magic-password',
    },
  })

  expect(loginStatus).toEqual(200)

  server.kill()
})

it('Fail reset password for a user that doesn\'t exist', async () => {
  const { url, server } = await startAuthServer({
    JWT_SECRET: 'TEST_JWT_VALUE',
    DATABASE_URL,
  })

  await expect(async () => {
    await got.post(`${url}/reset-password`, {
      json: {
        email: 'fake@user.com',
      },
    })
  }).rejects.toThrow('Response code 401 (Unauthorized)')

  server.kill()
})

it('Fail set password with an expired ticket', async () => {
  const { url, server } = await startAuthServer({
    JWT_SECRET: 'TEST_JWT_VALUE',
    DATABASE_URL,
  })

  const EXPIRED_TICKET = 'be297282-fdb7-48ec-a737-ee4cb893fa0d'

  await expect(async () => {
    await got.post(`${url}/password`, {
      json: {
        ticket: EXPIRED_TICKET,
        password: 'new-magic-password',
      },
    })
  }).rejects.toThrow('Response code 401 (Unauthorized)')

  server.kill()
})
