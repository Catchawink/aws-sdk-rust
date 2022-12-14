#![allow(deprecated)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_return)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>You can use Amazon CloudWatch Logs to monitor, store, and access your log files from
//! EC2 instances, CloudTrail, and other sources. You can then retrieve the associated
//! log data from CloudWatch Logs using the CloudWatch console, CloudWatch Logs commands in the
//! Amazon Web Services CLI, CloudWatch Logs API, or CloudWatch Logs SDK.</p>
//! <p>You can use CloudWatch Logs to:</p>
//! <ul>
//! <li>
//! <p>
//! <b>Monitor logs from EC2 instances in real-time</b>: You
//! can use CloudWatch Logs to monitor applications and systems using log data. For example,
//! CloudWatch Logs can track the number of errors that occur in your application logs and
//! send you a notification whenever the rate of errors exceeds a threshold that you specify.
//! CloudWatch Logs uses your log data for monitoring so no code changes are required. For
//! example, you can monitor application logs for specific literal terms (such as
//! "NullReferenceException") or count the number of occurrences of a literal term at a
//! particular position in log data (such as "404" status codes in an Apache access log). When
//! the term you are searching for is found, CloudWatch Logs reports the data to a CloudWatch
//! metric that you specify.</p>
//! </li>
//! <li>
//! <p>
//! <b>Monitor CloudTrail logged events</b>: You can
//! create alarms in CloudWatch and receive notifications of particular API activity as
//! captured by CloudTrail. You can use the notification to perform troubleshooting.</p>
//! </li>
//! <li>
//! <p>
//! <b>Archive log data</b>: You can use CloudWatch Logs to
//! store your log data in highly durable storage. You can change the log retention setting so
//! that any log events older than this setting are automatically deleted. The CloudWatch Logs
//! agent makes it easy to quickly send both rotated and non-rotated log data off of a host
//! and into the log service. You can then access the raw log data when you need it.</p>
//! </li>
//! </ul>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.
//!
//! # Examples
//! Examples can be found [here](https://github.com/awslabs/aws-sdk-rust/tree/main/examples/cloudwatchlogs).

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

/// Client and fluent builders for calling the service.
pub mod client;

/// Configuration for the service.
pub mod config;

/// All error types that operations can return. Documentation on these types is copied from the model.
pub mod error;

mod error_meta;

/// Input structures for operations. Documentation on these types is copied from the model.
pub mod input;

/// Data structures used by operation inputs/outputs. Documentation on these types is copied from the model.
pub mod model;

/// All operations that this crate can perform.
pub mod operation;

/// Output structures for operations. Documentation on these types is copied from the model.
pub mod output;

/// Data primitives referenced by other data types.
pub mod types;

mod aws_endpoint;

pub mod middleware;

mod no_credentials;

mod operation_deser;

mod operation_ser;

/// Paginators for the service
pub mod paginator;

mod json_deser;

mod json_ser;

/// Generated accessors for nested fields
mod lens;

mod json_errors;

/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("cloudwatchlogs", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
