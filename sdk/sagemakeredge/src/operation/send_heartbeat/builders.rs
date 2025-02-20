// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::send_heartbeat::_send_heartbeat_output::SendHeartbeatOutputBuilder;

pub use crate::operation::send_heartbeat::_send_heartbeat_input::SendHeartbeatInputBuilder;

impl SendHeartbeatInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::send_heartbeat::SendHeartbeatOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::send_heartbeat::SendHeartbeatError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.send_heartbeat();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SendHeartbeat`.
///
/// <p>Use to get the current status of devices registered on SageMaker Edge Manager.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SendHeartbeatFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::send_heartbeat::builders::SendHeartbeatInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::send_heartbeat::SendHeartbeatOutput,
        crate::operation::send_heartbeat::SendHeartbeatError,
    > for SendHeartbeatFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::send_heartbeat::SendHeartbeatOutput,
            crate::operation::send_heartbeat::SendHeartbeatError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SendHeartbeatFluentBuilder {
    /// Creates a new `SendHeartbeat`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SendHeartbeat as a reference.
    pub fn as_input(&self) -> &crate::operation::send_heartbeat::builders::SendHeartbeatInputBuilder {
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
        crate::operation::send_heartbeat::SendHeartbeatOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::send_heartbeat::SendHeartbeatError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::send_heartbeat::SendHeartbeat::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::send_heartbeat::SendHeartbeat::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::send_heartbeat::SendHeartbeatOutput,
        crate::operation::send_heartbeat::SendHeartbeatError,
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
    /// Appends an item to `AgentMetrics`.
    ///
    /// To override the contents of this collection use [`set_agent_metrics`](Self::set_agent_metrics).
    ///
    /// <p>For internal use. Returns a list of SageMaker Edge Manager agent operating metrics.</p>
    pub fn agent_metrics(mut self, input: crate::types::EdgeMetric) -> Self {
        self.inner = self.inner.agent_metrics(input);
        self
    }
    /// <p>For internal use. Returns a list of SageMaker Edge Manager agent operating metrics.</p>
    pub fn set_agent_metrics(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::EdgeMetric>>) -> Self {
        self.inner = self.inner.set_agent_metrics(input);
        self
    }
    /// <p>For internal use. Returns a list of SageMaker Edge Manager agent operating metrics.</p>
    pub fn get_agent_metrics(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::EdgeMetric>> {
        self.inner.get_agent_metrics()
    }
    /// Appends an item to `Models`.
    ///
    /// To override the contents of this collection use [`set_models`](Self::set_models).
    ///
    /// <p>Returns a list of models deployed on the the device.</p>
    pub fn models(mut self, input: crate::types::Model) -> Self {
        self.inner = self.inner.models(input);
        self
    }
    /// <p>Returns a list of models deployed on the the device.</p>
    pub fn set_models(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Model>>) -> Self {
        self.inner = self.inner.set_models(input);
        self
    }
    /// <p>Returns a list of models deployed on the the device.</p>
    pub fn get_models(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Model>> {
        self.inner.get_models()
    }
    /// <p>Returns the version of the agent.</p>
    pub fn agent_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.agent_version(input.into());
        self
    }
    /// <p>Returns the version of the agent.</p>
    pub fn set_agent_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_agent_version(input);
        self
    }
    /// <p>Returns the version of the agent.</p>
    pub fn get_agent_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_agent_version()
    }
    /// <p>The unique name of the device.</p>
    pub fn device_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.device_name(input.into());
        self
    }
    /// <p>The unique name of the device.</p>
    pub fn set_device_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_device_name(input);
        self
    }
    /// <p>The unique name of the device.</p>
    pub fn get_device_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_device_name()
    }
    /// <p>The name of the fleet that the device belongs to.</p>
    pub fn device_fleet_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.device_fleet_name(input.into());
        self
    }
    /// <p>The name of the fleet that the device belongs to.</p>
    pub fn set_device_fleet_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_device_fleet_name(input);
        self
    }
    /// <p>The name of the fleet that the device belongs to.</p>
    pub fn get_device_fleet_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_device_fleet_name()
    }
    /// <p>Returns the result of a deployment on the device.</p>
    pub fn deployment_result(mut self, input: crate::types::DeploymentResult) -> Self {
        self.inner = self.inner.deployment_result(input);
        self
    }
    /// <p>Returns the result of a deployment on the device.</p>
    pub fn set_deployment_result(mut self, input: ::std::option::Option<crate::types::DeploymentResult>) -> Self {
        self.inner = self.inner.set_deployment_result(input);
        self
    }
    /// <p>Returns the result of a deployment on the device.</p>
    pub fn get_deployment_result(&self) -> &::std::option::Option<crate::types::DeploymentResult> {
        self.inner.get_deployment_result()
    }
}
