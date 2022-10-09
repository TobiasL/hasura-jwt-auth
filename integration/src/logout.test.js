const got = require('got')
const toughCookie = require('tough-cookie')

const { DATABASE_URL } = require('./helpers/knexClient')
const databaseLifecycle = require('./helpers/databaseLifecycle')
const startAuthServer = require('./helpers/startAuthServer')

databaseLifecycle()

it('Logout and make sure that the cookies are removed', async () => {
  const { url, server } = await startAuthServer({
    JWT_SECRET: 'TEST_JWT_VALUE',
    DATABASE_URL,
  })

  const cookieJar = new toughCookie.CookieJar()

  await got.post(`${url}/register`, {
    cookieJar,
    json: {
      email: 'lars@domain.com',
      password: 'lars',
      name: 'Lars Larsson',
    },
  })

  await got.post(`${url}/logout`, { cookieJar })

  await expect(async () => {
    await got.post(`${url}/refresh`, { cookieJar })
  }).rejects.toThrow('Response code 401 (Unauthorized)')

  server.kill()
})
