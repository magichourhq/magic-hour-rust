mod client;
mod core;
pub mod environment;
pub mod models;
pub mod resources;
pub use client::Client;
pub use core::{
    auth::{OAuth2ClientCredentials, OAuth2Password},
    response::BinaryResponse, error::{ApiError, Error},
    patch::Patch, upload_file::UploadFile,
};
pub type SdkResult<T> = Result<T, crate::core::error::Error>;
