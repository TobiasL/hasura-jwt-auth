const axios = require('axios')

const { DATABASE_URL } = require('./helpers/knexClient')
const databaseLifecycle = require('./helpers/databaseLifecycle')
const startAuthServer = require('./helpers/startAuthServer')
const startMockServer = require('./helpers/startMockServer')

databaseLifecycle()

it('Call external service if POST_RESET_PASSWORD_URL is set', async () => {
  const externalServiceMock = jest.fn()
  const { url: mockUrl, server: mockServer } = await startMockServer('/reset-password', externalServiceMock)

  const { url, server } = await startAuthServer({
    JWT_SECRET: 'TEST_JWT_VALUE',
    DATABASE_URL,
    POST_RESET_PASSWORD_URL: mockUrl,
  })

  await axios.post(`${url}/register`, {
    email: 'lars@domain.com',
    password: 'lars',
    name: 'Lars Larsson',
  })

  const { status: resetStatus } = await axios.post(`${url}/reset-password`, {
    email: 'lars@domain.com',
  })

  expect(resetStatus).toEqual(200)
  expect(externalServiceMock).toHaveBeenCalledWith({
    email: 'lars@domain.com',
    ticket: expect.any(String),
  })

  server.kill()
  mockServer.close()
})
