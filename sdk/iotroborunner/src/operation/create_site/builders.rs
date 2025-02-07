// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_site::_create_site_output::CreateSiteOutputBuilder;

pub use crate::operation::create_site::_create_site_input::CreateSiteInputBuilder;

impl CreateSiteInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_site::CreateSiteOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_site::CreateSiteError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_site();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateSite`.
///
/// Grants permission to create a site
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateSiteFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_site::builders::CreateSiteInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_site::CreateSiteOutput,
        crate::operation::create_site::CreateSiteError,
    > for CreateSiteFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_site::CreateSiteOutput,
            crate::operation::create_site::CreateSiteError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateSiteFluentBuilder {
    /// Creates a new `CreateSite`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateSite as a reference.
    pub fn as_input(&self) -> &crate::operation::create_site::builders::CreateSiteInputBuilder {
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
        crate::operation::create_site::CreateSiteOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_site::CreateSiteError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_site::CreateSite::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_site::CreateSite::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_site::CreateSiteOutput,
        crate::operation::create_site::CreateSiteError,
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
    /// Token used for detecting replayed requests. Replayed requests will not be performed multiple times.
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// Token used for detecting replayed requests. Replayed requests will not be performed multiple times.
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// Token used for detecting replayed requests. Replayed requests will not be performed multiple times.
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// Human friendly name of the resource.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// Human friendly name of the resource.
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// Human friendly name of the resource.
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// A valid ISO 3166-1 alpha-2 code for the country in which the site resides. e.g., US.
    pub fn country_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.country_code(input.into());
        self
    }
    /// A valid ISO 3166-1 alpha-2 code for the country in which the site resides. e.g., US.
    pub fn set_country_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_country_code(input);
        self
    }
    /// A valid ISO 3166-1 alpha-2 code for the country in which the site resides. e.g., US.
    pub fn get_country_code(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_country_code()
    }
    /// A high-level description of the site.
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// A high-level description of the site.
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// A high-level description of the site.
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
}
