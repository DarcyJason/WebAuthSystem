use crate::application::commands::auth::login::LoginCommand;
use crate::application::errors::ApplicationError;
use crate::application::queries::auth::login::LoginView;
use crate::domain::auth::repositories::AuthRepository;
use crate::domain::user::value_objects::{Email, Username};

pub struct LoginCase<R>
where
    R: AuthRepository,
{
    auth_repo: R,
}

impl<R> LoginCase<R>
where
    R: AuthRepository,
{
    pub fn new(auth_repo: R) -> Self {
        LoginCase { auth_repo }
    }
    pub async fn execute(&self, cmd: LoginCommand) -> Result<(&str, LoginView), ApplicationError> {
        let (username, email) = if cmd.username_or_email.contains('@') {
            (None, Some(Email::new(cmd.username_or_email)?))
        } else {
            (Some(Username::new(cmd.username_or_email)?), None)
        };
        let user = self
            .auth_repo
            .login(username, email, cmd.password.trim().to_string())
            .await?;
        let data = LoginView {
            user_id: user.id().to_string(),
            username: user.username().to_string(),
            email: user.email().to_string(),
        };
        Ok(("login success", data))
    }
}
