// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::remove_trust_store_revocations::_remove_trust_store_revocations_output::RemoveTrustStoreRevocationsOutputBuilder;

pub use crate::operation::remove_trust_store_revocations::_remove_trust_store_revocations_input::RemoveTrustStoreRevocationsInputBuilder;

impl RemoveTrustStoreRevocationsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::remove_trust_store_revocations::RemoveTrustStoreRevocationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::remove_trust_store_revocations::RemoveTrustStoreRevocationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.remove_trust_store_revocations();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RemoveTrustStoreRevocations`.
///
/// <p>Removes the specified revocation file from the specified trust store.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RemoveTrustStoreRevocationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::remove_trust_store_revocations::builders::RemoveTrustStoreRevocationsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::remove_trust_store_revocations::RemoveTrustStoreRevocationsOutput,
        crate::operation::remove_trust_store_revocations::RemoveTrustStoreRevocationsError,
    > for RemoveTrustStoreRevocationsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::remove_trust_store_revocations::RemoveTrustStoreRevocationsOutput,
            crate::operation::remove_trust_store_revocations::RemoveTrustStoreRevocationsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RemoveTrustStoreRevocationsFluentBuilder {
    /// Creates a new `RemoveTrustStoreRevocations`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RemoveTrustStoreRevocations as a reference.
    pub fn as_input(&self) -> &crate::operation::remove_trust_store_revocations::builders::RemoveTrustStoreRevocationsInputBuilder {
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
        crate::operation::remove_trust_store_revocations::RemoveTrustStoreRevocationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::remove_trust_store_revocations::RemoveTrustStoreRevocationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::remove_trust_store_revocations::RemoveTrustStoreRevocations::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::remove_trust_store_revocations::RemoveTrustStoreRevocations::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::remove_trust_store_revocations::RemoveTrustStoreRevocationsOutput,
        crate::operation::remove_trust_store_revocations::RemoveTrustStoreRevocationsError,
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
    /// <p>The Amazon Resource Name (ARN) of the trust store.</p>
    pub fn trust_store_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.trust_store_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the trust store.</p>
    pub fn set_trust_store_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_trust_store_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the trust store.</p>
    pub fn get_trust_store_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_trust_store_arn()
    }
    /// Appends an item to `RevocationIds`.
    ///
    /// To override the contents of this collection use [`set_revocation_ids`](Self::set_revocation_ids).
    ///
    /// <p>The revocation IDs of the revocation files you want to remove.</p>
    pub fn revocation_ids(mut self, input: i64) -> Self {
        self.inner = self.inner.revocation_ids(input);
        self
    }
    /// <p>The revocation IDs of the revocation files you want to remove.</p>
    pub fn set_revocation_ids(mut self, input: ::std::option::Option<::std::vec::Vec<i64>>) -> Self {
        self.inner = self.inner.set_revocation_ids(input);
        self
    }
    /// <p>The revocation IDs of the revocation files you want to remove.</p>
    pub fn get_revocation_ids(&self) -> &::std::option::Option<::std::vec::Vec<i64>> {
        self.inner.get_revocation_ids()
    }
}
