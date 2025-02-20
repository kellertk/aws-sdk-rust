// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_configuration_set::_create_configuration_set_output::CreateConfigurationSetOutputBuilder;

pub use crate::operation::create_configuration_set::_create_configuration_set_input::CreateConfigurationSetInputBuilder;

impl CreateConfigurationSetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_configuration_set::CreateConfigurationSetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_configuration_set::CreateConfigurationSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_configuration_set();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateConfigurationSet`.
///
/// <p>Creates a configuration set.</p>
/// <p>Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/dg/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
/// <p>You can execute this operation no more than once per second.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateConfigurationSetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_configuration_set::builders::CreateConfigurationSetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_configuration_set::CreateConfigurationSetOutput,
        crate::operation::create_configuration_set::CreateConfigurationSetError,
    > for CreateConfigurationSetFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_configuration_set::CreateConfigurationSetOutput,
            crate::operation::create_configuration_set::CreateConfigurationSetError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateConfigurationSetFluentBuilder {
    /// Creates a new `CreateConfigurationSet`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateConfigurationSet as a reference.
    pub fn as_input(&self) -> &crate::operation::create_configuration_set::builders::CreateConfigurationSetInputBuilder {
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
        crate::operation::create_configuration_set::CreateConfigurationSetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_configuration_set::CreateConfigurationSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_configuration_set::CreateConfigurationSet::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_configuration_set::CreateConfigurationSet::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_configuration_set::CreateConfigurationSetOutput,
        crate::operation::create_configuration_set::CreateConfigurationSetError,
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
    /// <p>A data structure that contains the name of the configuration set.</p>
    pub fn configuration_set(mut self, input: crate::types::ConfigurationSet) -> Self {
        self.inner = self.inner.configuration_set(input);
        self
    }
    /// <p>A data structure that contains the name of the configuration set.</p>
    pub fn set_configuration_set(mut self, input: ::std::option::Option<crate::types::ConfigurationSet>) -> Self {
        self.inner = self.inner.set_configuration_set(input);
        self
    }
    /// <p>A data structure that contains the name of the configuration set.</p>
    pub fn get_configuration_set(&self) -> &::std::option::Option<crate::types::ConfigurationSet> {
        self.inner.get_configuration_set()
    }
}
