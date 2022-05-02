const knexCleaner = require('knex-cleaner')

const { knexClient } = require('./knexClient')

const migrationTables = ['_sqlx_migrations']

const databaseLifecycle = () => {
  beforeEach(async () => knexCleaner.clean(knexClient, { mode: 'delete', ignoreTables: migrationTables }))
  afterAll(async () => {
    await knexClient.raw('DROP TABLE IF EXISTS user_metadata;')

    knexClient.destroy()
  })
}

module.exports = databaseLifecycle
