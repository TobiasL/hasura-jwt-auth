const { spawn } = require('child_process')
const axios = require('axios')
const getPort = require('get-port')

const databaseLifecycle = require('./utils/databaseLifecycle')

databaseLifecycle()

let server

afterAll(() => server.kill())

const createServer = async (extendedEnv) => {
  const port = await getPort()

  const env = { ...process.env,  ...extendedEnv, PORT: port }
  server = spawn('cargo', ['run'], { env })

  const serverUrl = `http://127.0.0.1:${port}`

  while (true) {
    try {
      const { status }  = await axios(`${serverUrl}/health`)

      if (status === 200) return serverUrl
    } catch (err) {
      await new Promise((resolve) => setTimeout(resolve, 50))
    }
  }
}

it('Register a user and login', async () => {
  const serverUrl = await createServer({
    JWT_SECRET: '123',
    DATABASE_URL: 'postgres://postgres:postgrespassword@localhost:5432/auth_db',
  })

  const { status: registerStatus }  = await axios.post(`${serverUrl}/register`, {
    email: "lars@domain.com",
    password: "lars",
    name: "Lars Larsson",
  })

  expect(registerStatus).toEqual(200)

  const { data }  = await axios.post(`${serverUrl}/login`, {
    email: "lars@domain.com",
    password: "lars"
  })

  expect(data.refresh).toEqual(expect.any(String))
  expect(data.jwt_token).toEqual(expect.any(String))

  const [header, payload] = data.jwt_token.split('.')

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
    }
  })
})
