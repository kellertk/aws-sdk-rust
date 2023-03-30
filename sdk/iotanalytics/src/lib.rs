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
//! IoT Analytics allows you to collect large amounts of device data, process messages, and store them. You can then query the data and run sophisticated analytics on it. IoT Analytics enables advanced data exploration through integration with Jupyter Notebooks and data visualization through integration with Amazon QuickSight.
//! 
//! Traditional analytics and business intelligence tools are designed to process structured data. IoT data often comes from devices that record noisy processes (such as temperature, motion, or sound). As a result the data from these devices can have significant gaps, corrupted messages, and false readings that must be cleaned up before analysis can occur. Also, IoT data is often only meaningful in the context of other data from external sources.
//! 
//! IoT Analytics automates the steps required to analyze data from IoT devices. IoT Analytics filters, transforms, and enriches IoT data before storing it in a time-series data store for analysis. You can set up the service to collect only the data you need from your devices, apply mathematical transforms to process the data, and enrich the data with device-specific metadata such as device type and location before storing it. Then, you can analyze your data by running queries using the built-in SQL query engine, or perform more complex analytics and machine learning inference. IoT Analytics includes pre-built models for common IoT use cases so you can answer questions like which devices are about to fail or which customers are at risk of abandoning their wearable devices.
//! 
//! ## Getting Started
//! 
//! > Examples are available for many services and operations, check out the
//! > [examples folder in GitHub](https://github.com/awslabs/aws-sdk-rust/tree/main/examples).
//! 
//! The SDK provides one crate per AWS service. You must add [Tokio](https://crates.io/crates/tokio)
//! as a dependency within your Rust project to execute asynchronous code. To add `aws-sdk-iotanalytics` to
//! your project, add the following to your **Cargo.toml** file:
//! 
//! ```toml
//! [dependencies]
//! aws-config = "0.0.0-smithy-rs-head"
//! aws-sdk-iotanalytics = "0.25.0"
//! tokio = { version = "1", features = ["full"] }
//! ```
//! 
//! Then in code, a client can be created with the following:
//! 
//! ```rust,no_run
//! use aws_sdk_iotanalytics as iotanalytics;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), iotanalytics::Error> {
//!     let config = aws_config::load_from_env().await;
//!     let client = iotanalytics::Client::new(&config);
//! 
//!     // ... make some calls with the client
//! 
//!     Ok(())
//! }
//! ```
//! 
//! See the [client documentation](https://docs.rs/aws-sdk-iotanalytics/latest/aws_sdk_iotanalytics/client/struct.Client.html)
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


// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

pub use aws_credential_types::Credentials;

pub use aws_types::region::Region;

pub(crate) static API_METADATA: aws_http::user_agent::ApiMetadata =
                    aws_http::user_agent::ApiMetadata::new("iotanalytics", crate::PKG_VERSION);

pub use aws_types::app_name::AppName;

pub use aws_smithy_http::endpoint::Endpoint;


/// Crate version number.
                pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Client and fluent builders for calling AWS IoT Analytics.
pub mod client;

/// Configuration for AWS IoT Analytics.
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

