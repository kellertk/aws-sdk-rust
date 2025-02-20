// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_instance_storage_config::_update_instance_storage_config_output::UpdateInstanceStorageConfigOutputBuilder;

pub use crate::operation::update_instance_storage_config::_update_instance_storage_config_input::UpdateInstanceStorageConfigInputBuilder;

impl UpdateInstanceStorageConfigInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_instance_storage_config::UpdateInstanceStorageConfigOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_instance_storage_config::UpdateInstanceStorageConfigError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_instance_storage_config();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateInstanceStorageConfig`.
///
/// <p>This API is in preview release for Amazon Connect and is subject to change.</p>
/// <p>Updates an existing configuration for a resource type. This API is idempotent.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateInstanceStorageConfigFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_instance_storage_config::builders::UpdateInstanceStorageConfigInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_instance_storage_config::UpdateInstanceStorageConfigOutput,
        crate::operation::update_instance_storage_config::UpdateInstanceStorageConfigError,
    > for UpdateInstanceStorageConfigFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_instance_storage_config::UpdateInstanceStorageConfigOutput,
            crate::operation::update_instance_storage_config::UpdateInstanceStorageConfigError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateInstanceStorageConfigFluentBuilder {
    /// Creates a new `UpdateInstanceStorageConfig`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateInstanceStorageConfig as a reference.
    pub fn as_input(&self) -> &crate::operation::update_instance_storage_config::builders::UpdateInstanceStorageConfigInputBuilder {
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
        crate::operation::update_instance_storage_config::UpdateInstanceStorageConfigOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_instance_storage_config::UpdateInstanceStorageConfigError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_instance_storage_config::UpdateInstanceStorageConfig::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_instance_storage_config::UpdateInstanceStorageConfig::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_instance_storage_config::UpdateInstanceStorageConfigOutput,
        crate::operation::update_instance_storage_config::UpdateInstanceStorageConfigError,
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
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_id()
    }
    /// <p>The existing association identifier that uniquely identifies the resource type and storage config for the given instance ID.</p>
    pub fn association_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.association_id(input.into());
        self
    }
    /// <p>The existing association identifier that uniquely identifies the resource type and storage config for the given instance ID.</p>
    pub fn set_association_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_association_id(input);
        self
    }
    /// <p>The existing association identifier that uniquely identifies the resource type and storage config for the given instance ID.</p>
    pub fn get_association_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_association_id()
    }
    /// <p>A valid resource type.</p>
    pub fn resource_type(mut self, input: crate::types::InstanceStorageResourceType) -> Self {
        self.inner = self.inner.resource_type(input);
        self
    }
    /// <p>A valid resource type.</p>
    pub fn set_resource_type(mut self, input: ::std::option::Option<crate::types::InstanceStorageResourceType>) -> Self {
        self.inner = self.inner.set_resource_type(input);
        self
    }
    /// <p>A valid resource type.</p>
    pub fn get_resource_type(&self) -> &::std::option::Option<crate::types::InstanceStorageResourceType> {
        self.inner.get_resource_type()
    }
    /// <p>The storage configuration for the instance.</p>
    pub fn storage_config(mut self, input: crate::types::InstanceStorageConfig) -> Self {
        self.inner = self.inner.storage_config(input);
        self
    }
    /// <p>The storage configuration for the instance.</p>
    pub fn set_storage_config(mut self, input: ::std::option::Option<crate::types::InstanceStorageConfig>) -> Self {
        self.inner = self.inner.set_storage_config(input);
        self
    }
    /// <p>The storage configuration for the instance.</p>
    pub fn get_storage_config(&self) -> &::std::option::Option<crate::types::InstanceStorageConfig> {
        self.inner.get_storage_config()
    }
}
