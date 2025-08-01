use std::fmt::Display;
#[derive(Debug, Default, Clone)]
pub enum Environment {
    #[default]
    Environment,
    MockServer,
}
impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Environment => write!(f, "https://api.magichour.ai"),
            Environment::MockServer => {
                write!(f, "https://api.sideko.dev/v1/mock/magichour/magic-hour/0.33.0")
            }
        }
    }
}
#[derive(Clone, Debug)]
pub enum BaseUrl {
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
impl From<String> for BaseUrl {
    fn from(value: String) -> Self {
        BaseUrl::Custom(value)
    }
}
impl From<Environment> for BaseUrl {
    fn from(value: Environment) -> Self {
        BaseUrl::Env(value)
    }
}
