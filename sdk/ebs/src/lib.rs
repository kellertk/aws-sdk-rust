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
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(rustdoc::bare_urls)]

#![warn(missing_docs)]
//! **Please Note: The SDK is currently in Developer Preview and is intended strictly for
//! feedback purposes only. Do not use this SDK for production workloads.**
//! 
//! You can use the Amazon Elastic Block Store (Amazon EBS) direct APIs to create Amazon EBS snapshots, write data directly to your snapshots, read data on your snapshots, and identify the differences or changes between two snapshots. If you’re an independent software vendor (ISV) who offers backup services for Amazon EBS, the EBS direct APIs make it more efficient and cost-effective to track incremental changes on your Amazon EBS volumes through snapshots. This can be done without having to create new volumes from snapshots, and then use Amazon Elastic Compute Cloud (Amazon EC2) instances to compare the differences.
//! 
//! You can create incremental snapshots directly from data on-premises into volumes and the cloud to use for quick disaster recovery. With the ability to write and read snapshots, you can write your on-premises data to an snapshot during a disaster. Then after recovery, you can restore it back to Amazon Web Services or on-premises from the snapshot. You no longer need to build and maintain complex mechanisms to copy data to and from Amazon EBS.
//! 
//! This API reference provides detailed information about the actions, data types, parameters, and errors of the EBS direct APIs. For more information about the elements that make up the EBS direct APIs, and examples of how to use them effectively, see [Accessing the Contents of an Amazon EBS Snapshot](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-accessing-snapshot.html) in the _Amazon Elastic Compute Cloud User Guide_. For more information about the supported Amazon Web Services Regions, endpoints, and service quotas for the EBS direct APIs, see [Amazon Elastic Block Store Endpoints and Quotas](https://docs.aws.amazon.com/general/latest/gr/ebs-service.html) in the _Amazon Web Services General Reference_.
//! 
//! ## Getting Started
//! 
//! > Examples are available for many services and operations, check out the
//! > [examples folder in GitHub](https://github.com/awslabs/aws-sdk-rust/tree/main/examples).
//! 
//! The SDK provides one crate per AWS service. You must add [Tokio](https://crates.io/crates/tokio)
//! as a dependency within your Rust project to execute asynchronous code. To add `aws-sdk-ebs` to
//! your project, add the following to your **Cargo.toml** file:
//! 
//! ```toml
//! [dependencies]
//! aws-config = "0.0.0-smithy-rs-head"
//! aws-sdk-ebs = "0.25.0"
//! tokio = { version = "1", features = ["full"] }
//! ```
//! 
//! Then in code, a client can be created with the following:
//! 
//! ```rust,no_run
//! use aws_sdk_ebs as ebs;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), ebs::Error> {
//!     let config = aws_config::load_from_env().await;
//!     let client = ebs::Client::new(&config);
//! 
//!     // ... make some calls with the client
//! 
//!     Ok(())
//! }
//! ```
//! 
//! See the [client documentation](https://docs.rs/aws-sdk-ebs/latest/aws_sdk_ebs/client/struct.Client.html)
//! for information on what calls can be made, and the inputs and outputs for each of those calls.
//! 
//! ## Using the SDK
//! 
//! Until the SDK is released, we will be adding information about using the SDK to the
//! [Developer Guide](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html). Feel free to suggest
//! additional sections for the guide by opening an issue and describing what you are trying to do.
//! 
//! ## Getting Help
//! 
//! * [GitHub discussions](https://github.com/awslabs/aws-sdk-rust/discussions) - For ideas, RFCs & general questions
//! * [GitHub issues](https://github.com/awslabs/aws-sdk-rust/issues/new/choose) - For bug reports & feature requests
//! * [Generated Docs (latest version)](https://awslabs.github.io/aws-sdk-rust/)
//! * [Usage examples](https://github.com/awslabs/aws-sdk-rust/tree/main/examples)
//! 
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
//! Examples can be found [here](https://github.com/awslabs/aws-sdk-rust/tree/main/examples/ebs).


// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

pub use aws_credential_types::Credentials;

pub use aws_types::region::Region;

pub(crate) static API_METADATA: aws_http::user_agent::ApiMetadata =
                    aws_http::user_agent::ApiMetadata::new("ebs", crate::PKG_VERSION);

pub use aws_types::app_name::AppName;

pub use aws_smithy_http::endpoint::Endpoint;


/// Crate version number.
                pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Client and fluent builders for calling Amazon Elastic Block Store.
pub mod client;

/// Configuration for Amazon Elastic Block Store.
pub mod config;

/// Endpoint resolution functionality.
pub mod endpoint;

/// All error types that operations can return. Documentation on these types is copied from the model.
pub mod error;

mod error_meta;

/// Input structures for operations. Documentation on these types is copied from the model.
pub mod input;

/// Data structures used by operation inputs/outputs.
pub mod model;

/// All operations that this crate can perform.
pub mod operation;

/// Output structures for operations. Documentation on these types is copied from the model.
pub mod output;

/// Data primitives referenced by other data types.
pub mod types;

mod idempotency_token;

/// 
pub mod middleware;

/// 
mod no_credentials;

/// Paginators for the service
pub mod paginator;

mod lens;

pub(crate) mod protocol_serde;

mod endpoint_lib;

mod json_errors;

#[doc(inline)]
pub use client::Client;

