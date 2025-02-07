// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_key::_put_key_output::PutKeyOutputBuilder;

pub use crate::operation::put_key::_put_key_input::PutKeyInputBuilder;

impl PutKeyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_key::PutKeyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_key::PutKeyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_key();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutKey`.
///
/// <p>Creates a new key value pair or replaces the value of an existing key.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutKeyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_key::builders::PutKeyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::put_key::PutKeyOutput, crate::operation::put_key::PutKeyError>
    for PutKeyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::put_key::PutKeyOutput, crate::operation::put_key::PutKeyError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutKeyFluentBuilder {
    /// Creates a new `PutKey`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutKey as a reference.
    pub fn as_input(&self) -> &crate::operation::put_key::builders::PutKeyInputBuilder {
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
        crate::operation::put_key::PutKeyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_key::PutKeyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_key::PutKey::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_key::PutKey::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<crate::operation::put_key::PutKeyOutput, crate::operation::put_key::PutKeyError, Self> {
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
    /// <p>The key to put.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key(input.into());
        self
    }
    /// <p>The key to put.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key(input);
        self
    }
    /// <p>The key to put.</p>
    pub fn get_key(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key()
    }
    /// <p>The value to put.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.value(input.into());
        self
    }
    /// <p>The value to put.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_value(input);
        self
    }
    /// <p>The value to put.</p>
    pub fn get_value(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_value()
    }
    /// <p>The Amazon Resource Name (ARN) of the Key Value Store.</p>
    pub fn kvs_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kvs_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Key Value Store.</p>
    pub fn set_kvs_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kvs_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Key Value Store.</p>
    pub fn get_kvs_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_kvs_arn()
    }
    /// <p>The current version (ETag) of the Key Value Store that you are putting keys into, which you can get using DescribeKeyValueStore.</p>
    pub fn if_match(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.if_match(input.into());
        self
    }
    /// <p>The current version (ETag) of the Key Value Store that you are putting keys into, which you can get using DescribeKeyValueStore.</p>
    pub fn set_if_match(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_if_match(input);
        self
    }
    /// <p>The current version (ETag) of the Key Value Store that you are putting keys into, which you can get using DescribeKeyValueStore.</p>
    pub fn get_if_match(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_if_match()
    }
}
