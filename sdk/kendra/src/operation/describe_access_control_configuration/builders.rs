// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_access_control_configuration::_describe_access_control_configuration_output::DescribeAccessControlConfigurationOutputBuilder;

pub use crate::operation::describe_access_control_configuration::_describe_access_control_configuration_input::DescribeAccessControlConfigurationInputBuilder;

impl DescribeAccessControlConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_access_control_configuration::DescribeAccessControlConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_access_control_configuration::DescribeAccessControlConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_access_control_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeAccessControlConfiguration`.
///
/// <p>Gets information about an access control configuration that you created for your documents in an index. This includes user and group access information for your documents. This is useful for user context filtering, where search results are filtered based on the user or their group access to documents.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeAccessControlConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_access_control_configuration::builders::DescribeAccessControlConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_access_control_configuration::DescribeAccessControlConfigurationOutput,
        crate::operation::describe_access_control_configuration::DescribeAccessControlConfigurationError,
    > for DescribeAccessControlConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_access_control_configuration::DescribeAccessControlConfigurationOutput,
            crate::operation::describe_access_control_configuration::DescribeAccessControlConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeAccessControlConfigurationFluentBuilder {
    /// Creates a new `DescribeAccessControlConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeAccessControlConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_access_control_configuration::builders::DescribeAccessControlConfigurationInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_access_control_configuration::DescribeAccessControlConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_access_control_configuration::DescribeAccessControlConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_access_control_configuration::DescribeAccessControlConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_access_control_configuration::DescribeAccessControlConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_access_control_configuration::DescribeAccessControlConfigurationOutput,
        crate::operation::describe_access_control_configuration::DescribeAccessControlConfigurationError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The identifier of the index for an access control configuration.</p>
    pub fn index_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.index_id(input.into());
        self
    }
    /// <p>The identifier of the index for an access control configuration.</p>
    pub fn set_index_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_index_id(input);
        self
    }
    /// <p>The identifier of the index for an access control configuration.</p>
    pub fn get_index_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_index_id()
    }
    /// <p>The identifier of the access control configuration you want to get information on.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The identifier of the access control configuration you want to get information on.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The identifier of the access control configuration you want to get information on.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
}
