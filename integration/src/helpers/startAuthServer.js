const { spawn } = require('child_process')
const axios = require('axios')
const getPort = require('get-port')

const startAuthServer = async (extendedEnv) => new Promise(async (resolve, reject) => {
  const port = await getPort()

  const env = { ...process.env, ...extendedEnv, PORT: port }
  const server = spawn('cargo', ['run'], { env })
    .on('exit', (code) => reject(code))

  const url = `http://127.0.0.1:${port}`

  while (!server.exitCode) {
    try {
      const { status } = await axios(`${url}/readyz`)

      if (status === 200) {
        return resolve({ url, server })
      }
    } catch (err) {
      await new Promise((resolveWait) => setTimeout(resolveWait, 50))
    }
  }
})

module.exports = startAuthServer
