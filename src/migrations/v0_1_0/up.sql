CREATE SCHEMA verita;

CREATE TABLE verita.realm (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  operator BOOLEAN NOT NULL,
  created TIMESTAMPTZ NOT NULL,
  updated TIMESTAMPTZ NOT NULL,
  UNIQUE (name)
);

CREATE TABLE verita."user" (
  id BIGSERIAL PRIMARY KEY,
  realm_id INTEGER NOT NULL
    REFERENCES realm (id),
  username TEXT NOT NULL,
  email TEXT,
  email_verified BOOLEAN NOT NULL,
  operator BOOLEAN NOT NULL,
  created TIMESTAMPTZ NOT NULL,
  updated TIMESTAMPTZ NOT NULL,
  UNIQUE (realm_id, username),
  UNIQUE (realm_id, email)
);

CREATE TABLE verita.credential_config (
  id SERIAL PRIMARY KEY,
  realm_id INTEGER NOT NULL
    REFERENCES realm (id),
  hash BYTEA NOT NULL,
  salt BYTEA,
  iterations INTEGER,
  created TIMESTAMPTZ NOT NULL,
  updated TIMESTAMPTZ NOT NULL
);

CREATE TABLE verita.user_credential (
  user_id BIGINT PRIMARY KEY 
    REFERENCES "user" (id),
  credential_config_id INTEGER NOT NULL
    REFERENCES credential_config (id),
  content BYTEA NOT NULL,
  temporary BOOLEAN NOT NULL,
  created TIMESTAMPTZ NOT NULL,
  updated TIMESTAMPTZ NOT NULL
);
