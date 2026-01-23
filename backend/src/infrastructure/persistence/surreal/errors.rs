pub enum SurrealDBError {
    RequestHealthEndpointError,
    ConnectionError,
    ExecuteQueryError,
    ParseRecordToUserError,
    RepositoryError(String),
}
