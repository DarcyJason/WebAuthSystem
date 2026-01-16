DEFINE TABLE OVERWRITE user SCHEMALESS;

# DEFINE FIELD OVERWRITE field ON user;
DEFINE FIELD OVERWRITE id ON user TYPE record DEFAULT rand::uuid::v4();
DEFINE FIELD OVERWRITE username ON user TYPE string;
DEFINE FIELD OVERWRITE email ON user TYPE string;
DEFINE FIELD OVERWRITE hash_password ON user TYPE string;
DEFINE FIELD OVERWRITE created_at ON user TYPE datetime DEFAULT time::now();
DEFINE FIELD OVERWRITE updated_at ON user TYPE datetime DEFAULT time::now();
