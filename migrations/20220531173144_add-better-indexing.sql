ALTER TABLE users DROP COLUMN avatar_url;

CREATE INDEX ON refresh_tokens(refresh_token);
CREATE INDEX ON refresh_tokens(expires_at);

CREATE INDEX ON users(ticket);
CREATE INDEX ON users(ticket_expires_at);
