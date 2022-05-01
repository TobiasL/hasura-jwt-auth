const { spawn } = require('child_process')
const axios = require('axios')
const getPort = require('get-port')

const startAuthServer = async (extendedEnv) => {
  const port = await getPort()

  const env = { ...process.env,  ...extendedEnv, PORT: port }
  const server = spawn('cargo', ['run'], { env })

  const url = `http://127.0.0.1:${port}`

  while (true) {
    try {
      const { status }  = await axios(`${url}/health`)

      if (status === 200) {
        return { url, server }
      }
    } catch (err) {
      await new Promise((resolve) => setTimeout(resolve, 50))
    }
  }
}
module.exports = startAuthServer
