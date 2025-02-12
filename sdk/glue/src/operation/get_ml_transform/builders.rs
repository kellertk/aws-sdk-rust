// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_ml_transform::_get_ml_transform_output::GetMlTransformOutputBuilder;

pub use crate::operation::get_ml_transform::_get_ml_transform_input::GetMlTransformInputBuilder;

impl GetMlTransformInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_ml_transform::GetMlTransformOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_ml_transform::GetMLTransformError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_ml_transform();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetMLTransform`.
///
/// <p>Gets an Glue machine learning transform artifact and all its corresponding metadata. Machine learning transforms are a special type of transform that use machine learning to learn the details of the transformation to be performed by learning from examples provided by humans. These transformations are then saved by Glue. You can retrieve their metadata by calling <code>GetMLTransform</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetMLTransformFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_ml_transform::builders::GetMlTransformInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_ml_transform::GetMlTransformOutput,
        crate::operation::get_ml_transform::GetMLTransformError,
    > for GetMLTransformFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_ml_transform::GetMlTransformOutput,
            crate::operation::get_ml_transform::GetMLTransformError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetMLTransformFluentBuilder {
    /// Creates a new `GetMLTransform`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetMLTransform as a reference.
    pub fn as_input(&self) -> &crate::operation::get_ml_transform::builders::GetMlTransformInputBuilder {
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
        crate::operation::get_ml_transform::GetMlTransformOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_ml_transform::GetMLTransformError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_ml_transform::GetMLTransform::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_ml_transform::GetMLTransform::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_ml_transform::GetMlTransformOutput,
        crate::operation::get_ml_transform::GetMLTransformError,
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
    /// <p>The unique identifier of the transform, generated at the time that the transform was created.</p>
    pub fn transform_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.transform_id(input.into());
        self
    }
    /// <p>The unique identifier of the transform, generated at the time that the transform was created.</p>
    pub fn set_transform_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_transform_id(input);
        self
    }
    /// <p>The unique identifier of the transform, generated at the time that the transform was created.</p>
    pub fn get_transform_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_transform_id()
    }
}
