// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_resolver_dnssec_configs::_list_resolver_dnssec_configs_output::ListResolverDnssecConfigsOutputBuilder;

pub use crate::operation::list_resolver_dnssec_configs::_list_resolver_dnssec_configs_input::ListResolverDnssecConfigsInputBuilder;

impl ListResolverDnssecConfigsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_resolver_dnssec_configs::ListResolverDnssecConfigsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_resolver_dnssec_configs::ListResolverDnssecConfigsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_resolver_dnssec_configs();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListResolverDnssecConfigs`.
///
/// <p>Lists the configurations for DNSSEC validation that are associated with the current Amazon Web Services account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListResolverDnssecConfigsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_resolver_dnssec_configs::builders::ListResolverDnssecConfigsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_resolver_dnssec_configs::ListResolverDnssecConfigsOutput,
        crate::operation::list_resolver_dnssec_configs::ListResolverDnssecConfigsError,
    > for ListResolverDnssecConfigsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_resolver_dnssec_configs::ListResolverDnssecConfigsOutput,
            crate::operation::list_resolver_dnssec_configs::ListResolverDnssecConfigsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListResolverDnssecConfigsFluentBuilder {
    /// Creates a new `ListResolverDnssecConfigs`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListResolverDnssecConfigs as a reference.
    pub fn as_input(&self) -> &crate::operation::list_resolver_dnssec_configs::builders::ListResolverDnssecConfigsInputBuilder {
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
        crate::operation::list_resolver_dnssec_configs::ListResolverDnssecConfigsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_resolver_dnssec_configs::ListResolverDnssecConfigsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_resolver_dnssec_configs::ListResolverDnssecConfigs::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_resolver_dnssec_configs::ListResolverDnssecConfigs::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_resolver_dnssec_configs::ListResolverDnssecConfigsOutput,
        crate::operation::list_resolver_dnssec_configs::ListResolverDnssecConfigsError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_resolver_dnssec_configs::paginator::ListResolverDnssecConfigsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_resolver_dnssec_configs::paginator::ListResolverDnssecConfigsPaginator {
        crate::operation::list_resolver_dnssec_configs::paginator::ListResolverDnssecConfigsPaginator::new(self.handle, self.inner)
    }
    /// <p><i>Optional</i>: An integer that specifies the maximum number of DNSSEC configuration results that you want Amazon Route 53 to return. If you don't specify a value for <code>MaxResults</code>, Route 53 returns up to 100 configuration per page.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p><i>Optional</i>: An integer that specifies the maximum number of DNSSEC configuration results that you want Amazon Route 53 to return. If you don't specify a value for <code>MaxResults</code>, Route 53 returns up to 100 configuration per page.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p><i>Optional</i>: An integer that specifies the maximum number of DNSSEC configuration results that you want Amazon Route 53 to return. If you don't specify a value for <code>MaxResults</code>, Route 53 returns up to 100 configuration per page.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>(Optional) If the current Amazon Web Services account has more than <code>MaxResults</code> DNSSEC configurations, use <code>NextToken</code> to get the second and subsequent pages of results.</p>
    /// <p>For the first <code>ListResolverDnssecConfigs</code> request, omit this value.</p>
    /// <p>For the second and subsequent requests, get the value of <code>NextToken</code> from the previous response and specify that value for <code>NextToken</code> in the request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>(Optional) If the current Amazon Web Services account has more than <code>MaxResults</code> DNSSEC configurations, use <code>NextToken</code> to get the second and subsequent pages of results.</p>
    /// <p>For the first <code>ListResolverDnssecConfigs</code> request, omit this value.</p>
    /// <p>For the second and subsequent requests, get the value of <code>NextToken</code> from the previous response and specify that value for <code>NextToken</code> in the request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>(Optional) If the current Amazon Web Services account has more than <code>MaxResults</code> DNSSEC configurations, use <code>NextToken</code> to get the second and subsequent pages of results.</p>
    /// <p>For the first <code>ListResolverDnssecConfigs</code> request, omit this value.</p>
    /// <p>For the second and subsequent requests, get the value of <code>NextToken</code> from the previous response and specify that value for <code>NextToken</code> in the request.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>An optional specification to return a subset of objects.</p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>An optional specification to return a subset of objects.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>An optional specification to return a subset of objects.</p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filters()
    }
}
