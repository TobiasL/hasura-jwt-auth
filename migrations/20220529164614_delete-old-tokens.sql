CREATE FUNCTION delete_old_refresh_tokens() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
  DELETE FROM refresh_tokens WHERE expires_at < CURRENT_TIMESTAMP;
  RETURN NULL;
END;
$$;

CREATE TRIGGER trigger_delete_old_refresh_tokens
    AFTER INSERT ON refresh_tokens
    EXECUTE PROCEDURE delete_old_refresh_tokens();
