// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_notification_settings::_update_notification_settings_output::UpdateNotificationSettingsOutputBuilder;

pub use crate::operation::update_notification_settings::_update_notification_settings_input::UpdateNotificationSettingsInputBuilder;

impl UpdateNotificationSettingsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_notification_settings::UpdateNotificationSettingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_notification_settings::UpdateNotificationSettingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_notification_settings();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateNotificationSettings`.
///
/// <p>The <code>UpdateNotificationSettings</code> operation creates, updates, disables or re-enables notifications for a HIT type. If you call the UpdateNotificationSettings operation for a HIT type that already has a notification specification, the operation replaces the old specification with a new one. You can call the UpdateNotificationSettings operation to enable or disable notifications for the HIT type, without having to modify the notification specification itself by providing updates to the Active status without specifying a new notification specification. To change the Active status of a HIT type's notifications, the HIT type must already have a notification specification, or one must be provided in the same call to <code>UpdateNotificationSettings</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateNotificationSettingsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_notification_settings::builders::UpdateNotificationSettingsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_notification_settings::UpdateNotificationSettingsOutput,
        crate::operation::update_notification_settings::UpdateNotificationSettingsError,
    > for UpdateNotificationSettingsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_notification_settings::UpdateNotificationSettingsOutput,
            crate::operation::update_notification_settings::UpdateNotificationSettingsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateNotificationSettingsFluentBuilder {
    /// Creates a new `UpdateNotificationSettings`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateNotificationSettings as a reference.
    pub fn as_input(&self) -> &crate::operation::update_notification_settings::builders::UpdateNotificationSettingsInputBuilder {
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
        crate::operation::update_notification_settings::UpdateNotificationSettingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_notification_settings::UpdateNotificationSettingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_notification_settings::UpdateNotificationSettings::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_notification_settings::UpdateNotificationSettings::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_notification_settings::UpdateNotificationSettingsOutput,
        crate::operation::update_notification_settings::UpdateNotificationSettingsError,
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
    /// <p>The ID of the HIT type whose notification specification is being updated.</p>
    pub fn hit_type_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.hit_type_id(input.into());
        self
    }
    /// <p>The ID of the HIT type whose notification specification is being updated.</p>
    pub fn set_hit_type_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_hit_type_id(input);
        self
    }
    /// <p>The ID of the HIT type whose notification specification is being updated.</p>
    pub fn get_hit_type_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_hit_type_id()
    }
    /// <p>The notification specification for the HIT type.</p>
    pub fn notification(mut self, input: crate::types::NotificationSpecification) -> Self {
        self.inner = self.inner.notification(input);
        self
    }
    /// <p>The notification specification for the HIT type.</p>
    pub fn set_notification(mut self, input: ::std::option::Option<crate::types::NotificationSpecification>) -> Self {
        self.inner = self.inner.set_notification(input);
        self
    }
    /// <p>The notification specification for the HIT type.</p>
    pub fn get_notification(&self) -> &::std::option::Option<crate::types::NotificationSpecification> {
        self.inner.get_notification()
    }
    /// <p>Specifies whether notifications are sent for HITs of this HIT type, according to the notification specification. You must specify either the Notification parameter or the Active parameter for the call to UpdateNotificationSettings to succeed.</p>
    pub fn active(mut self, input: bool) -> Self {
        self.inner = self.inner.active(input);
        self
    }
    /// <p>Specifies whether notifications are sent for HITs of this HIT type, according to the notification specification. You must specify either the Notification parameter or the Active parameter for the call to UpdateNotificationSettings to succeed.</p>
    pub fn set_active(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_active(input);
        self
    }
    /// <p>Specifies whether notifications are sent for HITs of this HIT type, according to the notification specification. You must specify either the Notification parameter or the Active parameter for the call to UpdateNotificationSettings to succeed.</p>
    pub fn get_active(&self) -> &::std::option::Option<bool> {
        self.inner.get_active()
    }
}
