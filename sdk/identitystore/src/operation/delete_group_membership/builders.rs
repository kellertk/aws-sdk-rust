// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_group_membership::_delete_group_membership_output::DeleteGroupMembershipOutputBuilder;

pub use crate::operation::delete_group_membership::_delete_group_membership_input::DeleteGroupMembershipInputBuilder;

impl DeleteGroupMembershipInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_group_membership::DeleteGroupMembershipOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_group_membership::DeleteGroupMembershipError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_group_membership();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteGroupMembership`.
///
/// <p>Delete a membership within a group given <code>MembershipId</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteGroupMembershipFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_group_membership::builders::DeleteGroupMembershipInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_group_membership::DeleteGroupMembershipOutput,
        crate::operation::delete_group_membership::DeleteGroupMembershipError,
    > for DeleteGroupMembershipFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_group_membership::DeleteGroupMembershipOutput,
            crate::operation::delete_group_membership::DeleteGroupMembershipError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteGroupMembershipFluentBuilder {
    /// Creates a new `DeleteGroupMembership`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteGroupMembership as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_group_membership::builders::DeleteGroupMembershipInputBuilder {
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
        crate::operation::delete_group_membership::DeleteGroupMembershipOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_group_membership::DeleteGroupMembershipError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_group_membership::DeleteGroupMembership::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_group_membership::DeleteGroupMembership::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_group_membership::DeleteGroupMembershipOutput,
        crate::operation::delete_group_membership::DeleteGroupMembershipError,
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
    /// <p>The globally unique identifier for the identity store.</p>
    pub fn identity_store_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identity_store_id(input.into());
        self
    }
    /// <p>The globally unique identifier for the identity store.</p>
    pub fn set_identity_store_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identity_store_id(input);
        self
    }
    /// <p>The globally unique identifier for the identity store.</p>
    pub fn get_identity_store_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_identity_store_id()
    }
    /// <p>The identifier for a <code>GroupMembership</code> in an identity store.</p>
    pub fn membership_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.membership_id(input.into());
        self
    }
    /// <p>The identifier for a <code>GroupMembership</code> in an identity store.</p>
    pub fn set_membership_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_membership_id(input);
        self
    }
    /// <p>The identifier for a <code>GroupMembership</code> in an identity store.</p>
    pub fn get_membership_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_membership_id()
    }
}
