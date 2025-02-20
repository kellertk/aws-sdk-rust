// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_execution::_get_execution_output::GetExecutionOutputBuilder;

pub use crate::operation::get_execution::_get_execution_input::GetExecutionInputBuilder;

impl GetExecutionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_execution::GetExecutionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_execution::GetExecutionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_execution();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetExecution`.
///
/// <p>Exports data based on the source data update.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetExecutionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_execution::builders::GetExecutionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_execution::GetExecutionOutput,
        crate::operation::get_execution::GetExecutionError,
    > for GetExecutionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_execution::GetExecutionOutput,
            crate::operation::get_execution::GetExecutionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetExecutionFluentBuilder {
    /// Creates a new `GetExecution`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetExecution as a reference.
    pub fn as_input(&self) -> &crate::operation::get_execution::builders::GetExecutionInputBuilder {
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
        crate::operation::get_execution::GetExecutionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_execution::GetExecutionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_execution::GetExecution::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_execution::GetExecution::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_execution::GetExecutionOutput,
        crate::operation::get_execution::GetExecutionError,
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
    /// <p>The Amazon Resource Name (ARN) of the Export object that generated this specific execution.</p>
    pub fn export_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.export_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Export object that generated this specific execution.</p>
    pub fn set_export_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_export_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Export object that generated this specific execution.</p>
    pub fn get_export_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_export_arn()
    }
    /// <p>The ID for this specific execution.</p>
    pub fn execution_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.execution_id(input.into());
        self
    }
    /// <p>The ID for this specific execution.</p>
    pub fn set_execution_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_execution_id(input);
        self
    }
    /// <p>The ID for this specific execution.</p>
    pub fn get_execution_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_execution_id()
    }
}
