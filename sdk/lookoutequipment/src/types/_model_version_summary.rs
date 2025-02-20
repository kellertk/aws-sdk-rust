// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about the specific model version.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModelVersionSummary {
    /// <p>The name of the model that this model version is a version of.</p>
    pub model_name: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the model that this model version is a version of.</p>
    pub model_arn: ::std::option::Option<::std::string::String>,
    /// <p>The version of the model.</p>
    pub model_version: ::std::option::Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the model version.</p>
    pub model_version_arn: ::std::option::Option<::std::string::String>,
    /// <p>The time when this model version was created.</p>
    pub created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The current status of the model version.</p>
    pub status: ::std::option::Option<crate::types::ModelVersionStatus>,
    /// <p>Indicates how this model version was generated.</p>
    pub source_type: ::std::option::Option<crate::types::ModelVersionSourceType>,
}
impl ModelVersionSummary {
    /// <p>The name of the model that this model version is a version of.</p>
    pub fn model_name(&self) -> ::std::option::Option<&str> {
        self.model_name.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the model that this model version is a version of.</p>
    pub fn model_arn(&self) -> ::std::option::Option<&str> {
        self.model_arn.as_deref()
    }
    /// <p>The version of the model.</p>
    pub fn model_version(&self) -> ::std::option::Option<i64> {
        self.model_version
    }
    /// <p>The Amazon Resource Name (ARN) of the model version.</p>
    pub fn model_version_arn(&self) -> ::std::option::Option<&str> {
        self.model_version_arn.as_deref()
    }
    /// <p>The time when this model version was created.</p>
    pub fn created_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_at.as_ref()
    }
    /// <p>The current status of the model version.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ModelVersionStatus> {
        self.status.as_ref()
    }
    /// <p>Indicates how this model version was generated.</p>
    pub fn source_type(&self) -> ::std::option::Option<&crate::types::ModelVersionSourceType> {
        self.source_type.as_ref()
    }
}
impl ModelVersionSummary {
    /// Creates a new builder-style object to manufacture [`ModelVersionSummary`](crate::types::ModelVersionSummary).
    pub fn builder() -> crate::types::builders::ModelVersionSummaryBuilder {
        crate::types::builders::ModelVersionSummaryBuilder::default()
    }
}

/// A builder for [`ModelVersionSummary`](crate::types::ModelVersionSummary).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ModelVersionSummaryBuilder {
    pub(crate) model_name: ::std::option::Option<::std::string::String>,
    pub(crate) model_arn: ::std::option::Option<::std::string::String>,
    pub(crate) model_version: ::std::option::Option<i64>,
    pub(crate) model_version_arn: ::std::option::Option<::std::string::String>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) status: ::std::option::Option<crate::types::ModelVersionStatus>,
    pub(crate) source_type: ::std::option::Option<crate::types::ModelVersionSourceType>,
}
impl ModelVersionSummaryBuilder {
    /// <p>The name of the model that this model version is a version of.</p>
    pub fn model_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.model_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the model that this model version is a version of.</p>
    pub fn set_model_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.model_name = input;
        self
    }
    /// <p>The name of the model that this model version is a version of.</p>
    pub fn get_model_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.model_name
    }
    /// <p>The Amazon Resource Name (ARN) of the model that this model version is a version of.</p>
    pub fn model_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.model_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the model that this model version is a version of.</p>
    pub fn set_model_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.model_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the model that this model version is a version of.</p>
    pub fn get_model_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.model_arn
    }
    /// <p>The version of the model.</p>
    pub fn model_version(mut self, input: i64) -> Self {
        self.model_version = ::std::option::Option::Some(input);
        self
    }
    /// <p>The version of the model.</p>
    pub fn set_model_version(mut self, input: ::std::option::Option<i64>) -> Self {
        self.model_version = input;
        self
    }
    /// <p>The version of the model.</p>
    pub fn get_model_version(&self) -> &::std::option::Option<i64> {
        &self.model_version
    }
    /// <p>The Amazon Resource Name (ARN) of the model version.</p>
    pub fn model_version_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.model_version_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the model version.</p>
    pub fn set_model_version_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.model_version_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the model version.</p>
    pub fn get_model_version_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.model_version_arn
    }
    /// <p>The time when this model version was created.</p>
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time when this model version was created.</p>
    pub fn set_created_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_at = input;
        self
    }
    /// <p>The time when this model version was created.</p>
    pub fn get_created_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_at
    }
    /// <p>The current status of the model version.</p>
    pub fn status(mut self, input: crate::types::ModelVersionStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current status of the model version.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ModelVersionStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>The current status of the model version.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::ModelVersionStatus> {
        &self.status
    }
    /// <p>Indicates how this model version was generated.</p>
    pub fn source_type(mut self, input: crate::types::ModelVersionSourceType) -> Self {
        self.source_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates how this model version was generated.</p>
    pub fn set_source_type(mut self, input: ::std::option::Option<crate::types::ModelVersionSourceType>) -> Self {
        self.source_type = input;
        self
    }
    /// <p>Indicates how this model version was generated.</p>
    pub fn get_source_type(&self) -> &::std::option::Option<crate::types::ModelVersionSourceType> {
        &self.source_type
    }
    /// Consumes the builder and constructs a [`ModelVersionSummary`](crate::types::ModelVersionSummary).
    pub fn build(self) -> crate::types::ModelVersionSummary {
        crate::types::ModelVersionSummary {
            model_name: self.model_name,
            model_arn: self.model_arn,
            model_version: self.model_version,
            model_version_arn: self.model_version_arn,
            created_at: self.created_at,
            status: self.status,
            source_type: self.source_type,
        }
    }
}
