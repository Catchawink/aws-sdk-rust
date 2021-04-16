//! <fullname>Amazon Kinesis Data Streams Service API Reference</fullname>
//! <p>Amazon Kinesis Data Streams is a managed service that scales elastically for
//! real-time processing of streaming big data.</p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_json_errors;
mod blob_serde;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
pub mod input;
mod instant_epoch;
pub mod model;
pub mod operation;
pub mod output;
mod serde_util;
mod serializer;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("kinesis", PKG_VERSION);
pub use aws_auth::Credentials;
pub use aws_types::region::Region;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
