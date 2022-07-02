const { spawn } = require('child_process')
const axios = require('axios')
const getPort = require('get-port')

// eslint-disable-next-line no-async-promise-executor, consistent-return
const startAuthServer = async (extendedEnv) => new Promise(async (resolve, reject) => {
  const port = await getPort()

  const env = { ...process.env, ...extendedEnv, PORT: port }
  const server = spawn('cargo', ['run'], { env })
    .on('exit', (code) => reject(code))

  if (process.env.DEBUG) {
    // eslint-disable-next-line no-console
    server.stdout.on('data', (data) => console.log(`stdout: ${data}`))
    // eslint-disable-next-line no-console
    server.stderr.on('data', (data) => console.error(`stderr: ${data}`))
  }

  const url = `http://127.0.0.1:${port}`

  while (!server.exitCode) {
    try {
      // eslint-disable-next-line no-await-in-loop, no-promise-executor-return
      const { status } = await axios(`${url}/readyz`)

      if (status === 200) {
        // eslint-disable-next-line no-promise-executor-return
        return resolve({ url, server })
      }
    } catch (err) {
      // eslint-disable-next-line no-await-in-loop, no-promise-executor-return
      await new Promise((resolveWait) => setTimeout(resolveWait, 50))
    }
  }
})

module.exports = startAuthServer
