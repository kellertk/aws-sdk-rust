// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_prefetch_schedule::_delete_prefetch_schedule_output::DeletePrefetchScheduleOutputBuilder;

pub use crate::operation::delete_prefetch_schedule::_delete_prefetch_schedule_input::DeletePrefetchScheduleInputBuilder;

impl DeletePrefetchScheduleInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_prefetch_schedule::DeletePrefetchScheduleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_prefetch_schedule::DeletePrefetchScheduleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_prefetch_schedule();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeletePrefetchSchedule`.
///
/// <p>Deletes a prefetch schedule for a specific playback configuration. If you call <code>DeletePrefetchSchedule</code> on an expired prefetch schedule, MediaTailor returns an HTTP 404 status code. For more information about ad prefetching, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/prefetching-ads.html">Using ad prefetching</a> in the <i>MediaTailor User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeletePrefetchScheduleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_prefetch_schedule::builders::DeletePrefetchScheduleInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_prefetch_schedule::DeletePrefetchScheduleOutput,
        crate::operation::delete_prefetch_schedule::DeletePrefetchScheduleError,
    > for DeletePrefetchScheduleFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_prefetch_schedule::DeletePrefetchScheduleOutput,
            crate::operation::delete_prefetch_schedule::DeletePrefetchScheduleError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeletePrefetchScheduleFluentBuilder {
    /// Creates a new `DeletePrefetchSchedule`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeletePrefetchSchedule as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_prefetch_schedule::builders::DeletePrefetchScheduleInputBuilder {
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
        crate::operation::delete_prefetch_schedule::DeletePrefetchScheduleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_prefetch_schedule::DeletePrefetchScheduleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_prefetch_schedule::DeletePrefetchSchedule::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_prefetch_schedule::DeletePrefetchSchedule::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_prefetch_schedule::DeletePrefetchScheduleOutput,
        crate::operation::delete_prefetch_schedule::DeletePrefetchScheduleError,
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
    /// <p>The name of the prefetch schedule. If the action is successful, the service sends back an HTTP 204 response with an empty HTTP body.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the prefetch schedule. If the action is successful, the service sends back an HTTP 204 response with an empty HTTP body.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the prefetch schedule. If the action is successful, the service sends back an HTTP 204 response with an empty HTTP body.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The name of the playback configuration for this prefetch schedule.</p>
    pub fn playback_configuration_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.playback_configuration_name(input.into());
        self
    }
    /// <p>The name of the playback configuration for this prefetch schedule.</p>
    pub fn set_playback_configuration_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_playback_configuration_name(input);
        self
    }
    /// <p>The name of the playback configuration for this prefetch schedule.</p>
    pub fn get_playback_configuration_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_playback_configuration_name()
    }
}
