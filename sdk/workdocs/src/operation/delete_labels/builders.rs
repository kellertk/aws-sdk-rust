// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_labels::_delete_labels_output::DeleteLabelsOutputBuilder;

pub use crate::operation::delete_labels::_delete_labels_input::DeleteLabelsInputBuilder;

impl DeleteLabelsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_labels::DeleteLabelsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_labels::DeleteLabelsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_labels();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteLabels`.
///
/// <p>Deletes the specified list of labels from a resource.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteLabelsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_labels::builders::DeleteLabelsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_labels::DeleteLabelsOutput,
        crate::operation::delete_labels::DeleteLabelsError,
    > for DeleteLabelsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_labels::DeleteLabelsOutput,
            crate::operation::delete_labels::DeleteLabelsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteLabelsFluentBuilder {
    /// Creates a new `DeleteLabels`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteLabels as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_labels::builders::DeleteLabelsInputBuilder {
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
        crate::operation::delete_labels::DeleteLabelsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_labels::DeleteLabelsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_labels::DeleteLabels::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_labels::DeleteLabels::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_labels::DeleteLabelsOutput,
        crate::operation::delete_labels::DeleteLabelsError,
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
    /// <p>The ID of the resource.</p>
    pub fn resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_id(input.into());
        self
    }
    /// <p>The ID of the resource.</p>
    pub fn set_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_id(input);
        self
    }
    /// <p>The ID of the resource.</p>
    pub fn get_resource_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_id()
    }
    /// <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p>
    pub fn authentication_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.authentication_token(input.into());
        self
    }
    /// <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p>
    pub fn set_authentication_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_authentication_token(input);
        self
    }
    /// <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p>
    pub fn get_authentication_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_authentication_token()
    }
    /// Appends an item to `Labels`.
    ///
    /// To override the contents of this collection use [`set_labels`](Self::set_labels).
    ///
    /// <p>List of labels to delete from the resource.</p>
    pub fn labels(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.labels(input.into());
        self
    }
    /// <p>List of labels to delete from the resource.</p>
    pub fn set_labels(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_labels(input);
        self
    }
    /// <p>List of labels to delete from the resource.</p>
    pub fn get_labels(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_labels()
    }
    /// <p>Flag to request removal of all labels from the specified resource.</p>
    pub fn delete_all(mut self, input: bool) -> Self {
        self.inner = self.inner.delete_all(input);
        self
    }
    /// <p>Flag to request removal of all labels from the specified resource.</p>
    pub fn set_delete_all(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_delete_all(input);
        self
    }
    /// <p>Flag to request removal of all labels from the specified resource.</p>
    pub fn get_delete_all(&self) -> &::std::option::Option<bool> {
        self.inner.get_delete_all()
    }
}
