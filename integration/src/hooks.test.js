const axios = require('axios')
const { createMock, cleanExternalMocks } = require('external-mock')

const { DATABASE_URL } = require('./helpers/knexClient')
const databaseLifecycle = require('./helpers/databaseLifecycle')
const startAuthServer = require('./helpers/startAuthServer')

databaseLifecycle()

afterEach(() => cleanExternalMocks())

it('Call external service if POST_RESET_PASSWORD_URL is set', async () => {
  const externalServiceMock = jest.fn()

  createMock(5454).post('/reset-password').spy(externalServiceMock).reply(200)

  const { url, server } = await startAuthServer({
    JWT_SECRET: 'TEST_JWT_VALUE',
    DATABASE_URL,
    POST_RESET_PASSWORD_URL: 'http://localhost:5454/reset-password',
  })

  await axios.post(`${url}/register`, {
    email: 'lars@domain.com',
    password: 'lars',
    name: 'Lars Larsson',
  })

  await axios.post(`${url}/reset-password`, { email: 'lars@domain.com' })

  expect(externalServiceMock).toHaveBeenCalledWith({
    email: 'lars@domain.com',
    ticket: expect.any(String),
  })

  server.kill()
})

it('Call external service if POST_SET_PASSWORD_URL is set', async () => {
  const externalServiceMock = jest.fn()

  createMock(5454).post('/set-password').spy(externalServiceMock).reply(200)

  const { url, server } = await startAuthServer({
    JWT_SECRET: 'TEST_JWT_VALUE',
    DATABASE_URL,
    POST_SET_PASSWORD_URL: 'http://localhost:5454/set-password',
  })

  await axios.post(`${url}/register`, {
    email: 'lars@domain.com',
    password: 'lars',
    name: 'Lars Larsson',
  })

  const { data: resetResponse } = await axios.post(`${url}/reset-password`, {
    email: 'lars@domain.com',
  })

  const { status: setPasswordStatus } = await axios.post(`${url}/password`, {
    ticket: resetResponse.ticket,
    password: 'new-magic-password',
  })

  expect(setPasswordStatus).toEqual(200)
  expect(externalServiceMock).toHaveBeenCalledWith({
    email: 'lars@domain.com',
  })

  server.kill()
})

it('Call external service if POST_REGISTER_URL is set', async () => {
  const externalServiceMock = jest.fn()

  createMock(5454).post('/register').spy(externalServiceMock).reply(200)

  const { url, server } = await startAuthServer({
    JWT_SECRET: 'TEST_JWT_VALUE',
    DATABASE_URL,
    POST_REGISTER_URL: 'http://localhost:5454/register',
  })

  const { status: registerStatus } = await axios.post(`${url}/register`, {
    email: 'lars@domain.com',
    password: 'lars',
    name: 'Lars Larsson',
  })

  expect(registerStatus).toEqual(200)
  expect(externalServiceMock).toHaveBeenCalledWith({
    email: 'lars@domain.com',
    id: expect.any(String),
  })

  server.kill()
})
