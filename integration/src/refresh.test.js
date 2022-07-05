const axios = require('axios')

const { DATABASE_URL } = require('./helpers/knexClient')
const databaseLifecycle = require('./helpers/databaseLifecycle')
const startAuthServer = require('./helpers/startAuthServer')

databaseLifecycle()

it('Use a non existing refresh token', async () => {
  const { url, server } = await startAuthServer({
    JWT_SECRET: 'TEST_JWT_VALUE',
    DATABASE_URL,
  })

  const { status: refreshStatus } = await axios.post(`${url}/refresh`, {
    refresh: 'b12fdeae-e998-4772-85a4-35f0cb8575d0',
  }, { validateStatus: (status) => status < 500 })

  expect(refreshStatus).toEqual(401)

  server.kill()
})

it('Ensure that the refresh token is invalidated after usage', async () => {
  const { url, server } = await startAuthServer({
    JWT_SECRET: 'TEST_JWT_VALUE',
    DATABASE_URL,
  })

  const { data: registerResponse } = await axios.post(`${url}/register`, {
    email: 'lars@domain.com',
    password: 'lars',
    name: 'Lars Larsson',
  })

  await axios.post(`${url}/refresh`, {
    refresh: registerResponse.refresh,
  })

  const { status: refreshStatus } = await axios.post(`${url}/refresh`, {
    refresh: registerResponse.refresh,
  }, { validateStatus: (status) => status < 500 })

  expect(refreshStatus).toEqual(401)

  server.kill()
})
