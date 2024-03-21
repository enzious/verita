CREATE TABLE realm (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE "user" (
  id BIGSERIAL PRIMARY KEY,
  realm_id INTEGER REFERENCES realm (id),
  name TEXT NOT NULL
);

CREATE TABLE user_credential (
  user_id BIGINT REFERENCES "user" (id),
  credential_config_id INTEGER REFERENCES credential_config (id),
  content TEXT
);

CREATE TABLE credential_config (
  id SERIAL PRIMARY KEY,
  hash TEXT NOT NULL,
  salt TEXT
);
