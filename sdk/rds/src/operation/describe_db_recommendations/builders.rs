// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_db_recommendations::_describe_db_recommendations_output::DescribeDbRecommendationsOutputBuilder;

pub use crate::operation::describe_db_recommendations::_describe_db_recommendations_input::DescribeDbRecommendationsInputBuilder;

impl DescribeDbRecommendationsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_db_recommendations::DescribeDbRecommendationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_db_recommendations::DescribeDBRecommendationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_db_recommendations();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeDBRecommendations`.
///
/// <p>Describes the recommendations to resolve the issues for your DB instances, DB clusters, and DB parameter groups.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeDBRecommendationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_db_recommendations::builders::DescribeDbRecommendationsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_db_recommendations::DescribeDbRecommendationsOutput,
        crate::operation::describe_db_recommendations::DescribeDBRecommendationsError,
    > for DescribeDBRecommendationsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_db_recommendations::DescribeDbRecommendationsOutput,
            crate::operation::describe_db_recommendations::DescribeDBRecommendationsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeDBRecommendationsFluentBuilder {
    /// Creates a new `DescribeDBRecommendations`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeDBRecommendations as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_db_recommendations::builders::DescribeDbRecommendationsInputBuilder {
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
        crate::operation::describe_db_recommendations::DescribeDbRecommendationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_db_recommendations::DescribeDBRecommendationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_db_recommendations::DescribeDBRecommendations::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_db_recommendations::DescribeDBRecommendations::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_db_recommendations::DescribeDbRecommendationsOutput,
        crate::operation::describe_db_recommendations::DescribeDBRecommendationsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_db_recommendations::paginator::DescribeDbRecommendationsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_db_recommendations::paginator::DescribeDbRecommendationsPaginator {
        crate::operation::describe_db_recommendations::paginator::DescribeDbRecommendationsPaginator::new(self.handle, self.inner)
    }
    /// <p>A filter to include only the recommendations that were updated after this specified time.</p>
    pub fn last_updated_after(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.last_updated_after(input);
        self
    }
    /// <p>A filter to include only the recommendations that were updated after this specified time.</p>
    pub fn set_last_updated_after(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_last_updated_after(input);
        self
    }
    /// <p>A filter to include only the recommendations that were updated after this specified time.</p>
    pub fn get_last_updated_after(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_last_updated_after()
    }
    /// <p>A filter to include only the recommendations that were updated before this specified time.</p>
    pub fn last_updated_before(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.last_updated_before(input);
        self
    }
    /// <p>A filter to include only the recommendations that were updated before this specified time.</p>
    pub fn set_last_updated_before(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_last_updated_before(input);
        self
    }
    /// <p>A filter to include only the recommendations that were updated before this specified time.</p>
    pub fn get_last_updated_before(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_last_updated_before()
    }
    /// <p>The language that you choose to return the list of recommendations.</p>
    /// <p>Valid values:</p>
    /// <ul>
    /// <li>
    /// <p><code>en</code></p></li>
    /// <li>
    /// <p><code>en_UK</code></p></li>
    /// <li>
    /// <p><code>de</code></p></li>
    /// <li>
    /// <p><code>es</code></p></li>
    /// <li>
    /// <p><code>fr</code></p></li>
    /// <li>
    /// <p><code>id</code></p></li>
    /// <li>
    /// <p><code>it</code></p></li>
    /// <li>
    /// <p><code>ja</code></p></li>
    /// <li>
    /// <p><code>ko</code></p></li>
    /// <li>
    /// <p><code>pt_BR</code></p></li>
    /// <li>
    /// <p><code>zh_TW</code></p></li>
    /// <li>
    /// <p><code>zh_CN</code></p></li>
    /// </ul>
    pub fn locale(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.locale(input.into());
        self
    }
    /// <p>The language that you choose to return the list of recommendations.</p>
    /// <p>Valid values:</p>
    /// <ul>
    /// <li>
    /// <p><code>en</code></p></li>
    /// <li>
    /// <p><code>en_UK</code></p></li>
    /// <li>
    /// <p><code>de</code></p></li>
    /// <li>
    /// <p><code>es</code></p></li>
    /// <li>
    /// <p><code>fr</code></p></li>
    /// <li>
    /// <p><code>id</code></p></li>
    /// <li>
    /// <p><code>it</code></p></li>
    /// <li>
    /// <p><code>ja</code></p></li>
    /// <li>
    /// <p><code>ko</code></p></li>
    /// <li>
    /// <p><code>pt_BR</code></p></li>
    /// <li>
    /// <p><code>zh_TW</code></p></li>
    /// <li>
    /// <p><code>zh_CN</code></p></li>
    /// </ul>
    pub fn set_locale(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_locale(input);
        self
    }
    /// <p>The language that you choose to return the list of recommendations.</p>
    /// <p>Valid values:</p>
    /// <ul>
    /// <li>
    /// <p><code>en</code></p></li>
    /// <li>
    /// <p><code>en_UK</code></p></li>
    /// <li>
    /// <p><code>de</code></p></li>
    /// <li>
    /// <p><code>es</code></p></li>
    /// <li>
    /// <p><code>fr</code></p></li>
    /// <li>
    /// <p><code>id</code></p></li>
    /// <li>
    /// <p><code>it</code></p></li>
    /// <li>
    /// <p><code>ja</code></p></li>
    /// <li>
    /// <p><code>ko</code></p></li>
    /// <li>
    /// <p><code>pt_BR</code></p></li>
    /// <li>
    /// <p><code>zh_TW</code></p></li>
    /// <li>
    /// <p><code>zh_CN</code></p></li>
    /// </ul>
    pub fn get_locale(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_locale()
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>A filter that specifies one or more recommendations to describe.</p>
    /// <p>Supported Filters:</p>
    /// <ul>
    /// <li>
    /// <p><code>recommendation-id</code> - Accepts a list of recommendation identifiers. The results list only includes the recommendations whose identifier is one of the specified filter values.</p></li>
    /// <li>
    /// <p><code>status</code> - Accepts a list of recommendation statuses.</p>
    /// <p>Valid values:</p>
    /// <ul>
    /// <li>
    /// <p><code>active</code> - The recommendations which are ready for you to apply.</p></li>
    /// <li>
    /// <p><code>pending</code> - The applied or scheduled recommendations which are in progress.</p></li>
    /// <li>
    /// <p><code>resolved</code> - The recommendations which are completed.</p></li>
    /// <li>
    /// <p><code>dismissed</code> - The recommendations that you dismissed.</p></li>
    /// </ul>
    /// <p>The results list only includes the recommendations whose status is one of the specified filter values.</p></li>
    /// <li>
    /// <p><code>severity</code> - Accepts a list of recommendation severities. The results list only includes the recommendations whose severity is one of the specified filter values.</p>
    /// <p>Valid values:</p>
    /// <ul>
    /// <li>
    /// <p><code>high</code></p></li>
    /// <li>
    /// <p><code>medium</code></p></li>
    /// <li>
    /// <p><code>low</code></p></li>
    /// <li>
    /// <p><code>informational</code></p></li>
    /// </ul></li>
    /// <li>
    /// <p><code>type-id</code> - Accepts a list of recommendation type identifiers. The results list only includes the recommendations whose type is one of the specified filter values.</p></li>
    /// <li>
    /// <p><code>dbi-resource-id</code> - Accepts a list of database resource identifiers. The results list only includes the recommendations that generated for the specified databases.</p></li>
    /// <li>
    /// <p><code>cluster-resource-id</code> - Accepts a list of cluster resource identifiers. The results list only includes the recommendations that generated for the specified clusters.</p></li>
    /// <li>
    /// <p><code>pg-arn</code> - Accepts a list of parameter group ARNs. The results list only includes the recommendations that generated for the specified parameter groups.</p></li>
    /// <li>
    /// <p><code>cluster-pg-arn</code> - Accepts a list of cluster parameter group ARNs. The results list only includes the recommendations that generated for the specified cluster parameter groups.</p></li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>A filter that specifies one or more recommendations to describe.</p>
    /// <p>Supported Filters:</p>
    /// <ul>
    /// <li>
    /// <p><code>recommendation-id</code> - Accepts a list of recommendation identifiers. The results list only includes the recommendations whose identifier is one of the specified filter values.</p></li>
    /// <li>
    /// <p><code>status</code> - Accepts a list of recommendation statuses.</p>
    /// <p>Valid values:</p>
    /// <ul>
    /// <li>
    /// <p><code>active</code> - The recommendations which are ready for you to apply.</p></li>
    /// <li>
    /// <p><code>pending</code> - The applied or scheduled recommendations which are in progress.</p></li>
    /// <li>
    /// <p><code>resolved</code> - The recommendations which are completed.</p></li>
    /// <li>
    /// <p><code>dismissed</code> - The recommendations that you dismissed.</p></li>
    /// </ul>
    /// <p>The results list only includes the recommendations whose status is one of the specified filter values.</p></li>
    /// <li>
    /// <p><code>severity</code> - Accepts a list of recommendation severities. The results list only includes the recommendations whose severity is one of the specified filter values.</p>
    /// <p>Valid values:</p>
    /// <ul>
    /// <li>
    /// <p><code>high</code></p></li>
    /// <li>
    /// <p><code>medium</code></p></li>
    /// <li>
    /// <p><code>low</code></p></li>
    /// <li>
    /// <p><code>informational</code></p></li>
    /// </ul></li>
    /// <li>
    /// <p><code>type-id</code> - Accepts a list of recommendation type identifiers. The results list only includes the recommendations whose type is one of the specified filter values.</p></li>
    /// <li>
    /// <p><code>dbi-resource-id</code> - Accepts a list of database resource identifiers. The results list only includes the recommendations that generated for the specified databases.</p></li>
    /// <li>
    /// <p><code>cluster-resource-id</code> - Accepts a list of cluster resource identifiers. The results list only includes the recommendations that generated for the specified clusters.</p></li>
    /// <li>
    /// <p><code>pg-arn</code> - Accepts a list of parameter group ARNs. The results list only includes the recommendations that generated for the specified parameter groups.</p></li>
    /// <li>
    /// <p><code>cluster-pg-arn</code> - Accepts a list of cluster parameter group ARNs. The results list only includes the recommendations that generated for the specified cluster parameter groups.</p></li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>A filter that specifies one or more recommendations to describe.</p>
    /// <p>Supported Filters:</p>
    /// <ul>
    /// <li>
    /// <p><code>recommendation-id</code> - Accepts a list of recommendation identifiers. The results list only includes the recommendations whose identifier is one of the specified filter values.</p></li>
    /// <li>
    /// <p><code>status</code> - Accepts a list of recommendation statuses.</p>
    /// <p>Valid values:</p>
    /// <ul>
    /// <li>
    /// <p><code>active</code> - The recommendations which are ready for you to apply.</p></li>
    /// <li>
    /// <p><code>pending</code> - The applied or scheduled recommendations which are in progress.</p></li>
    /// <li>
    /// <p><code>resolved</code> - The recommendations which are completed.</p></li>
    /// <li>
    /// <p><code>dismissed</code> - The recommendations that you dismissed.</p></li>
    /// </ul>
    /// <p>The results list only includes the recommendations whose status is one of the specified filter values.</p></li>
    /// <li>
    /// <p><code>severity</code> - Accepts a list of recommendation severities. The results list only includes the recommendations whose severity is one of the specified filter values.</p>
    /// <p>Valid values:</p>
    /// <ul>
    /// <li>
    /// <p><code>high</code></p></li>
    /// <li>
    /// <p><code>medium</code></p></li>
    /// <li>
    /// <p><code>low</code></p></li>
    /// <li>
    /// <p><code>informational</code></p></li>
    /// </ul></li>
    /// <li>
    /// <p><code>type-id</code> - Accepts a list of recommendation type identifiers. The results list only includes the recommendations whose type is one of the specified filter values.</p></li>
    /// <li>
    /// <p><code>dbi-resource-id</code> - Accepts a list of database resource identifiers. The results list only includes the recommendations that generated for the specified databases.</p></li>
    /// <li>
    /// <p><code>cluster-resource-id</code> - Accepts a list of cluster resource identifiers. The results list only includes the recommendations that generated for the specified clusters.</p></li>
    /// <li>
    /// <p><code>pg-arn</code> - Accepts a list of parameter group ARNs. The results list only includes the recommendations that generated for the specified parameter groups.</p></li>
    /// <li>
    /// <p><code>cluster-pg-arn</code> - Accepts a list of cluster parameter group ARNs. The results list only includes the recommendations that generated for the specified cluster parameter groups.</p></li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filters()
    }
    /// <p>The maximum number of recommendations to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that you can retrieve the remaining results.</p>
    pub fn max_records(mut self, input: i32) -> Self {
        self.inner = self.inner.max_records(input);
        self
    }
    /// <p>The maximum number of recommendations to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that you can retrieve the remaining results.</p>
    pub fn set_max_records(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_records(input);
        self
    }
    /// <p>The maximum number of recommendations to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that you can retrieve the remaining results.</p>
    pub fn get_max_records(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_records()
    }
    /// <p>An optional pagination token provided by a previous <code>DescribeDBRecommendations</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>An optional pagination token provided by a previous <code>DescribeDBRecommendations</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>An optional pagination token provided by a previous <code>DescribeDBRecommendations</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_marker()
    }
}
