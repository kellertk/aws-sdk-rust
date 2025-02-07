// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::add_profile_permission::_add_profile_permission_output::AddProfilePermissionOutputBuilder;

pub use crate::operation::add_profile_permission::_add_profile_permission_input::AddProfilePermissionInputBuilder;

impl AddProfilePermissionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::add_profile_permission::AddProfilePermissionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::add_profile_permission::AddProfilePermissionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.add_profile_permission();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AddProfilePermission`.
///
/// <p>Adds cross-account permissions to a signing profile.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AddProfilePermissionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::add_profile_permission::builders::AddProfilePermissionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::add_profile_permission::AddProfilePermissionOutput,
        crate::operation::add_profile_permission::AddProfilePermissionError,
    > for AddProfilePermissionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::add_profile_permission::AddProfilePermissionOutput,
            crate::operation::add_profile_permission::AddProfilePermissionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AddProfilePermissionFluentBuilder {
    /// Creates a new `AddProfilePermission`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AddProfilePermission as a reference.
    pub fn as_input(&self) -> &crate::operation::add_profile_permission::builders::AddProfilePermissionInputBuilder {
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
        crate::operation::add_profile_permission::AddProfilePermissionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::add_profile_permission::AddProfilePermissionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::add_profile_permission::AddProfilePermission::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::add_profile_permission::AddProfilePermission::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::add_profile_permission::AddProfilePermissionOutput,
        crate::operation::add_profile_permission::AddProfilePermissionError,
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
    /// <p>The human-readable name of the signing profile.</p>
    pub fn profile_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.profile_name(input.into());
        self
    }
    /// <p>The human-readable name of the signing profile.</p>
    pub fn set_profile_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_profile_name(input);
        self
    }
    /// <p>The human-readable name of the signing profile.</p>
    pub fn get_profile_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_profile_name()
    }
    /// <p>The version of the signing profile.</p>
    pub fn profile_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.profile_version(input.into());
        self
    }
    /// <p>The version of the signing profile.</p>
    pub fn set_profile_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_profile_version(input);
        self
    }
    /// <p>The version of the signing profile.</p>
    pub fn get_profile_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_profile_version()
    }
    /// <p>The AWS Signer action permitted as part of cross-account permissions.</p>
    pub fn action(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.action(input.into());
        self
    }
    /// <p>The AWS Signer action permitted as part of cross-account permissions.</p>
    pub fn set_action(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_action(input);
        self
    }
    /// <p>The AWS Signer action permitted as part of cross-account permissions.</p>
    pub fn get_action(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_action()
    }
    /// <p>The AWS principal receiving cross-account permissions. This may be an IAM role or another AWS account ID.</p>
    pub fn principal(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.principal(input.into());
        self
    }
    /// <p>The AWS principal receiving cross-account permissions. This may be an IAM role or another AWS account ID.</p>
    pub fn set_principal(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_principal(input);
        self
    }
    /// <p>The AWS principal receiving cross-account permissions. This may be an IAM role or another AWS account ID.</p>
    pub fn get_principal(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_principal()
    }
    /// <p>A unique identifier for the current profile revision.</p>
    pub fn revision_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.revision_id(input.into());
        self
    }
    /// <p>A unique identifier for the current profile revision.</p>
    pub fn set_revision_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_revision_id(input);
        self
    }
    /// <p>A unique identifier for the current profile revision.</p>
    pub fn get_revision_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_revision_id()
    }
    /// <p>A unique identifier for the cross-account permission statement.</p>
    pub fn statement_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.statement_id(input.into());
        self
    }
    /// <p>A unique identifier for the cross-account permission statement.</p>
    pub fn set_statement_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_statement_id(input);
        self
    }
    /// <p>A unique identifier for the cross-account permission statement.</p>
    pub fn get_statement_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_statement_id()
    }
}
