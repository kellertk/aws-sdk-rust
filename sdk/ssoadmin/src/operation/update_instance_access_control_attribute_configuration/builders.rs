// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_instance_access_control_attribute_configuration::_update_instance_access_control_attribute_configuration_output::UpdateInstanceAccessControlAttributeConfigurationOutputBuilder;

pub use crate::operation::update_instance_access_control_attribute_configuration::_update_instance_access_control_attribute_configuration_input::UpdateInstanceAccessControlAttributeConfigurationInputBuilder;

impl UpdateInstanceAccessControlAttributeConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_instance_access_control_attribute_configuration::UpdateInstanceAccessControlAttributeConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_instance_access_control_attribute_configuration::UpdateInstanceAccessControlAttributeConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_instance_access_control_attribute_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateInstanceAccessControlAttributeConfiguration`.
///
/// <p>Updates the IAM Identity Center identity store attributes that you can use with the IAM Identity Center instance for attributes-based access control (ABAC). When using an external identity provider as an identity source, you can pass attributes through the SAML assertion as an alternative to configuring attributes from the IAM Identity Center identity store. If a SAML assertion passes any of these attributes, IAM Identity Center replaces the attribute value with the value from the IAM Identity Center identity store. For more information about ABAC, see <a href="/singlesignon/latest/userguide/abac.html">Attribute-Based Access Control</a> in the <i>IAM Identity Center User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateInstanceAccessControlAttributeConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_instance_access_control_attribute_configuration::builders::UpdateInstanceAccessControlAttributeConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_instance_access_control_attribute_configuration::UpdateInstanceAccessControlAttributeConfigurationOutput,
        crate::operation::update_instance_access_control_attribute_configuration::UpdateInstanceAccessControlAttributeConfigurationError,
    > for UpdateInstanceAccessControlAttributeConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_instance_access_control_attribute_configuration::UpdateInstanceAccessControlAttributeConfigurationOutput,
            crate::operation::update_instance_access_control_attribute_configuration::UpdateInstanceAccessControlAttributeConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateInstanceAccessControlAttributeConfigurationFluentBuilder {
    /// Creates a new `UpdateInstanceAccessControlAttributeConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateInstanceAccessControlAttributeConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::update_instance_access_control_attribute_configuration::builders::UpdateInstanceAccessControlAttributeConfigurationInputBuilder{
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
        crate::operation::update_instance_access_control_attribute_configuration::UpdateInstanceAccessControlAttributeConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_instance_access_control_attribute_configuration::UpdateInstanceAccessControlAttributeConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_instance_access_control_attribute_configuration::UpdateInstanceAccessControlAttributeConfiguration::operation_runtime_plugins(
                            self.handle.runtime_plugins.clone(),
                            &self.handle.conf,
                            self.config_override,
                        );
        crate::operation::update_instance_access_control_attribute_configuration::UpdateInstanceAccessControlAttributeConfiguration::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_instance_access_control_attribute_configuration::UpdateInstanceAccessControlAttributeConfigurationOutput,
        crate::operation::update_instance_access_control_attribute_configuration::UpdateInstanceAccessControlAttributeConfigurationError,
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
    /// <p>The ARN of the IAM Identity Center instance under which the operation will be executed.</p>
    pub fn instance_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_arn(input.into());
        self
    }
    /// <p>The ARN of the IAM Identity Center instance under which the operation will be executed.</p>
    pub fn set_instance_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_arn(input);
        self
    }
    /// <p>The ARN of the IAM Identity Center instance under which the operation will be executed.</p>
    pub fn get_instance_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_arn()
    }
    /// <p>Updates the attributes for your ABAC configuration.</p>
    pub fn instance_access_control_attribute_configuration(mut self, input: crate::types::InstanceAccessControlAttributeConfiguration) -> Self {
        self.inner = self.inner.instance_access_control_attribute_configuration(input);
        self
    }
    /// <p>Updates the attributes for your ABAC configuration.</p>
    pub fn set_instance_access_control_attribute_configuration(
        mut self,
        input: ::std::option::Option<crate::types::InstanceAccessControlAttributeConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_instance_access_control_attribute_configuration(input);
        self
    }
    /// <p>Updates the attributes for your ABAC configuration.</p>
    pub fn get_instance_access_control_attribute_configuration(
        &self,
    ) -> &::std::option::Option<crate::types::InstanceAccessControlAttributeConfiguration> {
        self.inner.get_instance_access_control_attribute_configuration()
    }
}
