// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_world_generation_job::_create_world_generation_job_output::CreateWorldGenerationJobOutputBuilder;

pub use crate::operation::create_world_generation_job::_create_world_generation_job_input::CreateWorldGenerationJobInputBuilder;

impl CreateWorldGenerationJobInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_world_generation_job::CreateWorldGenerationJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_world_generation_job::CreateWorldGenerationJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_world_generation_job();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateWorldGenerationJob`.
///
/// <p>Creates worlds using the specified template.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateWorldGenerationJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_world_generation_job::builders::CreateWorldGenerationJobInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_world_generation_job::CreateWorldGenerationJobOutput,
        crate::operation::create_world_generation_job::CreateWorldGenerationJobError,
    > for CreateWorldGenerationJobFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_world_generation_job::CreateWorldGenerationJobOutput,
            crate::operation::create_world_generation_job::CreateWorldGenerationJobError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateWorldGenerationJobFluentBuilder {
    /// Creates a new `CreateWorldGenerationJob`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateWorldGenerationJob as a reference.
    pub fn as_input(&self) -> &crate::operation::create_world_generation_job::builders::CreateWorldGenerationJobInputBuilder {
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
        crate::operation::create_world_generation_job::CreateWorldGenerationJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_world_generation_job::CreateWorldGenerationJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_world_generation_job::CreateWorldGenerationJob::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_world_generation_job::CreateWorldGenerationJob::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_world_generation_job::CreateWorldGenerationJobOutput,
        crate::operation::create_world_generation_job::CreateWorldGenerationJobError,
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
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_request_token()
    }
    /// <p>The Amazon Resource Name (arn) of the world template describing the worlds you want to create.</p>
    pub fn template(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.template(input.into());
        self
    }
    /// <p>The Amazon Resource Name (arn) of the world template describing the worlds you want to create.</p>
    pub fn set_template(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_template(input);
        self
    }
    /// <p>The Amazon Resource Name (arn) of the world template describing the worlds you want to create.</p>
    pub fn get_template(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_template()
    }
    /// <p>Information about the world count.</p>
    pub fn world_count(mut self, input: crate::types::WorldCount) -> Self {
        self.inner = self.inner.world_count(input);
        self
    }
    /// <p>Information about the world count.</p>
    pub fn set_world_count(mut self, input: ::std::option::Option<crate::types::WorldCount>) -> Self {
        self.inner = self.inner.set_world_count(input);
        self
    }
    /// <p>Information about the world count.</p>
    pub fn get_world_count(&self) -> &::std::option::Option<crate::types::WorldCount> {
        self.inner.get_world_count()
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A map that contains tag keys and tag values that are attached to the world generator job.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>A map that contains tag keys and tag values that are attached to the world generator job.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A map that contains tag keys and tag values that are attached to the world generator job.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
    /// Adds a key-value pair to `worldTags`.
    ///
    /// To override the contents of this collection use [`set_world_tags`](Self::set_world_tags).
    ///
    /// <p>A map that contains tag keys and tag values that are attached to the generated worlds.</p>
    pub fn world_tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.world_tags(k.into(), v.into());
        self
    }
    /// <p>A map that contains tag keys and tag values that are attached to the generated worlds.</p>
    pub fn set_world_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_world_tags(input);
        self
    }
    /// <p>A map that contains tag keys and tag values that are attached to the generated worlds.</p>
    pub fn get_world_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_world_tags()
    }
}
