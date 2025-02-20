// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_home_region_controls::_describe_home_region_controls_output::DescribeHomeRegionControlsOutputBuilder;

pub use crate::operation::describe_home_region_controls::_describe_home_region_controls_input::DescribeHomeRegionControlsInputBuilder;

impl DescribeHomeRegionControlsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_home_region_controls::DescribeHomeRegionControlsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_home_region_controls::DescribeHomeRegionControlsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_home_region_controls();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeHomeRegionControls`.
///
/// <p>This API permits filtering on the <code>ControlId</code> and <code>HomeRegion</code> fields.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeHomeRegionControlsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_home_region_controls::builders::DescribeHomeRegionControlsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_home_region_controls::DescribeHomeRegionControlsOutput,
        crate::operation::describe_home_region_controls::DescribeHomeRegionControlsError,
    > for DescribeHomeRegionControlsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_home_region_controls::DescribeHomeRegionControlsOutput,
            crate::operation::describe_home_region_controls::DescribeHomeRegionControlsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeHomeRegionControlsFluentBuilder {
    /// Creates a new `DescribeHomeRegionControls`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeHomeRegionControls as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_home_region_controls::builders::DescribeHomeRegionControlsInputBuilder {
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
        crate::operation::describe_home_region_controls::DescribeHomeRegionControlsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_home_region_controls::DescribeHomeRegionControlsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_home_region_controls::DescribeHomeRegionControls::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_home_region_controls::DescribeHomeRegionControls::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_home_region_controls::DescribeHomeRegionControlsOutput,
        crate::operation::describe_home_region_controls::DescribeHomeRegionControlsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_home_region_controls::paginator::DescribeHomeRegionControlsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_home_region_controls::paginator::DescribeHomeRegionControlsPaginator {
        crate::operation::describe_home_region_controls::paginator::DescribeHomeRegionControlsPaginator::new(self.handle, self.inner)
    }
    /// <p>The <code>ControlID</code> is a unique identifier string of your <code>HomeRegionControl</code> object.</p>
    pub fn control_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.control_id(input.into());
        self
    }
    /// <p>The <code>ControlID</code> is a unique identifier string of your <code>HomeRegionControl</code> object.</p>
    pub fn set_control_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_control_id(input);
        self
    }
    /// <p>The <code>ControlID</code> is a unique identifier string of your <code>HomeRegionControl</code> object.</p>
    pub fn get_control_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_control_id()
    }
    /// <p>The name of the home region you'd like to view.</p>
    pub fn home_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.home_region(input.into());
        self
    }
    /// <p>The name of the home region you'd like to view.</p>
    pub fn set_home_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_home_region(input);
        self
    }
    /// <p>The name of the home region you'd like to view.</p>
    pub fn get_home_region(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_home_region()
    }
    /// <p>The target parameter specifies the identifier to which the home region is applied, which is always of type <code>ACCOUNT</code>. It applies the home region to the current <code>ACCOUNT</code>.</p>
    pub fn target(mut self, input: crate::types::Target) -> Self {
        self.inner = self.inner.target(input);
        self
    }
    /// <p>The target parameter specifies the identifier to which the home region is applied, which is always of type <code>ACCOUNT</code>. It applies the home region to the current <code>ACCOUNT</code>.</p>
    pub fn set_target(mut self, input: ::std::option::Option<crate::types::Target>) -> Self {
        self.inner = self.inner.set_target(input);
        self
    }
    /// <p>The target parameter specifies the identifier to which the home region is applied, which is always of type <code>ACCOUNT</code>. It applies the home region to the current <code>ACCOUNT</code>.</p>
    pub fn get_target(&self) -> &::std::option::Option<crate::types::Target> {
        self.inner.get_target()
    }
    /// <p>The maximum number of filtering results to display per page.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of filtering results to display per page.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of filtering results to display per page.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>If a <code>NextToken</code> was returned by a previous call, more results are available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If a <code>NextToken</code> was returned by a previous call, more results are available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>If a <code>NextToken</code> was returned by a previous call, more results are available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
