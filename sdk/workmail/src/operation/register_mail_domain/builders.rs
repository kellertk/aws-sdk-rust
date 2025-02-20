// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::register_mail_domain::_register_mail_domain_output::RegisterMailDomainOutputBuilder;

pub use crate::operation::register_mail_domain::_register_mail_domain_input::RegisterMailDomainInputBuilder;

impl RegisterMailDomainInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::register_mail_domain::RegisterMailDomainOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::register_mail_domain::RegisterMailDomainError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.register_mail_domain();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RegisterMailDomain`.
///
/// <p>Registers a new domain in WorkMail and SES, and configures it for use by WorkMail. Emails received by SES for this domain are routed to the specified WorkMail organization, and WorkMail has permanent permission to use the specified domain for sending your users' emails.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RegisterMailDomainFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::register_mail_domain::builders::RegisterMailDomainInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::register_mail_domain::RegisterMailDomainOutput,
        crate::operation::register_mail_domain::RegisterMailDomainError,
    > for RegisterMailDomainFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::register_mail_domain::RegisterMailDomainOutput,
            crate::operation::register_mail_domain::RegisterMailDomainError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RegisterMailDomainFluentBuilder {
    /// Creates a new `RegisterMailDomain`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RegisterMailDomain as a reference.
    pub fn as_input(&self) -> &crate::operation::register_mail_domain::builders::RegisterMailDomainInputBuilder {
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
        crate::operation::register_mail_domain::RegisterMailDomainOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::register_mail_domain::RegisterMailDomainError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::register_mail_domain::RegisterMailDomain::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::register_mail_domain::RegisterMailDomain::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::register_mail_domain::RegisterMailDomainOutput,
        crate::operation::register_mail_domain::RegisterMailDomainError,
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
    /// <p>Idempotency token used when retrying requests.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Idempotency token used when retrying requests.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Idempotency token used when retrying requests.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>The WorkMail organization under which you're creating the domain.</p>
    pub fn organization_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.organization_id(input.into());
        self
    }
    /// <p>The WorkMail organization under which you're creating the domain.</p>
    pub fn set_organization_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_organization_id(input);
        self
    }
    /// <p>The WorkMail organization under which you're creating the domain.</p>
    pub fn get_organization_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_organization_id()
    }
    /// <p>The name of the mail domain to create in WorkMail and SES.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The name of the mail domain to create in WorkMail and SES.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The name of the mail domain to create in WorkMail and SES.</p>
    pub fn get_domain_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_name()
    }
}
