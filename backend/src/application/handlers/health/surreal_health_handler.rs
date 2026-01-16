use crate::application::errors::ApplicationError;
use crate::domain::health::repositories::HealthRepository;

#[derive(Debug, Clone)]
pub struct SurrealHealthHandler<R>
where
    R: HealthRepository
{
    surreal_health_repo: R
}

impl<R> SurrealHealthHandler<R>
where
    R: HealthRepository
{
    pub fn new(surreal_health_repo: R) -> Self {
        SurrealHealthHandler { surreal_health_repo }
    }
    pub async fn handle(&self) -> Result<(), ApplicationError> {
        let handle_result = self
            .surreal_health_repo
            .check()
            .await
            .map_err(|e| ApplicationError::RepoitoryUnavailable(e.to_string()))?;
        Ok(handle_result)
    }
}
