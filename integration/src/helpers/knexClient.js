const knex = require('knex')
const knexCleaner = require('knex-cleaner')

const DATABASE_URL = 'postgres://postgres:postgrespassword@localhost:5432/auth_db'

const knexClient = knex({
  client: 'pg',
  connection: DATABASE_URL,
})

module.exports = {
  knexClient,
  DATABASE_URL,
}
