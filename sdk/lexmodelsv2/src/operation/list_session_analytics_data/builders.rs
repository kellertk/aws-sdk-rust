// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_session_analytics_data::_list_session_analytics_data_output::ListSessionAnalyticsDataOutputBuilder;

pub use crate::operation::list_session_analytics_data::_list_session_analytics_data_input::ListSessionAnalyticsDataInputBuilder;

impl ListSessionAnalyticsDataInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_session_analytics_data::ListSessionAnalyticsDataOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_session_analytics_data::ListSessionAnalyticsDataError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_session_analytics_data();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListSessionAnalyticsData`.
///
/// <p>Retrieves a list of metadata for individual user sessions with your bot. The <code>startDateTime</code> and <code>endDateTime</code> fields are required. These fields define a time range for which you want to retrieve results. Of the optional fields, you can organize the results in the following ways:</p>
/// <ul>
/// <li>
/// <p>Use the <code>filters</code> field to filter the results and the <code>sortBy</code> field to specify the values by which to sort the results.</p></li>
/// <li>
/// <p>Use the <code>maxResults</code> field to limit the number of results to return in a single response and the <code>nextToken</code> field to return the next batch of results if the response does not return the full set of results.</p></li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListSessionAnalyticsDataFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_session_analytics_data::builders::ListSessionAnalyticsDataInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_session_analytics_data::ListSessionAnalyticsDataOutput,
        crate::operation::list_session_analytics_data::ListSessionAnalyticsDataError,
    > for ListSessionAnalyticsDataFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_session_analytics_data::ListSessionAnalyticsDataOutput,
            crate::operation::list_session_analytics_data::ListSessionAnalyticsDataError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListSessionAnalyticsDataFluentBuilder {
    /// Creates a new `ListSessionAnalyticsData`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListSessionAnalyticsData as a reference.
    pub fn as_input(&self) -> &crate::operation::list_session_analytics_data::builders::ListSessionAnalyticsDataInputBuilder {
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
        crate::operation::list_session_analytics_data::ListSessionAnalyticsDataOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_session_analytics_data::ListSessionAnalyticsDataError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_session_analytics_data::ListSessionAnalyticsData::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_session_analytics_data::ListSessionAnalyticsData::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_session_analytics_data::ListSessionAnalyticsDataOutput,
        crate::operation::list_session_analytics_data::ListSessionAnalyticsDataError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_session_analytics_data::paginator::ListSessionAnalyticsDataPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_session_analytics_data::paginator::ListSessionAnalyticsDataPaginator {
        crate::operation::list_session_analytics_data::paginator::ListSessionAnalyticsDataPaginator::new(self.handle, self.inner)
    }
    /// <p>The identifier for the bot for which you want to retrieve session analytics.</p>
    pub fn bot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_id(input.into());
        self
    }
    /// <p>The identifier for the bot for which you want to retrieve session analytics.</p>
    pub fn set_bot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_id(input);
        self
    }
    /// <p>The identifier for the bot for which you want to retrieve session analytics.</p>
    pub fn get_bot_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bot_id()
    }
    /// <p>The date and time that marks the beginning of the range of time for which you want to see session analytics.</p>
    pub fn start_date_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_date_time(input);
        self
    }
    /// <p>The date and time that marks the beginning of the range of time for which you want to see session analytics.</p>
    pub fn set_start_date_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_start_date_time(input);
        self
    }
    /// <p>The date and time that marks the beginning of the range of time for which you want to see session analytics.</p>
    pub fn get_start_date_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_start_date_time()
    }
    /// <p>The date and time that marks the end of the range of time for which you want to see session analytics.</p>
    pub fn end_date_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_date_time(input);
        self
    }
    /// <p>The date and time that marks the end of the range of time for which you want to see session analytics.</p>
    pub fn set_end_date_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_end_date_time(input);
        self
    }
    /// <p>The date and time that marks the end of the range of time for which you want to see session analytics.</p>
    pub fn get_end_date_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_end_date_time()
    }
    /// <p>An object specifying the measure and method by which to sort the session analytics data.</p>
    pub fn sort_by(mut self, input: crate::types::SessionDataSortBy) -> Self {
        self.inner = self.inner.sort_by(input);
        self
    }
    /// <p>An object specifying the measure and method by which to sort the session analytics data.</p>
    pub fn set_sort_by(mut self, input: ::std::option::Option<crate::types::SessionDataSortBy>) -> Self {
        self.inner = self.inner.set_sort_by(input);
        self
    }
    /// <p>An object specifying the measure and method by which to sort the session analytics data.</p>
    pub fn get_sort_by(&self) -> &::std::option::Option<crate::types::SessionDataSortBy> {
        self.inner.get_sort_by()
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>A list of objects, each of which describes a condition by which you want to filter the results.</p>
    pub fn filters(mut self, input: crate::types::AnalyticsSessionFilter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>A list of objects, each of which describes a condition by which you want to filter the results.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AnalyticsSessionFilter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>A list of objects, each of which describes a condition by which you want to filter the results.</p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AnalyticsSessionFilter>> {
        self.inner.get_filters()
    }
    /// <p>The maximum number of results to return in each page of results. If there are fewer results than the maximum page size, only the actual number of results are returned.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in each page of results. If there are fewer results than the maximum page size, only the actual number of results are returned.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return in each page of results. If there are fewer results than the maximum page size, only the actual number of results are returned.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>If the response from the ListSessionAnalyticsData operation contains more results than specified in the maxResults parameter, a token is returned in the response.</p>
    /// <p>Use the returned token in the nextToken parameter of a ListSessionAnalyticsData request to return the next page of results. For a complete set of results, call the ListSessionAnalyticsData operation until the nextToken returned in the response is null.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If the response from the ListSessionAnalyticsData operation contains more results than specified in the maxResults parameter, a token is returned in the response.</p>
    /// <p>Use the returned token in the nextToken parameter of a ListSessionAnalyticsData request to return the next page of results. For a complete set of results, call the ListSessionAnalyticsData operation until the nextToken returned in the response is null.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>If the response from the ListSessionAnalyticsData operation contains more results than specified in the maxResults parameter, a token is returned in the response.</p>
    /// <p>Use the returned token in the nextToken parameter of a ListSessionAnalyticsData request to return the next page of results. For a complete set of results, call the ListSessionAnalyticsData operation until the nextToken returned in the response is null.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
