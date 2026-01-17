use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LoginView {
    pub user_id: String,
    pub username: String,
    pub email: String,
}
