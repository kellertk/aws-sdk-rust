// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::stop_app_block_builder::_stop_app_block_builder_output::StopAppBlockBuilderOutputBuilder;

pub use crate::operation::stop_app_block_builder::_stop_app_block_builder_input::StopAppBlockBuilderInputBuilder;

impl StopAppBlockBuilderInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::stop_app_block_builder::StopAppBlockBuilderOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::stop_app_block_builder::StopAppBlockBuilderError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.stop_app_block_builder();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StopAppBlockBuilder`.
///
/// <p>Stops an app block builder.</p>
/// <p>Stopping an app block builder terminates the instance, and the instance state is not persisted.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StopAppBlockBuilderFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::stop_app_block_builder::builders::StopAppBlockBuilderInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::stop_app_block_builder::StopAppBlockBuilderOutput,
        crate::operation::stop_app_block_builder::StopAppBlockBuilderError,
    > for StopAppBlockBuilderFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::stop_app_block_builder::StopAppBlockBuilderOutput,
            crate::operation::stop_app_block_builder::StopAppBlockBuilderError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StopAppBlockBuilderFluentBuilder {
    /// Creates a new `StopAppBlockBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StopAppBlockBuilder as a reference.
    pub fn as_input(&self) -> &crate::operation::stop_app_block_builder::builders::StopAppBlockBuilderInputBuilder {
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
        crate::operation::stop_app_block_builder::StopAppBlockBuilderOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::stop_app_block_builder::StopAppBlockBuilderError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::stop_app_block_builder::StopAppBlockBuilder::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::stop_app_block_builder::StopAppBlockBuilder::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::stop_app_block_builder::StopAppBlockBuilderOutput,
        crate::operation::stop_app_block_builder::StopAppBlockBuilderError,
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
    /// <p>The name of the app block builder.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the app block builder.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the app block builder.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
}
