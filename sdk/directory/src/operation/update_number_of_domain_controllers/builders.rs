// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_number_of_domain_controllers::_update_number_of_domain_controllers_output::UpdateNumberOfDomainControllersOutputBuilder;

pub use crate::operation::update_number_of_domain_controllers::_update_number_of_domain_controllers_input::UpdateNumberOfDomainControllersInputBuilder;

impl UpdateNumberOfDomainControllersInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_number_of_domain_controllers::UpdateNumberOfDomainControllersOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_number_of_domain_controllers::UpdateNumberOfDomainControllersError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_number_of_domain_controllers();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateNumberOfDomainControllers`.
///
/// <p>Adds or removes domain controllers to or from the directory. Based on the difference between current value and new value (provided through this API call), domain controllers will be added or removed. It may take up to 45 minutes for any new domain controllers to become fully active once the requested number of domain controllers is updated. During this time, you cannot make another update request.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateNumberOfDomainControllersFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_number_of_domain_controllers::builders::UpdateNumberOfDomainControllersInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_number_of_domain_controllers::UpdateNumberOfDomainControllersOutput,
        crate::operation::update_number_of_domain_controllers::UpdateNumberOfDomainControllersError,
    > for UpdateNumberOfDomainControllersFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_number_of_domain_controllers::UpdateNumberOfDomainControllersOutput,
            crate::operation::update_number_of_domain_controllers::UpdateNumberOfDomainControllersError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateNumberOfDomainControllersFluentBuilder {
    /// Creates a new `UpdateNumberOfDomainControllers`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateNumberOfDomainControllers as a reference.
    pub fn as_input(&self) -> &crate::operation::update_number_of_domain_controllers::builders::UpdateNumberOfDomainControllersInputBuilder {
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
        crate::operation::update_number_of_domain_controllers::UpdateNumberOfDomainControllersOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_number_of_domain_controllers::UpdateNumberOfDomainControllersError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_number_of_domain_controllers::UpdateNumberOfDomainControllers::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_number_of_domain_controllers::UpdateNumberOfDomainControllers::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_number_of_domain_controllers::UpdateNumberOfDomainControllersOutput,
        crate::operation::update_number_of_domain_controllers::UpdateNumberOfDomainControllersError,
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
    /// <p>Identifier of the directory to which the domain controllers will be added or removed.</p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.directory_id(input.into());
        self
    }
    /// <p>Identifier of the directory to which the domain controllers will be added or removed.</p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_directory_id(input);
        self
    }
    /// <p>Identifier of the directory to which the domain controllers will be added or removed.</p>
    pub fn get_directory_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_directory_id()
    }
    /// <p>The number of domain controllers desired in the directory.</p>
    pub fn desired_number(mut self, input: i32) -> Self {
        self.inner = self.inner.desired_number(input);
        self
    }
    /// <p>The number of domain controllers desired in the directory.</p>
    pub fn set_desired_number(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_desired_number(input);
        self
    }
    /// <p>The number of domain controllers desired in the directory.</p>
    pub fn get_desired_number(&self) -> &::std::option::Option<i32> {
        self.inner.get_desired_number()
    }
}
