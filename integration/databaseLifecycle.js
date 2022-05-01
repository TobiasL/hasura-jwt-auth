const knex = require('knex')
const knexCleaner = require('knex-cleaner')

const knexClient = knex({
  client: 'pg',
  connection: 'postgres://postgres:postgrespassword@localhost:5432/auth_db'
})

const migrationTables = ['_sqlx_migrations']

const databaseLifecycle = () => {
  beforeEach(async () => knexCleaner.clean(knexClient, { mode: 'delete', ignoreTables: migrationTables }))
  afterAll(() => knexClient.destroy())
}

module.exports = databaseLifecycle
