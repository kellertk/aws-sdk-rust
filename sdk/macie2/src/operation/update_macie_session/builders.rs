// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_macie_session::_update_macie_session_output::UpdateMacieSessionOutputBuilder;

pub use crate::operation::update_macie_session::_update_macie_session_input::UpdateMacieSessionInputBuilder;

impl UpdateMacieSessionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_macie_session::UpdateMacieSessionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_macie_session::UpdateMacieSessionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_macie_session();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateMacieSession`.
///
/// <p>Suspends or re-enables Amazon Macie, or updates the configuration settings for a Macie account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateMacieSessionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_macie_session::builders::UpdateMacieSessionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_macie_session::UpdateMacieSessionOutput,
        crate::operation::update_macie_session::UpdateMacieSessionError,
    > for UpdateMacieSessionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_macie_session::UpdateMacieSessionOutput,
            crate::operation::update_macie_session::UpdateMacieSessionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateMacieSessionFluentBuilder {
    /// Creates a new `UpdateMacieSession`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateMacieSession as a reference.
    pub fn as_input(&self) -> &crate::operation::update_macie_session::builders::UpdateMacieSessionInputBuilder {
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
        crate::operation::update_macie_session::UpdateMacieSessionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_macie_session::UpdateMacieSessionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_macie_session::UpdateMacieSession::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_macie_session::UpdateMacieSession::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_macie_session::UpdateMacieSessionOutput,
        crate::operation::update_macie_session::UpdateMacieSessionError,
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
    /// <p>Specifies how often to publish updates to policy findings for the account. This includes publishing updates to Security Hub and Amazon EventBridge (formerly Amazon CloudWatch Events).</p>
    pub fn finding_publishing_frequency(mut self, input: crate::types::FindingPublishingFrequency) -> Self {
        self.inner = self.inner.finding_publishing_frequency(input);
        self
    }
    /// <p>Specifies how often to publish updates to policy findings for the account. This includes publishing updates to Security Hub and Amazon EventBridge (formerly Amazon CloudWatch Events).</p>
    pub fn set_finding_publishing_frequency(mut self, input: ::std::option::Option<crate::types::FindingPublishingFrequency>) -> Self {
        self.inner = self.inner.set_finding_publishing_frequency(input);
        self
    }
    /// <p>Specifies how often to publish updates to policy findings for the account. This includes publishing updates to Security Hub and Amazon EventBridge (formerly Amazon CloudWatch Events).</p>
    pub fn get_finding_publishing_frequency(&self) -> &::std::option::Option<crate::types::FindingPublishingFrequency> {
        self.inner.get_finding_publishing_frequency()
    }
    /// <p>Specifies a new status for the account. Valid values are: ENABLED, resume all Amazon Macie activities for the account; and, PAUSED, suspend all Macie activities for the account.</p>
    pub fn status(mut self, input: crate::types::MacieStatus) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// <p>Specifies a new status for the account. Valid values are: ENABLED, resume all Amazon Macie activities for the account; and, PAUSED, suspend all Macie activities for the account.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::MacieStatus>) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
    /// <p>Specifies a new status for the account. Valid values are: ENABLED, resume all Amazon Macie activities for the account; and, PAUSED, suspend all Macie activities for the account.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::MacieStatus> {
        self.inner.get_status()
    }
}
