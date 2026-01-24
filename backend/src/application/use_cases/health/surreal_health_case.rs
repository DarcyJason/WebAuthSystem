use crate::application::errors::ApplicationError;
use crate::domain::health::repositories::db::HealthRepository;

#[derive(Debug, Clone)]
pub struct SurrealHealthCase<SH>
where
    SH: HealthRepository,
{
    surreal_health_repo: SH,
}

impl<SH> SurrealHealthCase<SH>
where
    SH: HealthRepository,
{
    pub fn new(surreal_health_repo: SH) -> Self {
        SurrealHealthCase {
            surreal_health_repo,
        }
    }
    pub async fn execute(&self) -> Result<(), ApplicationError> {
        self.surreal_health_repo
            .check()
            .await
            .map_err(|_| ApplicationError::InfrastructureError)?;
        Ok(())
    }
}
