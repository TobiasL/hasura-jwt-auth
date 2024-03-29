const got = require('got')
const toughCookie = require('tough-cookie')

const { DATABASE_URL } = require('./helpers/knexClient')
const databaseLifecycle = require('./helpers/databaseLifecycle')
const startAuthServer = require('./helpers/startAuthServer')

databaseLifecycle()

it('Use a non existing refresh token', async () => {
  const { url, server } = await startAuthServer({
    JWT_SECRET: 'TEST_JWT_VALUE',
    DATABASE_URL,
  })

  await expect(async () => got.post(`${url}/refresh`)).rejects.toThrow('Response code 401 (Unauthorized)')

  server.kill()
})

it('Ensure that the refresh token is invalidated after usage', async () => {
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

  const cookies = await cookieJar.getCookies(url)
  const oldRefreshCookie = cookies.find((cookie) => cookie.toString().includes('refresh='))

  await got.post(`${url}/refresh`, { cookieJar })

  await cookieJar.setCookie(oldRefreshCookie, url)

  await expect(async () => {
    await got.post(`${url}/refresh`, { cookieJar })
  }).rejects.toThrow('Response code 401 (Unauthorized)')

  server.kill()
})
