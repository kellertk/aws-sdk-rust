// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_backend_job::_update_backend_job_output::UpdateBackendJobOutputBuilder;

pub use crate::operation::update_backend_job::_update_backend_job_input::UpdateBackendJobInputBuilder;

impl UpdateBackendJobInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_backend_job::UpdateBackendJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_backend_job::UpdateBackendJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_backend_job();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateBackendJob`.
///
/// <p>Updates a specific job.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateBackendJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_backend_job::builders::UpdateBackendJobInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_backend_job::UpdateBackendJobOutput,
        crate::operation::update_backend_job::UpdateBackendJobError,
    > for UpdateBackendJobFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_backend_job::UpdateBackendJobOutput,
            crate::operation::update_backend_job::UpdateBackendJobError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateBackendJobFluentBuilder {
    /// Creates a new `UpdateBackendJob`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateBackendJob as a reference.
    pub fn as_input(&self) -> &crate::operation::update_backend_job::builders::UpdateBackendJobInputBuilder {
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
        crate::operation::update_backend_job::UpdateBackendJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_backend_job::UpdateBackendJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_backend_job::UpdateBackendJob::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_backend_job::UpdateBackendJob::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_backend_job::UpdateBackendJobOutput,
        crate::operation::update_backend_job::UpdateBackendJobError,
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
    /// <p>The app ID.</p>
    pub fn app_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.app_id(input.into());
        self
    }
    /// <p>The app ID.</p>
    pub fn set_app_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_app_id(input);
        self
    }
    /// <p>The app ID.</p>
    pub fn get_app_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_app_id()
    }
    /// <p>The name of the backend environment.</p>
    pub fn backend_environment_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.backend_environment_name(input.into());
        self
    }
    /// <p>The name of the backend environment.</p>
    pub fn set_backend_environment_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_backend_environment_name(input);
        self
    }
    /// <p>The name of the backend environment.</p>
    pub fn get_backend_environment_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_backend_environment_name()
    }
    /// <p>The ID for the job.</p>
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.job_id(input.into());
        self
    }
    /// <p>The ID for the job.</p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_job_id(input);
        self
    }
    /// <p>The ID for the job.</p>
    pub fn get_job_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_job_id()
    }
    /// <p>Filters the list of response objects to include only those with the specified operation name.</p>
    pub fn operation(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.operation(input.into());
        self
    }
    /// <p>Filters the list of response objects to include only those with the specified operation name.</p>
    pub fn set_operation(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_operation(input);
        self
    }
    /// <p>Filters the list of response objects to include only those with the specified operation name.</p>
    pub fn get_operation(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_operation()
    }
    /// <p>Filters the list of response objects to include only those with the specified status.</p>
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.status(input.into());
        self
    }
    /// <p>Filters the list of response objects to include only those with the specified status.</p>
    pub fn set_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
    /// <p>Filters the list of response objects to include only those with the specified status.</p>
    pub fn get_status(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_status()
    }
}
