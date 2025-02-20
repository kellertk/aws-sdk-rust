// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_repository_description::_update_repository_description_output::UpdateRepositoryDescriptionOutputBuilder;

pub use crate::operation::update_repository_description::_update_repository_description_input::UpdateRepositoryDescriptionInputBuilder;

impl UpdateRepositoryDescriptionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_repository_description::UpdateRepositoryDescriptionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_repository_description::UpdateRepositoryDescriptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_repository_description();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateRepositoryDescription`.
///
/// <p>Sets or changes the comment or description for a repository.</p><note>
/// <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a webpage can expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a webpage.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateRepositoryDescriptionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_repository_description::builders::UpdateRepositoryDescriptionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_repository_description::UpdateRepositoryDescriptionOutput,
        crate::operation::update_repository_description::UpdateRepositoryDescriptionError,
    > for UpdateRepositoryDescriptionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_repository_description::UpdateRepositoryDescriptionOutput,
            crate::operation::update_repository_description::UpdateRepositoryDescriptionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateRepositoryDescriptionFluentBuilder {
    /// Creates a new `UpdateRepositoryDescription`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateRepositoryDescription as a reference.
    pub fn as_input(&self) -> &crate::operation::update_repository_description::builders::UpdateRepositoryDescriptionInputBuilder {
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
        crate::operation::update_repository_description::UpdateRepositoryDescriptionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_repository_description::UpdateRepositoryDescriptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_repository_description::UpdateRepositoryDescription::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_repository_description::UpdateRepositoryDescription::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_repository_description::UpdateRepositoryDescriptionOutput,
        crate::operation::update_repository_description::UpdateRepositoryDescriptionError,
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
    /// <p>The name of the repository to set or change the comment or description for.</p>
    pub fn repository_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.repository_name(input.into());
        self
    }
    /// <p>The name of the repository to set or change the comment or description for.</p>
    pub fn set_repository_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_repository_name(input);
        self
    }
    /// <p>The name of the repository to set or change the comment or description for.</p>
    pub fn get_repository_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_repository_name()
    }
    /// <p>The new comment or description for the specified repository. Repository descriptions are limited to 1,000 characters.</p>
    pub fn repository_description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.repository_description(input.into());
        self
    }
    /// <p>The new comment or description for the specified repository. Repository descriptions are limited to 1,000 characters.</p>
    pub fn set_repository_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_repository_description(input);
        self
    }
    /// <p>The new comment or description for the specified repository. Repository descriptions are limited to 1,000 characters.</p>
    pub fn get_repository_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_repository_description()
    }
}
