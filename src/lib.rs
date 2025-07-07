mod client;
mod core;
pub mod environment;
pub mod models;
pub mod resources;
pub use client::Client;
pub use core::{
    response::BinaryResponse, error::{ApiError, Error},
    patch::Patch, upload_file::UploadFile,
};
pub use environment::Environment;
pub type SdkResult<T> = Result<T, crate::core::error::Error>;
