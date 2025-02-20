// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_certificate_provider::_create_certificate_provider_output::CreateCertificateProviderOutputBuilder;

pub use crate::operation::create_certificate_provider::_create_certificate_provider_input::CreateCertificateProviderInputBuilder;

impl CreateCertificateProviderInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_certificate_provider::CreateCertificateProviderOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_certificate_provider::CreateCertificateProviderError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_certificate_provider();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateCertificateProvider`.
///
/// <p>Creates an Amazon Web Services IoT Core certificate provider. You can use Amazon Web Services IoT Core certificate provider to customize how to sign a certificate signing request (CSR) in IoT fleet provisioning. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/provisioning-cert-provider.html">Customizing certificate signing using Amazon Web Services IoT Core certificate provider</a> from <i>Amazon Web Services IoT Core Developer Guide</i>.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">CreateCertificateProvider</a> action.</p><important>
/// <p>After you create a certificate provider, the behavior of <a href="https://docs.aws.amazon.com/iot/latest/developerguide/fleet-provision-api.html#create-cert-csr"> <code>CreateCertificateFromCsr</code> API for fleet provisioning</a> will change and all API calls to <code>CreateCertificateFromCsr</code> will invoke the certificate provider to create the certificates. It can take up to a few minutes for this behavior to change after a certificate provider is created.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateCertificateProviderFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_certificate_provider::builders::CreateCertificateProviderInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_certificate_provider::CreateCertificateProviderOutput,
        crate::operation::create_certificate_provider::CreateCertificateProviderError,
    > for CreateCertificateProviderFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_certificate_provider::CreateCertificateProviderOutput,
            crate::operation::create_certificate_provider::CreateCertificateProviderError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateCertificateProviderFluentBuilder {
    /// Creates a new `CreateCertificateProvider`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateCertificateProvider as a reference.
    pub fn as_input(&self) -> &crate::operation::create_certificate_provider::builders::CreateCertificateProviderInputBuilder {
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
        crate::operation::create_certificate_provider::CreateCertificateProviderOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_certificate_provider::CreateCertificateProviderError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_certificate_provider::CreateCertificateProvider::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_certificate_provider::CreateCertificateProvider::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_certificate_provider::CreateCertificateProviderOutput,
        crate::operation::create_certificate_provider::CreateCertificateProviderError,
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
    /// <p>The name of the certificate provider.</p>
    pub fn certificate_provider_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.certificate_provider_name(input.into());
        self
    }
    /// <p>The name of the certificate provider.</p>
    pub fn set_certificate_provider_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_certificate_provider_name(input);
        self
    }
    /// <p>The name of the certificate provider.</p>
    pub fn get_certificate_provider_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_certificate_provider_name()
    }
    /// <p>The ARN of the Lambda function that defines the authentication logic.</p>
    pub fn lambda_function_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.lambda_function_arn(input.into());
        self
    }
    /// <p>The ARN of the Lambda function that defines the authentication logic.</p>
    pub fn set_lambda_function_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_lambda_function_arn(input);
        self
    }
    /// <p>The ARN of the Lambda function that defines the authentication logic.</p>
    pub fn get_lambda_function_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_lambda_function_arn()
    }
    /// Appends an item to `accountDefaultForOperations`.
    ///
    /// To override the contents of this collection use [`set_account_default_for_operations`](Self::set_account_default_for_operations).
    ///
    /// <p>A list of the operations that the certificate provider will use to generate certificates. Valid value: <code>CreateCertificateFromCsr</code>.</p>
    pub fn account_default_for_operations(mut self, input: crate::types::CertificateProviderOperation) -> Self {
        self.inner = self.inner.account_default_for_operations(input);
        self
    }
    /// <p>A list of the operations that the certificate provider will use to generate certificates. Valid value: <code>CreateCertificateFromCsr</code>.</p>
    pub fn set_account_default_for_operations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::CertificateProviderOperation>>,
    ) -> Self {
        self.inner = self.inner.set_account_default_for_operations(input);
        self
    }
    /// <p>A list of the operations that the certificate provider will use to generate certificates. Valid value: <code>CreateCertificateFromCsr</code>.</p>
    pub fn get_account_default_for_operations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CertificateProviderOperation>> {
        self.inner.get_account_default_for_operations()
    }
    /// <p>A string that you can optionally pass in the <code>CreateCertificateProvider</code> request to make sure the request is idempotent.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A string that you can optionally pass in the <code>CreateCertificateProvider</code> request to make sure the request is idempotent.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A string that you can optionally pass in the <code>CreateCertificateProvider</code> request to make sure the request is idempotent.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Metadata which can be used to manage the certificate provider.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Metadata which can be used to manage the certificate provider.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Metadata which can be used to manage the certificate provider.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
