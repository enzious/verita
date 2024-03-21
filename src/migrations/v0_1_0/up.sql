CREATE SCHEMA verita;

CREATE TABLE verita.realm (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  created TIMESTAMPTZ,
  updated TIMESTAMPTZ,
  UNIQUE (name)
);

CREATE TABLE verita."user" (
  id BIGSERIAL PRIMARY KEY,
  realm_id INTEGER
    REFERENCES realm (id),
  username TEXT NOT NULL,
  email TEXT,
  created TIMESTAMPTZ,
  updated TIMESTAMPTZ,
  UNIQUE (realm_id, username)
);

CREATE TABLE verita.user_credential (
  user_id BIGINT PRIMARY KEY 
    REFERENCES "user" (id),
  credential_config_id INTEGER
    REFERENCES credential_config (id),
  content TEXT,
  created TIMESTAMPTZ,
  updated TIMESTAMPTZ
);

CREATE TABLE verita.credential_config (
  id SERIAL PRIMARY KEY,
  hash TEXT NOT NULL,
  salt TEXT,
  created TIMESTAMPTZ,
  updated TIMESTAMPTZ
);
