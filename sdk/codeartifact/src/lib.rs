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
//! CodeArtifact is a fully managed artifact repository compatible with language-native package managers and build tools such as npm, Apache Maven, pip, and dotnet. You can use CodeArtifact to share packages with development teams and pull packages. Packages can be pulled from both public and CodeArtifact repositories. You can also create an upstream relationship between a CodeArtifact repository and another repository, which effectively merges their contents from the point of view of a package manager client.
//! 
//! __CodeArtifact Components__
//! 
//! Use the information in this guide to help you work with the following CodeArtifact components:
//!   - __Repository__: A CodeArtifact repository contains a set of [package versions](https://docs.aws.amazon.com/codeartifact/latest/ug/welcome.html#welcome-concepts-package-version), each of which maps to a set of assets, or files. Repositories are polyglot, so a single repository can contain packages of any supported type. Each repository exposes endpoints for fetching and publishing packages using tools like the __ npm __ CLI, the Maven CLI (__ mvn __), Python CLIs (__ pip __ and twine), and NuGet CLIs (nuget and dotnet).
//!   - __Domain__: Repositories are aggregated into a higher-level entity known as a _domain_. All package assets and metadata are stored in the domain, but are consumed through repositories. A given package asset, such as a Maven JAR file, is stored once per domain, no matter how many repositories it's present in. All of the assets and metadata in a domain are encrypted with the same customer master key (CMK) stored in Key Management Service (KMS). Each repository is a member of a single domain and can't be moved to a different domain. The domain allows organizational policy to be applied across multiple repositories, such as which accounts can access repositories in the domain, and which public repositories can be used as sources of packages. Although an organization can have multiple domains, we recommend a single production domain that contains all published artifacts so that teams can find and share packages across their organization.
//!   - __Package__: A _package_ is a bundle of software and the metadata required to resolve dependencies and install the software. CodeArtifact supports [npm](https://docs.aws.amazon.com/codeartifact/latest/ug/using-npm.html), [PyPI](https://docs.aws.amazon.com/codeartifact/latest/ug/using-python.html), [Maven](https://docs.aws.amazon.com/codeartifact/latest/ug/using-maven), and [NuGet](https://docs.aws.amazon.com/codeartifact/latest/ug/using-nuget) package formats. In CodeArtifact, a package consists of:
//!     - A _name_ (for example, webpack is the name of a popular npm package)
//!     - An optional namespace (for example, @types in @types/node)
//!     - A set of versions (for example, 1.0.0, 1.0.1, 1.0.2, etc.)
//!     - Package-level metadata (for example, npm tags)
//! 
//!   - __Package version__: A version of a package, such as @types/node 12.6.9. The version number format and semantics vary for different package formats. For example, npm package versions must conform to the [Semantic Versioning specification](https://semver.org/). In CodeArtifact, a package version consists of the version identifier, metadata at the package version level, and a set of assets.
//!   - __Upstream repository__: One repository is _upstream_ of another when the package versions in it can be accessed from the repository endpoint of the downstream repository, effectively merging the contents of the two repositories from the point of view of a client. CodeArtifact allows creating an upstream relationship between two repositories.
//!   - __Asset__: An individual file stored in CodeArtifact associated with a package version, such as an npm .tgz file or Maven POM and JAR files.
//! 
//! CodeArtifact supports these operations:
//!   - AssociateExternalConnection: Adds an existing external connection to a repository.
//!   - CopyPackageVersions: Copies package versions from one repository to another repository in the same domain.
//!   - CreateDomain: Creates a domain
//!   - CreateRepository: Creates a CodeArtifact repository in a domain.
//!   - DeleteDomain: Deletes a domain. You cannot delete a domain that contains repositories.
//!   - DeleteDomainPermissionsPolicy: Deletes the resource policy that is set on a domain.
//!   - DeletePackageVersions: Deletes versions of a package. After a package has been deleted, it can be republished, but its assets and metadata cannot be restored because they have been permanently removed from storage.
//!   - DeleteRepository: Deletes a repository.
//!   - DeleteRepositoryPermissionsPolicy: Deletes the resource policy that is set on a repository.
//!   - DescribeDomain: Returns a DomainDescription object that contains information about the requested domain.
//!   - DescribePackage: Returns a [PackageDescription](https://docs.aws.amazon.com/codeartifact/latest/APIReference/API_PackageDescription.html) object that contains details about a package.
//!   - DescribePackageVersion: Returns a [PackageVersionDescription](https://docs.aws.amazon.com/codeartifact/latest/APIReference/API_PackageVersionDescription.html) object that contains details about a package version.
//!   - DescribeRepository: Returns a RepositoryDescription object that contains detailed information about the requested repository.
//!   - DisposePackageVersions: Disposes versions of a package. A package version with the status Disposed cannot be restored because they have been permanently removed from storage.
//!   - DisassociateExternalConnection: Removes an existing external connection from a repository.
//!   - GetAuthorizationToken: Generates a temporary authorization token for accessing repositories in the domain. The token expires the authorization period has passed. The default authorization period is 12 hours and can be customized to any length with a maximum of 12 hours.
//!   - GetDomainPermissionsPolicy: Returns the policy of a resource that is attached to the specified domain.
//!   - GetPackageVersionAsset: Returns the contents of an asset that is in a package version.
//!   - GetPackageVersionReadme: Gets the readme file or descriptive text for a package version.
//!   - GetRepositoryEndpoint: Returns the endpoint of a repository for a specific package format. A repository has one endpoint for each package format:
//!     - maven
//!     - npm
//!     - nuget
//!     - pypi
//! 
//!   - GetRepositoryPermissionsPolicy: Returns the resource policy that is set on a repository.
//!   - ListDomains: Returns a list of DomainSummary objects. Each returned DomainSummary object contains information about a domain.
//!   - ListPackages: Lists the packages in a repository.
//!   - ListPackageVersionAssets: Lists the assets for a given package version.
//!   - ListPackageVersionDependencies: Returns a list of the direct dependencies for a package version.
//!   - ListPackageVersions: Returns a list of package versions for a specified package in a repository.
//!   - ListRepositories: Returns a list of repositories owned by the Amazon Web Services account that called this method.
//!   - ListRepositoriesInDomain: Returns a list of the repositories in a domain.
//!   - PutDomainPermissionsPolicy: Attaches a resource policy to a domain.
//!   - PutPackageOriginConfiguration: Sets the package origin configuration for a package, which determine how new versions of the package can be added to a specific repository.
//!   - PutRepositoryPermissionsPolicy: Sets the resource policy on a repository that specifies permissions to access it.
//!   - UpdatePackageVersionsStatus: Updates the status of one or more versions of a package.
//!   - UpdateRepository: Updates the properties of a repository.
//! 
//! ## Getting Started
//! 
//! > Examples are available for many services and operations, check out the
//! > [examples folder in GitHub](https://github.com/awslabs/aws-sdk-rust/tree/main/examples).
//! 
//! The SDK provides one crate per AWS service. You must add [Tokio](https://crates.io/crates/tokio)
//! as a dependency within your Rust project to execute asynchronous code. To add `aws-sdk-codeartifact` to
//! your project, add the following to your **Cargo.toml** file:
//! 
//! ```toml
//! [dependencies]
//! aws-config = "0.0.0-smithy-rs-head"
//! aws-sdk-codeartifact = "0.25.0"
//! tokio = { version = "1", features = ["full"] }
//! ```
//! 
//! Then in code, a client can be created with the following:
//! 
//! ```rust,no_run
//! use aws_sdk_codeartifact as codeartifact;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), codeartifact::Error> {
//!     let config = aws_config::load_from_env().await;
//!     let client = codeartifact::Client::new(&config);
//! 
//!     // ... make some calls with the client
//! 
//!     Ok(())
//! }
//! ```
//! 
//! See the [client documentation](https://docs.rs/aws-sdk-codeartifact/latest/aws_sdk_codeartifact/client/struct.Client.html)
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
                    aws_http::user_agent::ApiMetadata::new("codeartifact", crate::PKG_VERSION);

pub use aws_types::app_name::AppName;

pub use aws_smithy_http::endpoint::Endpoint;


/// Crate version number.
                pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Client and fluent builders for calling CodeArtifact.
pub mod client;

/// Configuration for CodeArtifact.
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

