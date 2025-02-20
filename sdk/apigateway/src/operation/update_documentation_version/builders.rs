// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_documentation_version::_update_documentation_version_output::UpdateDocumentationVersionOutputBuilder;

pub use crate::operation::update_documentation_version::_update_documentation_version_input::UpdateDocumentationVersionInputBuilder;

impl UpdateDocumentationVersionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_documentation_version::UpdateDocumentationVersionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_documentation_version::UpdateDocumentationVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_documentation_version();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateDocumentationVersion`.
///
/// <p>Updates a documentation version.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateDocumentationVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_documentation_version::builders::UpdateDocumentationVersionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_documentation_version::UpdateDocumentationVersionOutput,
        crate::operation::update_documentation_version::UpdateDocumentationVersionError,
    > for UpdateDocumentationVersionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_documentation_version::UpdateDocumentationVersionOutput,
            crate::operation::update_documentation_version::UpdateDocumentationVersionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateDocumentationVersionFluentBuilder {
    /// Creates a new `UpdateDocumentationVersion`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateDocumentationVersion as a reference.
    pub fn as_input(&self) -> &crate::operation::update_documentation_version::builders::UpdateDocumentationVersionInputBuilder {
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
        crate::operation::update_documentation_version::UpdateDocumentationVersionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_documentation_version::UpdateDocumentationVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_documentation_version::UpdateDocumentationVersion::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_documentation_version::UpdateDocumentationVersion::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_documentation_version::UpdateDocumentationVersionOutput,
        crate::operation::update_documentation_version::UpdateDocumentationVersionError,
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
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn rest_api_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.rest_api_id(input.into());
        self
    }
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn set_rest_api_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_rest_api_id(input);
        self
    }
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn get_rest_api_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_rest_api_id()
    }
    /// <p>The version identifier of the to-be-updated documentation version.</p>
    pub fn documentation_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.documentation_version(input.into());
        self
    }
    /// <p>The version identifier of the to-be-updated documentation version.</p>
    pub fn set_documentation_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_documentation_version(input);
        self
    }
    /// <p>The version identifier of the to-be-updated documentation version.</p>
    pub fn get_documentation_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_documentation_version()
    }
    /// Appends an item to `patchOperations`.
    ///
    /// To override the contents of this collection use [`set_patch_operations`](Self::set_patch_operations).
    ///
    /// <p>For more information about supported patch operations, see <a href="https://docs.aws.amazon.com/apigateway/latest/api/patch-operations.html">Patch Operations</a>.</p>
    pub fn patch_operations(mut self, input: crate::types::PatchOperation) -> Self {
        self.inner = self.inner.patch_operations(input);
        self
    }
    /// <p>For more information about supported patch operations, see <a href="https://docs.aws.amazon.com/apigateway/latest/api/patch-operations.html">Patch Operations</a>.</p>
    pub fn set_patch_operations(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PatchOperation>>) -> Self {
        self.inner = self.inner.set_patch_operations(input);
        self
    }
    /// <p>For more information about supported patch operations, see <a href="https://docs.aws.amazon.com/apigateway/latest/api/patch-operations.html">Patch Operations</a>.</p>
    pub fn get_patch_operations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::PatchOperation>> {
        self.inner.get_patch_operations()
    }
}
