// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Represents the properties of a logging configuration metadata.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LoggingConfigurationMetadata {
    /// The status of the logging configuration.
    pub status: ::std::option::Option<crate::types::LoggingConfigurationStatus>,
    /// The workspace where the logging configuration exists.
    pub workspace: ::std::string::String,
    /// The ARN of the CW log group to which the vended log data will be published.
    pub log_group_arn: ::std::string::String,
    /// The time when the logging configuration was created.
    pub created_at: ::aws_smithy_types::DateTime,
    /// The time when the logging configuration was modified.
    pub modified_at: ::aws_smithy_types::DateTime,
}
impl LoggingConfigurationMetadata {
    /// The status of the logging configuration.
    pub fn status(&self) -> ::std::option::Option<&crate::types::LoggingConfigurationStatus> {
        self.status.as_ref()
    }
    /// The workspace where the logging configuration exists.
    pub fn workspace(&self) -> &str {
        use std::ops::Deref;
        self.workspace.deref()
    }
    /// The ARN of the CW log group to which the vended log data will be published.
    pub fn log_group_arn(&self) -> &str {
        use std::ops::Deref;
        self.log_group_arn.deref()
    }
    /// The time when the logging configuration was created.
    pub fn created_at(&self) -> &::aws_smithy_types::DateTime {
        &self.created_at
    }
    /// The time when the logging configuration was modified.
    pub fn modified_at(&self) -> &::aws_smithy_types::DateTime {
        &self.modified_at
    }
}
impl LoggingConfigurationMetadata {
    /// Creates a new builder-style object to manufacture [`LoggingConfigurationMetadata`](crate::types::LoggingConfigurationMetadata).
    pub fn builder() -> crate::types::builders::LoggingConfigurationMetadataBuilder {
        crate::types::builders::LoggingConfigurationMetadataBuilder::default()
    }
}

/// A builder for [`LoggingConfigurationMetadata`](crate::types::LoggingConfigurationMetadata).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct LoggingConfigurationMetadataBuilder {
    pub(crate) status: ::std::option::Option<crate::types::LoggingConfigurationStatus>,
    pub(crate) workspace: ::std::option::Option<::std::string::String>,
    pub(crate) log_group_arn: ::std::option::Option<::std::string::String>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) modified_at: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl LoggingConfigurationMetadataBuilder {
    /// The status of the logging configuration.
    /// This field is required.
    pub fn status(mut self, input: crate::types::LoggingConfigurationStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// The status of the logging configuration.
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::LoggingConfigurationStatus>) -> Self {
        self.status = input;
        self
    }
    /// The status of the logging configuration.
    pub fn get_status(&self) -> &::std::option::Option<crate::types::LoggingConfigurationStatus> {
        &self.status
    }
    /// The workspace where the logging configuration exists.
    /// This field is required.
    pub fn workspace(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.workspace = ::std::option::Option::Some(input.into());
        self
    }
    /// The workspace where the logging configuration exists.
    pub fn set_workspace(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.workspace = input;
        self
    }
    /// The workspace where the logging configuration exists.
    pub fn get_workspace(&self) -> &::std::option::Option<::std::string::String> {
        &self.workspace
    }
    /// The ARN of the CW log group to which the vended log data will be published.
    /// This field is required.
    pub fn log_group_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.log_group_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// The ARN of the CW log group to which the vended log data will be published.
    pub fn set_log_group_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.log_group_arn = input;
        self
    }
    /// The ARN of the CW log group to which the vended log data will be published.
    pub fn get_log_group_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.log_group_arn
    }
    /// The time when the logging configuration was created.
    /// This field is required.
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// The time when the logging configuration was created.
    pub fn set_created_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_at = input;
        self
    }
    /// The time when the logging configuration was created.
    pub fn get_created_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_at
    }
    /// The time when the logging configuration was modified.
    /// This field is required.
    pub fn modified_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.modified_at = ::std::option::Option::Some(input);
        self
    }
    /// The time when the logging configuration was modified.
    pub fn set_modified_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.modified_at = input;
        self
    }
    /// The time when the logging configuration was modified.
    pub fn get_modified_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.modified_at
    }
    /// Consumes the builder and constructs a [`LoggingConfigurationMetadata`](crate::types::LoggingConfigurationMetadata).
    /// This method will fail if any of the following fields are not set:
    /// - [`workspace`](crate::types::builders::LoggingConfigurationMetadataBuilder::workspace)
    /// - [`log_group_arn`](crate::types::builders::LoggingConfigurationMetadataBuilder::log_group_arn)
    /// - [`created_at`](crate::types::builders::LoggingConfigurationMetadataBuilder::created_at)
    /// - [`modified_at`](crate::types::builders::LoggingConfigurationMetadataBuilder::modified_at)
    pub fn build(self) -> ::std::result::Result<crate::types::LoggingConfigurationMetadata, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::LoggingConfigurationMetadata {
            status: self.status,
            workspace: self.workspace.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "workspace",
                    "workspace was not specified but it is required when building LoggingConfigurationMetadata",
                )
            })?,
            log_group_arn: self.log_group_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "log_group_arn",
                    "log_group_arn was not specified but it is required when building LoggingConfigurationMetadata",
                )
            })?,
            created_at: self.created_at.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "created_at",
                    "created_at was not specified but it is required when building LoggingConfigurationMetadata",
                )
            })?,
            modified_at: self.modified_at.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "modified_at",
                    "modified_at was not specified but it is required when building LoggingConfigurationMetadata",
                )
            })?,
        })
    }
}
