// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_image_permissions::_describe_image_permissions_output::DescribeImagePermissionsOutputBuilder;

pub use crate::operation::describe_image_permissions::_describe_image_permissions_input::DescribeImagePermissionsInputBuilder;

impl DescribeImagePermissionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_image_permissions::DescribeImagePermissionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_image_permissions::DescribeImagePermissionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_image_permissions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeImagePermissions`.
///
/// <p>Retrieves a list that describes the permissions for shared AWS account IDs on a private image that you own.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeImagePermissionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_image_permissions::builders::DescribeImagePermissionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_image_permissions::DescribeImagePermissionsOutput,
        crate::operation::describe_image_permissions::DescribeImagePermissionsError,
    > for DescribeImagePermissionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_image_permissions::DescribeImagePermissionsOutput,
            crate::operation::describe_image_permissions::DescribeImagePermissionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeImagePermissionsFluentBuilder {
    /// Creates a new `DescribeImagePermissions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeImagePermissions as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_image_permissions::builders::DescribeImagePermissionsInputBuilder {
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
        crate::operation::describe_image_permissions::DescribeImagePermissionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_image_permissions::DescribeImagePermissionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_image_permissions::DescribeImagePermissions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_image_permissions::DescribeImagePermissions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_image_permissions::DescribeImagePermissionsOutput,
        crate::operation::describe_image_permissions::DescribeImagePermissionsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_image_permissions::paginator::DescribeImagePermissionsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_image_permissions::paginator::DescribeImagePermissionsPaginator {
        crate::operation::describe_image_permissions::paginator::DescribeImagePermissionsPaginator::new(self.handle, self.inner)
    }
    /// <p>The name of the private image for which to describe permissions. The image must be one that you own.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the private image for which to describe permissions. The image must be one that you own.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the private image for which to describe permissions. The image must be one that you own.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The maximum size of each page of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum size of each page of results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum size of each page of results.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// Appends an item to `SharedAwsAccountIds`.
    ///
    /// To override the contents of this collection use [`set_shared_aws_account_ids`](Self::set_shared_aws_account_ids).
    ///
    /// <p>The 12-digit identifier of one or more AWS accounts with which the image is shared.</p>
    pub fn shared_aws_account_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.shared_aws_account_ids(input.into());
        self
    }
    /// <p>The 12-digit identifier of one or more AWS accounts with which the image is shared.</p>
    pub fn set_shared_aws_account_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_shared_aws_account_ids(input);
        self
    }
    /// <p>The 12-digit identifier of one or more AWS accounts with which the image is shared.</p>
    pub fn get_shared_aws_account_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_shared_aws_account_ids()
    }
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
