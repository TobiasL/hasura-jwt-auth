const knexCleaner = require('knex-cleaner')

const knex = require('./knexClient')

const migrationTables = ['_sqlx_migrations']

const databaseLifecycle = () => {
  beforeEach(async () => knexCleaner.clean(knex, { mode: 'delete', ignoreTables: migrationTables }))
  afterAll(() => knex.destroy())
}

module.exports = databaseLifecycle
