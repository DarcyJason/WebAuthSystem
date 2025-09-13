DEFINE TABLE OVERWRITE user SCHEMALESS;

# DEFINE FIELD OVERWRITE field ON user;

DEFINE EVENT OVERWRITE user ON TABLE user WHEN $event == "UPDATE" THEN (
    UPDATE user SET updated_at = time::now() WHERE id = $after.id
);
