// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_budgets_for_resource::_list_budgets_for_resource_output::ListBudgetsForResourceOutputBuilder;

pub use crate::operation::list_budgets_for_resource::_list_budgets_for_resource_input::ListBudgetsForResourceInputBuilder;

impl ListBudgetsForResourceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_budgets_for_resource::ListBudgetsForResourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_budgets_for_resource::ListBudgetsForResourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_budgets_for_resource();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListBudgetsForResource`.
///
/// <p>Lists all the budgets associated to the specified resource.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListBudgetsForResourceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_budgets_for_resource::builders::ListBudgetsForResourceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_budgets_for_resource::ListBudgetsForResourceOutput,
        crate::operation::list_budgets_for_resource::ListBudgetsForResourceError,
    > for ListBudgetsForResourceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_budgets_for_resource::ListBudgetsForResourceOutput,
            crate::operation::list_budgets_for_resource::ListBudgetsForResourceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListBudgetsForResourceFluentBuilder {
    /// Creates a new `ListBudgetsForResource`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListBudgetsForResource as a reference.
    pub fn as_input(&self) -> &crate::operation::list_budgets_for_resource::builders::ListBudgetsForResourceInputBuilder {
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
        crate::operation::list_budgets_for_resource::ListBudgetsForResourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_budgets_for_resource::ListBudgetsForResourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_budgets_for_resource::ListBudgetsForResource::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_budgets_for_resource::ListBudgetsForResource::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_budgets_for_resource::ListBudgetsForResourceOutput,
        crate::operation::list_budgets_for_resource::ListBudgetsForResourceError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_budgets_for_resource::paginator::ListBudgetsForResourcePaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_budgets_for_resource::paginator::ListBudgetsForResourcePaginator {
        crate::operation::list_budgets_for_resource::paginator::ListBudgetsForResourcePaginator::new(self.handle, self.inner)
    }
    /// <p>The language code.</p>
    /// <ul>
    /// <li>
    /// <p><code>jp</code> - Japanese</p></li>
    /// <li>
    /// <p><code>zh</code> - Chinese</p></li>
    /// </ul>
    pub fn accept_language(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.accept_language(input.into());
        self
    }
    /// <p>The language code.</p>
    /// <ul>
    /// <li>
    /// <p><code>jp</code> - Japanese</p></li>
    /// <li>
    /// <p><code>zh</code> - Chinese</p></li>
    /// </ul>
    pub fn set_accept_language(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_accept_language(input);
        self
    }
    /// <p>The language code.</p>
    /// <ul>
    /// <li>
    /// <p><code>jp</code> - Japanese</p></li>
    /// <li>
    /// <p><code>zh</code> - Chinese</p></li>
    /// </ul>
    pub fn get_accept_language(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_accept_language()
    }
    /// <p>The resource identifier.</p>
    pub fn resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_id(input.into());
        self
    }
    /// <p>The resource identifier.</p>
    pub fn set_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_id(input);
        self
    }
    /// <p>The resource identifier.</p>
    pub fn get_resource_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_id()
    }
    /// <p>The maximum number of items to return with this call.</p>
    pub fn page_size(mut self, input: i32) -> Self {
        self.inner = self.inner.page_size(input);
        self
    }
    /// <p>The maximum number of items to return with this call.</p>
    pub fn set_page_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_page_size(input);
        self
    }
    /// <p>The maximum number of items to return with this call.</p>
    pub fn get_page_size(&self) -> &::std::option::Option<i32> {
        self.inner.get_page_size()
    }
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    pub fn page_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.page_token(input.into());
        self
    }
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    pub fn set_page_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_page_token(input);
        self
    }
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    pub fn get_page_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_page_token()
    }
}
