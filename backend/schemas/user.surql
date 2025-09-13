DEFINE TABLE OVERWRITE user SCHEMALESS;

# DEFINE FIELD OVERWRITE field ON user;
DEFINE FIELD OVERWRITE name ON user TYPE string ASSERT $value != NONE;
DEFINE FIELD OVERWRITE email ON user TYPE string ASSERT string::is::email($value);
DEFINE FIELD OVERWRITE password ON user TYPE string ASSERT $value != NONE;
DEFINE FIELD OVERWRITE created_at ON user TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON user TYPE datetime DEFAULT time::now();
