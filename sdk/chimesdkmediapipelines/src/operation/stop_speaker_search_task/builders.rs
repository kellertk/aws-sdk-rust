// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::stop_speaker_search_task::_stop_speaker_search_task_output::StopSpeakerSearchTaskOutputBuilder;

pub use crate::operation::stop_speaker_search_task::_stop_speaker_search_task_input::StopSpeakerSearchTaskInputBuilder;

impl StopSpeakerSearchTaskInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::stop_speaker_search_task::StopSpeakerSearchTaskOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::stop_speaker_search_task::StopSpeakerSearchTaskError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.stop_speaker_search_task();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StopSpeakerSearchTask`.
///
/// <p>Stops a speaker search task.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StopSpeakerSearchTaskFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::stop_speaker_search_task::builders::StopSpeakerSearchTaskInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::stop_speaker_search_task::StopSpeakerSearchTaskOutput,
        crate::operation::stop_speaker_search_task::StopSpeakerSearchTaskError,
    > for StopSpeakerSearchTaskFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::stop_speaker_search_task::StopSpeakerSearchTaskOutput,
            crate::operation::stop_speaker_search_task::StopSpeakerSearchTaskError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StopSpeakerSearchTaskFluentBuilder {
    /// Creates a new `StopSpeakerSearchTask`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StopSpeakerSearchTask as a reference.
    pub fn as_input(&self) -> &crate::operation::stop_speaker_search_task::builders::StopSpeakerSearchTaskInputBuilder {
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
        crate::operation::stop_speaker_search_task::StopSpeakerSearchTaskOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::stop_speaker_search_task::StopSpeakerSearchTaskError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::stop_speaker_search_task::StopSpeakerSearchTask::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::stop_speaker_search_task::StopSpeakerSearchTask::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::stop_speaker_search_task::StopSpeakerSearchTaskOutput,
        crate::operation::stop_speaker_search_task::StopSpeakerSearchTaskError,
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
    /// <p>The unique identifier of the resource to be updated. Valid values include the ID and ARN of the media insights pipeline.</p>
    pub fn identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identifier(input.into());
        self
    }
    /// <p>The unique identifier of the resource to be updated. Valid values include the ID and ARN of the media insights pipeline.</p>
    pub fn set_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identifier(input);
        self
    }
    /// <p>The unique identifier of the resource to be updated. Valid values include the ID and ARN of the media insights pipeline.</p>
    pub fn get_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_identifier()
    }
    /// <p>The speaker search task ID.</p>
    pub fn speaker_search_task_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.speaker_search_task_id(input.into());
        self
    }
    /// <p>The speaker search task ID.</p>
    pub fn set_speaker_search_task_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_speaker_search_task_id(input);
        self
    }
    /// <p>The speaker search task ID.</p>
    pub fn get_speaker_search_task_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_speaker_search_task_id()
    }
}
