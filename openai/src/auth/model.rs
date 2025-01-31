use std::fmt::{Display, Formatter};

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AuthStrategy {
    Apple,
    Web,
    Platform,
}

impl Default for AuthStrategy {
    fn default() -> Self {
        AuthStrategy::Web
    }
}

impl Display for AuthStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{self:?}")
    }
}

#[derive(Deserialize, Builder)]
pub struct AuthAccount {
    pub username: String,
    pub password: String,
    #[builder(setter(into, strip_option), default)]
    pub mfa: Option<String>,
    #[serde(default)]
    #[builder(setter(into, strip_option), default)]
    pub option: AuthStrategy,
    #[serde(rename = "cf-turnstile-response")]
    #[builder(setter(into, strip_option), default)]
    pub cf_turnstile_response: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OAuthAccessToken {
    pub access_token: String,
    pub refresh_token: String,
    pub id_token: String,
    pub expires_in: i64,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshToken {
    pub access_token: String,
    #[serde(default)]
    pub refresh_token: String,
    pub id_token: String,
    pub expires_in: i64,
}

#[derive(Deserialize)]
pub struct DashSession {
    pub object: String,
    pub user: DashUser,
    pub invites: Vec<Value>,
}

impl DashSession {
    pub fn sensitive_id(&self) -> &str {
        &self.user.session.sensitive_id
    }

    pub fn user_id(&self) -> &str {
        &self.user.id
    }

    pub fn nickname(&self) -> &str {
        &self.user.name
    }

    pub fn email(&self) -> &str {
        &self.user.email
    }

    pub fn picture(&self) -> &str {
        &self.user.picture
    }
}

#[derive(Deserialize)]
pub struct DashUser {
    pub object: String,
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: String,
    pub created: i64,
    pub session: DashUserSession,
}

#[derive(Deserialize, Serialize)]
pub struct DashUserSession {
    pub sensitive_id: String,
    pub object: String,
    pub name: Option<String>,
    pub created: i64,
    pub last_use: Option<i64>,
    pub publishable: bool,
}

#[derive(Deserialize)]
pub struct DashUserOrgsData {
    pub object: String,
    pub id: String,
    pub created: i64,
    pub title: String,
    pub name: String,
    pub description: String,
    pub personal: bool,
    pub is_default: bool,
    pub role: String,
    pub groups: Vec<Value>,
}

#[derive(Deserialize, Serialize)]
pub struct ApiKey {
    pub result: String,
    pub key: Option<Key>,
}

#[derive(Deserialize, Serialize)]
pub struct ApiKeyList {
    pub object: String,
    pub data: Vec<Key>,
}

#[derive(Deserialize, Serialize)]
pub struct Key {
    pub sensitive_id: String,
    pub object: String,
    pub name: Option<String>,
    pub created: i64,
    #[serde(rename = "last_use")]
    pub last_use: Value,
    pub publishable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SessionAccessToken {
    pub user: WebUser,
    pub expires: String,
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "authProvider")]
    pub auth_provider: String,
}

#[derive(Deserialize, Serialize)]
pub struct WebUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub image: String,
    pub picture: String,
    pub idp: String,
    pub iat: i64,
    pub mfa: bool,
    pub groups: Vec<Value>,
    #[serde(rename = "intercom_hash")]
    pub intercom_hash: String,
}

pub enum AccessToken {
    Session(SessionAccessToken),
    OAuth(OAuthAccessToken),
}

impl Serialize for AccessToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            AccessToken::Session(web) => serializer.serialize_some(web),
            AccessToken::OAuth(apple) => serializer.serialize_some(apple),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Billing {
    pub total_granted: f64,
    pub total_used: f64,
    pub total_available: f64,
    pub total_paid_available: f64,
    pub grants: Grants,
}

#[derive(Serialize, Deserialize)]
pub struct Grants {
    pub data: Vec<Daum>,
}

#[derive(Serialize, Deserialize)]
pub struct Daum {
    pub object: String,
    pub id: String,
    pub grant_amount: f64,
    pub used_amount: f64,
    pub effective_at: f64,
    pub expires_at: f64,
}
