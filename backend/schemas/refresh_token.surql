DEFINE TABLE OVERWRITE refresh_token SCHEMALESS;

DEFINE FIELD OVERWRITE user ON TABLE refresh_token TYPE record<user> ASSERT $value != NONE;
DEFINE FIELD OVERWRITE token_hash ON TABLE refresh_token TYPE string ASSERT $value != NONE;
DEFINE FIELD OVERWRITE expires_at ON TABLE refresh_token TYPE datetime ASSERT $value != NONE;
DEFINE FIELD OVERWRITE created_at ON refresh_token TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON refresh_token TYPE datetime DEFAULT time::now();

DEFINE INDEX token_hash_idx ON TABLE refresh_token COLUMNS token_hash;
DEFINE INDEX user_idx ON TABLE refresh_token COLUMNS user;
