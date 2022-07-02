const express = require('express')
const getPort = require('get-port')

const startMockServer = async (mockUrl, mockFunction) => {
  const port = await getPort()

  const app = express()

  app.use(express.json())
  app.post(mockUrl, (req, res) => {
    mockFunction(req.body)

    return res.sendStatus(200)
  })

  const server = app.listen(port)

  return { server, url: `http://localhost:${port}${mockUrl}` }
}

module.exports = startMockServer
