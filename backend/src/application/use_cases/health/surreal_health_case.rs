use crate::application::errors::ApplicationError;
use crate::domain::health::repositories::HealthRepository;

#[derive(Debug, Clone)]
pub struct SurrealHealthCase<R>
where
    R: HealthRepository,
{
    surreal_health_repo: R,
}

impl<R> SurrealHealthCase<R>
where
    R: HealthRepository,
{
    pub fn new(surreal_health_repo: R) -> Self {
        SurrealHealthCase {
            surreal_health_repo,
        }
    }
    pub async fn execute(&self) -> Result<(&str, ()), ApplicationError> {
        self.surreal_health_repo
            .check()
            .await
            .map_err(|_| ApplicationError::InfrastructureError)?;
        Ok(("SurrealDB is healthy", ()))
    }
}
