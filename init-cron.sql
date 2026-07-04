\c postgres

CREATE EXTENSION IF NOT EXISTS pg_cron;

SELECT cron.schedule_in_database(
    'cleanup-expired-sessions',
    '0 3 * * *',
    $$DELETE FROM sessions WHERE expires_at <= NOW();$$,
    'example_db'
);