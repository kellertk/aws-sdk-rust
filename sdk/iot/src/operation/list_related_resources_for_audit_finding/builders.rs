// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_related_resources_for_audit_finding::_list_related_resources_for_audit_finding_output::ListRelatedResourcesForAuditFindingOutputBuilder;

pub use crate::operation::list_related_resources_for_audit_finding::_list_related_resources_for_audit_finding_input::ListRelatedResourcesForAuditFindingInputBuilder;

impl ListRelatedResourcesForAuditFindingInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_related_resources_for_audit_finding::ListRelatedResourcesForAuditFindingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_related_resources_for_audit_finding::ListRelatedResourcesForAuditFindingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_related_resources_for_audit_finding();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListRelatedResourcesForAuditFinding`.
///
/// <p>The related resources of an Audit finding. The following resources can be returned from calling this API:</p>
/// <ul>
/// <li>
/// <p>DEVICE_CERTIFICATE</p></li>
/// <li>
/// <p>CA_CERTIFICATE</p></li>
/// <li>
/// <p>IOT_POLICY</p></li>
/// <li>
/// <p>COGNITO_IDENTITY_POOL</p></li>
/// <li>
/// <p>CLIENT_ID</p></li>
/// <li>
/// <p>ACCOUNT_SETTINGS</p></li>
/// <li>
/// <p>ROLE_ALIAS</p></li>
/// <li>
/// <p>IAM_ROLE</p></li>
/// <li>
/// <p>ISSUER_CERTIFICATE</p></li>
/// </ul><note>
/// <p>This API is similar to DescribeAuditFinding's <a href="https://docs.aws.amazon.com/iot/latest/apireference/API_DescribeAuditFinding.html">RelatedResources</a> but provides pagination and is not limited to 10 resources. When calling <a href="https://docs.aws.amazon.com/iot/latest/apireference/API_DescribeAuditFinding.html">DescribeAuditFinding</a> for the intermediate CA revoked for active device certificates check, RelatedResources will not be populated. You must use this API, ListRelatedResourcesForAuditFinding, to list the certificates.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListRelatedResourcesForAuditFindingFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_related_resources_for_audit_finding::builders::ListRelatedResourcesForAuditFindingInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_related_resources_for_audit_finding::ListRelatedResourcesForAuditFindingOutput,
        crate::operation::list_related_resources_for_audit_finding::ListRelatedResourcesForAuditFindingError,
    > for ListRelatedResourcesForAuditFindingFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_related_resources_for_audit_finding::ListRelatedResourcesForAuditFindingOutput,
            crate::operation::list_related_resources_for_audit_finding::ListRelatedResourcesForAuditFindingError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListRelatedResourcesForAuditFindingFluentBuilder {
    /// Creates a new `ListRelatedResourcesForAuditFinding`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListRelatedResourcesForAuditFinding as a reference.
    pub fn as_input(&self) -> &crate::operation::list_related_resources_for_audit_finding::builders::ListRelatedResourcesForAuditFindingInputBuilder {
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
        crate::operation::list_related_resources_for_audit_finding::ListRelatedResourcesForAuditFindingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_related_resources_for_audit_finding::ListRelatedResourcesForAuditFindingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::list_related_resources_for_audit_finding::ListRelatedResourcesForAuditFinding::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::list_related_resources_for_audit_finding::ListRelatedResourcesForAuditFinding::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_related_resources_for_audit_finding::ListRelatedResourcesForAuditFindingOutput,
        crate::operation::list_related_resources_for_audit_finding::ListRelatedResourcesForAuditFindingError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_related_resources_for_audit_finding::paginator::ListRelatedResourcesForAuditFindingPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_related_resources_for_audit_finding::paginator::ListRelatedResourcesForAuditFindingPaginator {
        crate::operation::list_related_resources_for_audit_finding::paginator::ListRelatedResourcesForAuditFindingPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The finding Id.</p>
    pub fn finding_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.finding_id(input.into());
        self
    }
    /// <p>The finding Id.</p>
    pub fn set_finding_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_finding_id(input);
        self
    }
    /// <p>The finding Id.</p>
    pub fn get_finding_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_finding_id()
    }
    /// <p>A token that can be used to retrieve the next set of results, or <code>null</code> if there are no additional results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token that can be used to retrieve the next set of results, or <code>null</code> if there are no additional results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A token that can be used to retrieve the next set of results, or <code>null</code> if there are no additional results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return at one time.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return at one time.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return at one time.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
