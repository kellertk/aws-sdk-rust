// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_work_group::_update_work_group_output::UpdateWorkGroupOutputBuilder;

pub use crate::operation::update_work_group::_update_work_group_input::UpdateWorkGroupInputBuilder;

impl UpdateWorkGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_work_group::UpdateWorkGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_work_group::UpdateWorkGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_work_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateWorkGroup`.
///
/// <p>Updates the workgroup with the specified name. The workgroup's name cannot be changed. Only <code>ConfigurationUpdates</code> can be specified.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateWorkGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_work_group::builders::UpdateWorkGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_work_group::UpdateWorkGroupOutput,
        crate::operation::update_work_group::UpdateWorkGroupError,
    > for UpdateWorkGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_work_group::UpdateWorkGroupOutput,
            crate::operation::update_work_group::UpdateWorkGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateWorkGroupFluentBuilder {
    /// Creates a new `UpdateWorkGroup`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateWorkGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::update_work_group::builders::UpdateWorkGroupInputBuilder {
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
        crate::operation::update_work_group::UpdateWorkGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_work_group::UpdateWorkGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_work_group::UpdateWorkGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_work_group::UpdateWorkGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_work_group::UpdateWorkGroupOutput,
        crate::operation::update_work_group::UpdateWorkGroupError,
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
    /// <p>The specified workgroup that will be updated.</p>
    pub fn work_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.work_group(input.into());
        self
    }
    /// <p>The specified workgroup that will be updated.</p>
    pub fn set_work_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_work_group(input);
        self
    }
    /// <p>The specified workgroup that will be updated.</p>
    pub fn get_work_group(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_work_group()
    }
    /// <p>The workgroup description.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The workgroup description.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The workgroup description.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>Contains configuration updates for an Athena SQL workgroup.</p>
    pub fn configuration_updates(mut self, input: crate::types::WorkGroupConfigurationUpdates) -> Self {
        self.inner = self.inner.configuration_updates(input);
        self
    }
    /// <p>Contains configuration updates for an Athena SQL workgroup.</p>
    pub fn set_configuration_updates(mut self, input: ::std::option::Option<crate::types::WorkGroupConfigurationUpdates>) -> Self {
        self.inner = self.inner.set_configuration_updates(input);
        self
    }
    /// <p>Contains configuration updates for an Athena SQL workgroup.</p>
    pub fn get_configuration_updates(&self) -> &::std::option::Option<crate::types::WorkGroupConfigurationUpdates> {
        self.inner.get_configuration_updates()
    }
    /// <p>The workgroup state that will be updated for the given workgroup.</p>
    pub fn state(mut self, input: crate::types::WorkGroupState) -> Self {
        self.inner = self.inner.state(input);
        self
    }
    /// <p>The workgroup state that will be updated for the given workgroup.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::WorkGroupState>) -> Self {
        self.inner = self.inner.set_state(input);
        self
    }
    /// <p>The workgroup state that will be updated for the given workgroup.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::WorkGroupState> {
        self.inner.get_state()
    }
}
