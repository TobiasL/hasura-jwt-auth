CREATE EXTENSION citext;
CREATE EXTENSION pgcrypto;

CREATE DOMAIN email AS citext
  CHECK ( value ~ '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$' );

CREATE FUNCTION public.set_current_timestamp_updated_at ()
  RETURNS TRIGGER
  LANGUAGE plpgsql
  AS $$
DECLARE
  _new record;
BEGIN
  _new := new;
  _new. "updated_at" = now();
  RETURN _new;
END;
$$;

CREATE TABLE public.users (
  id uuid DEFAULT public.gen_random_uuid() NOT NULL PRIMARY KEY,
  email email UNIQUE,
  password_hash text NOT NULL,
  name text NOT NULL,
  avatar_url text DEFAULT '' NOT NULL,
  default_role text DEFAULT 'user',
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  ticket uuid,
  ticket_expires_at timestamp with time zone
);

CREATE TABLE public.refresh_tokens (
  refresh_token uuid DEFAULT public.gen_random_uuid() NOT NULL PRIMARY KEY,
  created_at timestamp with time zone DEFAULT now() NOT NULL,
  expires_at timestamp with time zone NOT NULL,
  user_id uuid NOT NULL
);

ALTER TABLE public.refresh_tokens
  ADD CONSTRAINT fk_user FOREIGN KEY (user_id)
  REFERENCES public.users (id) ON UPDATE CASCADE ON DELETE CASCADE;

CREATE TRIGGER set_users_updated_at
  BEFORE UPDATE ON public.users
  FOR EACH ROW
  EXECUTE FUNCTION public.set_current_timestamp_updated_at ();
