// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::preview_privacy_impact::_preview_privacy_impact_output::PreviewPrivacyImpactOutputBuilder;

pub use crate::operation::preview_privacy_impact::_preview_privacy_impact_input::PreviewPrivacyImpactInputBuilder;

impl PreviewPrivacyImpactInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::preview_privacy_impact::PreviewPrivacyImpactOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::preview_privacy_impact::PreviewPrivacyImpactError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.preview_privacy_impact();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PreviewPrivacyImpact`.
///
/// <p>An estimate of the number of aggregation functions that the member who can query can run given epsilon and noise parameters.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PreviewPrivacyImpactFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::preview_privacy_impact::builders::PreviewPrivacyImpactInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::preview_privacy_impact::PreviewPrivacyImpactOutput,
        crate::operation::preview_privacy_impact::PreviewPrivacyImpactError,
    > for PreviewPrivacyImpactFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::preview_privacy_impact::PreviewPrivacyImpactOutput,
            crate::operation::preview_privacy_impact::PreviewPrivacyImpactError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PreviewPrivacyImpactFluentBuilder {
    /// Creates a new `PreviewPrivacyImpact`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PreviewPrivacyImpact as a reference.
    pub fn as_input(&self) -> &crate::operation::preview_privacy_impact::builders::PreviewPrivacyImpactInputBuilder {
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
        crate::operation::preview_privacy_impact::PreviewPrivacyImpactOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::preview_privacy_impact::PreviewPrivacyImpactError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::preview_privacy_impact::PreviewPrivacyImpact::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::preview_privacy_impact::PreviewPrivacyImpact::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::preview_privacy_impact::PreviewPrivacyImpactOutput,
        crate::operation::preview_privacy_impact::PreviewPrivacyImpactError,
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
    /// <p>A unique identifier for one of your memberships for a collaboration. Accepts a membership ID.</p>
    pub fn membership_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.membership_identifier(input.into());
        self
    }
    /// <p>A unique identifier for one of your memberships for a collaboration. Accepts a membership ID.</p>
    pub fn set_membership_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_membership_identifier(input);
        self
    }
    /// <p>A unique identifier for one of your memberships for a collaboration. Accepts a membership ID.</p>
    pub fn get_membership_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_membership_identifier()
    }
    /// <p>Specifies the desired epsilon and noise parameters to preview.</p>
    pub fn parameters(mut self, input: crate::types::PreviewPrivacyImpactParametersInput) -> Self {
        self.inner = self.inner.parameters(input);
        self
    }
    /// <p>Specifies the desired epsilon and noise parameters to preview.</p>
    pub fn set_parameters(mut self, input: ::std::option::Option<crate::types::PreviewPrivacyImpactParametersInput>) -> Self {
        self.inner = self.inner.set_parameters(input);
        self
    }
    /// <p>Specifies the desired epsilon and noise parameters to preview.</p>
    pub fn get_parameters(&self) -> &::std::option::Option<crate::types::PreviewPrivacyImpactParametersInput> {
        self.inner.get_parameters()
    }
}
