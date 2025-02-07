// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_profile::_update_profile_output::UpdateProfileOutputBuilder;

pub use crate::operation::update_profile::_update_profile_input::UpdateProfileInputBuilder;

impl UpdateProfileInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_profile::UpdateProfileOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_profile::UpdateProfileError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_profile();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateProfile`.
///
/// <p>Updates some of the parameters for an existing profile. Provide the <code>ProfileId</code> for the profile that you want to update, along with the new values for the parameters to update.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateProfileFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_profile::builders::UpdateProfileInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_profile::UpdateProfileOutput,
        crate::operation::update_profile::UpdateProfileError,
    > for UpdateProfileFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_profile::UpdateProfileOutput,
            crate::operation::update_profile::UpdateProfileError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateProfileFluentBuilder {
    /// Creates a new `UpdateProfile`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateProfile as a reference.
    pub fn as_input(&self) -> &crate::operation::update_profile::builders::UpdateProfileInputBuilder {
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
        crate::operation::update_profile::UpdateProfileOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_profile::UpdateProfileError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_profile::UpdateProfile::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_profile::UpdateProfile::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_profile::UpdateProfileOutput,
        crate::operation::update_profile::UpdateProfileError,
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
    /// <p>The identifier of the profile object that you are updating.</p>
    pub fn profile_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.profile_id(input.into());
        self
    }
    /// <p>The identifier of the profile object that you are updating.</p>
    pub fn set_profile_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_profile_id(input);
        self
    }
    /// <p>The identifier of the profile object that you are updating.</p>
    pub fn get_profile_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_profile_id()
    }
    /// Appends an item to `CertificateIds`.
    ///
    /// To override the contents of this collection use [`set_certificate_ids`](Self::set_certificate_ids).
    ///
    /// <p>An array of identifiers for the imported certificates. You use this identifier for working with profiles and partner profiles.</p>
    pub fn certificate_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.certificate_ids(input.into());
        self
    }
    /// <p>An array of identifiers for the imported certificates. You use this identifier for working with profiles and partner profiles.</p>
    pub fn set_certificate_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_certificate_ids(input);
        self
    }
    /// <p>An array of identifiers for the imported certificates. You use this identifier for working with profiles and partner profiles.</p>
    pub fn get_certificate_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_certificate_ids()
    }
}
