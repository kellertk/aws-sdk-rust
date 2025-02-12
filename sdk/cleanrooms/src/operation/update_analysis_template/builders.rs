// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_analysis_template::_update_analysis_template_output::UpdateAnalysisTemplateOutputBuilder;

pub use crate::operation::update_analysis_template::_update_analysis_template_input::UpdateAnalysisTemplateInputBuilder;

impl UpdateAnalysisTemplateInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_analysis_template::UpdateAnalysisTemplateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_analysis_template::UpdateAnalysisTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_analysis_template();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateAnalysisTemplate`.
///
/// <p>Updates the analysis template metadata.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateAnalysisTemplateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_analysis_template::builders::UpdateAnalysisTemplateInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_analysis_template::UpdateAnalysisTemplateOutput,
        crate::operation::update_analysis_template::UpdateAnalysisTemplateError,
    > for UpdateAnalysisTemplateFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_analysis_template::UpdateAnalysisTemplateOutput,
            crate::operation::update_analysis_template::UpdateAnalysisTemplateError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateAnalysisTemplateFluentBuilder {
    /// Creates a new `UpdateAnalysisTemplate`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateAnalysisTemplate as a reference.
    pub fn as_input(&self) -> &crate::operation::update_analysis_template::builders::UpdateAnalysisTemplateInputBuilder {
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
        crate::operation::update_analysis_template::UpdateAnalysisTemplateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_analysis_template::UpdateAnalysisTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_analysis_template::UpdateAnalysisTemplate::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_analysis_template::UpdateAnalysisTemplate::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_analysis_template::UpdateAnalysisTemplateOutput,
        crate::operation::update_analysis_template::UpdateAnalysisTemplateError,
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
    /// <p>The identifier for a membership resource.</p>
    pub fn membership_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.membership_identifier(input.into());
        self
    }
    /// <p>The identifier for a membership resource.</p>
    pub fn set_membership_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_membership_identifier(input);
        self
    }
    /// <p>The identifier for a membership resource.</p>
    pub fn get_membership_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_membership_identifier()
    }
    /// <p>The identifier for the analysis template resource.</p>
    pub fn analysis_template_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.analysis_template_identifier(input.into());
        self
    }
    /// <p>The identifier for the analysis template resource.</p>
    pub fn set_analysis_template_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_analysis_template_identifier(input);
        self
    }
    /// <p>The identifier for the analysis template resource.</p>
    pub fn get_analysis_template_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_analysis_template_identifier()
    }
    /// <p>A new description for the analysis template.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A new description for the analysis template.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A new description for the analysis template.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
}
