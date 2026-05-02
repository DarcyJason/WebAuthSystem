use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AccessToken(String);

impl AccessToken {
    pub fn new(access_token_value: impl Into<String>) -> Self {
        Self(access_token_value.into())
    }
    pub fn value(&self) -> String {
        self.0.to_owned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    sub: String,
    ver: String, 
    iat: usize,
    exp: usize,
}

impl AccessTokenClaims {
    pub fn new(sub: impl Into<String>, ver: impl Into<String>, iat: usize, exp: usize) -> Self {
        AccessTokenClaims {
            sub: sub.into(),
            ver: ver.into(),
            iat,
            exp,
        }
    }
    pub fn sub(&self) -> &str {
        &self.sub
    }
    pub fn ver(&self) -> &str {
        &self.ver
    }
    pub fn iat(&self) -> usize {
        self.iat
    }
    pub fn exp(&self) -> usize {
        self.exp
    }
}
