const axios = require('axios')

const { knexClient, DATABASE_URL } = require('./helpers/knexClient')
const databaseLifecycle = require('./helpers/databaseLifecycle')
const startAuthServer = require('./helpers/startAuthServer')

databaseLifecycle()

it('Reset password and login with the new password', async () => {
  const { url, server } = await startAuthServer({
    JWT_SECRET: 'TEST_JWT_VALUE',
    DATABASE_URL,
  })

  const { status: registerStatus }  = await axios.post(`${url}/register`, {
    email: "lars@domain.com",
    password: "lars",
    name: "Lars Larsson",
  })

  expect(registerStatus).toEqual(200)

  const { status: resetStatus }  = await axios.post(`${url}/reset-password`, {
    email: "lars@domain.com",
  })

  expect(resetStatus).toEqual(200)

  const { ticket: ticketSentToEmail } = await knexClient('users').select('ticket')
    .where('email', "lars@domain.com").first()

  const { status: setPasswordStatus } = await axios.post(`${url}/password`, {
    ticket: ticketSentToEmail,
    password: "new-magic-password",
  })

  expect(setPasswordStatus).toEqual(200)

  const { status: loginStatus }  = await axios.post(`${url}/login`, {
    email: "lars@domain.com",
    password: "new-magic-password"
  })

  expect(loginStatus).toEqual(200)

  server.kill()
})

it('Fail reset password for a user that doesn\'t exist', async () => {
  const { url, server } = await startAuthServer({
    JWT_SECRET: 'TEST_JWT_VALUE',
    DATABASE_URL,
  })

  const { status }  = await axios.post(`${url}/reset-password`, {
    email: "fake@user.com",
  }, { validateStatus: (status) => status < 500 })

  expect(status).toEqual(401)

  server.kill()
})

it('Fail set password with an expired ticket', async () => {
  const { url, server } = await startAuthServer({
    JWT_SECRET: 'TEST_JWT_VALUE',
    DATABASE_URL,
  })

  const EXPIRED_TICKET = 'be297282-fdb7-48ec-a737-ee4cb893fa0d'

  const { status: setPasswordStatus } = await axios.post(`${url}/password`, {
    ticket: EXPIRED_TICKET,
    password: "new-magic-password",
  }, { validateStatus: (status) => status < 500 })

  expect(setPasswordStatus).toEqual(401)

  server.kill()
})
