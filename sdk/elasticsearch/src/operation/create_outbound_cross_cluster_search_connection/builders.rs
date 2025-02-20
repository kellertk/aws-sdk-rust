// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_outbound_cross_cluster_search_connection::_create_outbound_cross_cluster_search_connection_output::CreateOutboundCrossClusterSearchConnectionOutputBuilder;

pub use crate::operation::create_outbound_cross_cluster_search_connection::_create_outbound_cross_cluster_search_connection_input::CreateOutboundCrossClusterSearchConnectionInputBuilder;

impl CreateOutboundCrossClusterSearchConnectionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_outbound_cross_cluster_search_connection::CreateOutboundCrossClusterSearchConnectionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_outbound_cross_cluster_search_connection::CreateOutboundCrossClusterSearchConnectionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_outbound_cross_cluster_search_connection();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateOutboundCrossClusterSearchConnection`.
///
/// <p>Creates a new cross-cluster search connection from a source domain to a destination domain.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateOutboundCrossClusterSearchConnectionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_outbound_cross_cluster_search_connection::builders::CreateOutboundCrossClusterSearchConnectionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_outbound_cross_cluster_search_connection::CreateOutboundCrossClusterSearchConnectionOutput,
        crate::operation::create_outbound_cross_cluster_search_connection::CreateOutboundCrossClusterSearchConnectionError,
    > for CreateOutboundCrossClusterSearchConnectionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_outbound_cross_cluster_search_connection::CreateOutboundCrossClusterSearchConnectionOutput,
            crate::operation::create_outbound_cross_cluster_search_connection::CreateOutboundCrossClusterSearchConnectionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateOutboundCrossClusterSearchConnectionFluentBuilder {
    /// Creates a new `CreateOutboundCrossClusterSearchConnection`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateOutboundCrossClusterSearchConnection as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::create_outbound_cross_cluster_search_connection::builders::CreateOutboundCrossClusterSearchConnectionInputBuilder {
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
        crate::operation::create_outbound_cross_cluster_search_connection::CreateOutboundCrossClusterSearchConnectionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_outbound_cross_cluster_search_connection::CreateOutboundCrossClusterSearchConnectionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::create_outbound_cross_cluster_search_connection::CreateOutboundCrossClusterSearchConnection::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::create_outbound_cross_cluster_search_connection::CreateOutboundCrossClusterSearchConnection::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_outbound_cross_cluster_search_connection::CreateOutboundCrossClusterSearchConnectionOutput,
        crate::operation::create_outbound_cross_cluster_search_connection::CreateOutboundCrossClusterSearchConnectionError,
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
    /// <p>Specifies the <code><code>DomainInformation</code></code> for the source Elasticsearch domain.</p>
    pub fn source_domain_info(mut self, input: crate::types::DomainInformation) -> Self {
        self.inner = self.inner.source_domain_info(input);
        self
    }
    /// <p>Specifies the <code><code>DomainInformation</code></code> for the source Elasticsearch domain.</p>
    pub fn set_source_domain_info(mut self, input: ::std::option::Option<crate::types::DomainInformation>) -> Self {
        self.inner = self.inner.set_source_domain_info(input);
        self
    }
    /// <p>Specifies the <code><code>DomainInformation</code></code> for the source Elasticsearch domain.</p>
    pub fn get_source_domain_info(&self) -> &::std::option::Option<crate::types::DomainInformation> {
        self.inner.get_source_domain_info()
    }
    /// <p>Specifies the <code><code>DomainInformation</code></code> for the destination Elasticsearch domain.</p>
    pub fn destination_domain_info(mut self, input: crate::types::DomainInformation) -> Self {
        self.inner = self.inner.destination_domain_info(input);
        self
    }
    /// <p>Specifies the <code><code>DomainInformation</code></code> for the destination Elasticsearch domain.</p>
    pub fn set_destination_domain_info(mut self, input: ::std::option::Option<crate::types::DomainInformation>) -> Self {
        self.inner = self.inner.set_destination_domain_info(input);
        self
    }
    /// <p>Specifies the <code><code>DomainInformation</code></code> for the destination Elasticsearch domain.</p>
    pub fn get_destination_domain_info(&self) -> &::std::option::Option<crate::types::DomainInformation> {
        self.inner.get_destination_domain_info()
    }
    /// <p>Specifies the connection alias that will be used by the customer for this connection.</p>
    pub fn connection_alias(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connection_alias(input.into());
        self
    }
    /// <p>Specifies the connection alias that will be used by the customer for this connection.</p>
    pub fn set_connection_alias(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_connection_alias(input);
        self
    }
    /// <p>Specifies the connection alias that will be used by the customer for this connection.</p>
    pub fn get_connection_alias(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_connection_alias()
    }
}
