// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_continuous_export::_start_continuous_export_output::StartContinuousExportOutputBuilder;

pub use crate::operation::start_continuous_export::_start_continuous_export_input::StartContinuousExportInputBuilder;

impl StartContinuousExportInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_continuous_export::StartContinuousExportOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_continuous_export::StartContinuousExportError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_continuous_export();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartContinuousExport`.
///
/// <p>Start the continuous flow of agent's discovered data into Amazon Athena.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartContinuousExportFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_continuous_export::builders::StartContinuousExportInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_continuous_export::StartContinuousExportOutput,
        crate::operation::start_continuous_export::StartContinuousExportError,
    > for StartContinuousExportFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_continuous_export::StartContinuousExportOutput,
            crate::operation::start_continuous_export::StartContinuousExportError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartContinuousExportFluentBuilder {
    /// Creates a new `StartContinuousExport`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartContinuousExport as a reference.
    pub fn as_input(&self) -> &crate::operation::start_continuous_export::builders::StartContinuousExportInputBuilder {
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
        crate::operation::start_continuous_export::StartContinuousExportOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_continuous_export::StartContinuousExportError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_continuous_export::StartContinuousExport::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_continuous_export::StartContinuousExport::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_continuous_export::StartContinuousExportOutput,
        crate::operation::start_continuous_export::StartContinuousExportError,
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
}
