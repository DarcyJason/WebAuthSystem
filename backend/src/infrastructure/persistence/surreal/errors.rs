pub enum SurrealDBError {
    SendRequestError,
    ConnectionError,
    ExecuteQueryError,
    ParseRecordToUserError,
    RepositoryError(String),
}
