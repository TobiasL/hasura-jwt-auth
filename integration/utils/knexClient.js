const knex = require('knex')

const knexClient = knex({
  client: 'pg',
  connection: 'postgres://postgres:postgrespassword@localhost:5432/auth_db'
})

module.exports = knexClient
