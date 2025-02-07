// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_image_frame::_get_image_frame_output::GetImageFrameOutputBuilder;

pub use crate::operation::get_image_frame::_get_image_frame_input::GetImageFrameInputBuilder;

impl GetImageFrameInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_image_frame::GetImageFrameOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_image_frame::GetImageFrameError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_image_frame();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetImageFrame`.
///
/// <p>Get an image frame (pixel data) for an image set.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetImageFrameFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_image_frame::builders::GetImageFrameInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_image_frame::GetImageFrameOutput,
        crate::operation::get_image_frame::GetImageFrameError,
    > for GetImageFrameFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_image_frame::GetImageFrameOutput,
            crate::operation::get_image_frame::GetImageFrameError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetImageFrameFluentBuilder {
    /// Creates a new `GetImageFrame`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetImageFrame as a reference.
    pub fn as_input(&self) -> &crate::operation::get_image_frame::builders::GetImageFrameInputBuilder {
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
        crate::operation::get_image_frame::GetImageFrameOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_image_frame::GetImageFrameError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_image_frame::GetImageFrame::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_image_frame::GetImageFrame::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_image_frame::GetImageFrameOutput,
        crate::operation::get_image_frame::GetImageFrameError,
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
    /// <p>The data store identifier.</p>
    pub fn datastore_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.datastore_id(input.into());
        self
    }
    /// <p>The data store identifier.</p>
    pub fn set_datastore_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_datastore_id(input);
        self
    }
    /// <p>The data store identifier.</p>
    pub fn get_datastore_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_datastore_id()
    }
    /// <p>The image set identifier.</p>
    pub fn image_set_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.image_set_id(input.into());
        self
    }
    /// <p>The image set identifier.</p>
    pub fn set_image_set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_image_set_id(input);
        self
    }
    /// <p>The image set identifier.</p>
    pub fn get_image_set_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_image_set_id()
    }
    /// <p>Information about the image frame (pixel data) identifier.</p>
    pub fn image_frame_information(mut self, input: crate::types::ImageFrameInformation) -> Self {
        self.inner = self.inner.image_frame_information(input);
        self
    }
    /// <p>Information about the image frame (pixel data) identifier.</p>
    pub fn set_image_frame_information(mut self, input: ::std::option::Option<crate::types::ImageFrameInformation>) -> Self {
        self.inner = self.inner.set_image_frame_information(input);
        self
    }
    /// <p>Information about the image frame (pixel data) identifier.</p>
    pub fn get_image_frame_information(&self) -> &::std::option::Option<crate::types::ImageFrameInformation> {
        self.inner.get_image_frame_information()
    }
}
