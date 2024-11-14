/// Generated by Sideko (sideko.dev)
use std::fmt::Display;
#[derive(Debug, Default, Clone)]
pub enum Environment {
    #[default]
    Environment,
}
impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Environment => write!(f, "https://api.magichour.ai"),
        }
    }
}
#[derive(Clone, Debug)]
pub(crate) enum BaseUrl {
    Env(crate::environment::Environment),
    Custom(String),
}
impl Default for BaseUrl {
    fn default() -> Self {
        BaseUrl::Env(crate::environment::Environment::default())
    }
}
impl std::fmt::Display for BaseUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Env(e) => write!(f, "{e}"),
            Self::Custom(url) => write!(f, "{url}"),
        }
    }
}
