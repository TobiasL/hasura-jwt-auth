const startAuthServer = require('./startAuthServer')

it('Start server without JWT_SECRET env exits with code 101', async () => {
  const env = {}

  await expect(async () => await startAuthServer(env)).rejects.toBe(101)
})

it('Start server without DATABASE_URL env exits with code 101', async () => {
  const env = { JWT_SECRET: 'TEST_JWT_VALUE' }

  await expect(async () => await startAuthServer(env)).rejects.toBe(101)
})

it('Start server with JWT_ORG_CUSTOM_CLAIM env for a table that doesn\'t exist. Exits with code 101', async () => {
  const env = { JWT_ORG_CUSTOM_CLAIM: 'fake_table.fake_column' }

  await expect(async () => await startAuthServer(env)).rejects.toBe(101)
})
