// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_data_lake_organization_configuration::_delete_data_lake_organization_configuration_output::DeleteDataLakeOrganizationConfigurationOutputBuilder;

pub use crate::operation::delete_data_lake_organization_configuration::_delete_data_lake_organization_configuration_input::DeleteDataLakeOrganizationConfigurationInputBuilder;

impl DeleteDataLakeOrganizationConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_data_lake_organization_configuration::DeleteDataLakeOrganizationConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_data_lake_organization_configuration::DeleteDataLakeOrganizationConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_data_lake_organization_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteDataLakeOrganizationConfiguration`.
///
/// <p>Turns off automatic enablement of Amazon Security Lake for member accounts that are added to an organization in Organizations. Only the delegated Security Lake administrator for an organization can perform this operation. If the delegated Security Lake administrator performs this operation, new member accounts won't automatically contribute data to the data lake.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteDataLakeOrganizationConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_data_lake_organization_configuration::builders::DeleteDataLakeOrganizationConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_data_lake_organization_configuration::DeleteDataLakeOrganizationConfigurationOutput,
        crate::operation::delete_data_lake_organization_configuration::DeleteDataLakeOrganizationConfigurationError,
    > for DeleteDataLakeOrganizationConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_data_lake_organization_configuration::DeleteDataLakeOrganizationConfigurationOutput,
            crate::operation::delete_data_lake_organization_configuration::DeleteDataLakeOrganizationConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteDataLakeOrganizationConfigurationFluentBuilder {
    /// Creates a new `DeleteDataLakeOrganizationConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteDataLakeOrganizationConfiguration as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::delete_data_lake_organization_configuration::builders::DeleteDataLakeOrganizationConfigurationInputBuilder {
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
        crate::operation::delete_data_lake_organization_configuration::DeleteDataLakeOrganizationConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_data_lake_organization_configuration::DeleteDataLakeOrganizationConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::delete_data_lake_organization_configuration::DeleteDataLakeOrganizationConfiguration::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::delete_data_lake_organization_configuration::DeleteDataLakeOrganizationConfiguration::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_data_lake_organization_configuration::DeleteDataLakeOrganizationConfigurationOutput,
        crate::operation::delete_data_lake_organization_configuration::DeleteDataLakeOrganizationConfigurationError,
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
    /// Appends an item to `autoEnableNewAccount`.
    ///
    /// To override the contents of this collection use [`set_auto_enable_new_account`](Self::set_auto_enable_new_account).
    ///
    /// <p>Turns off automatic enablement of Security Lake for member accounts that are added to an organization.</p>
    pub fn auto_enable_new_account(mut self, input: crate::types::DataLakeAutoEnableNewAccountConfiguration) -> Self {
        self.inner = self.inner.auto_enable_new_account(input);
        self
    }
    /// <p>Turns off automatic enablement of Security Lake for member accounts that are added to an organization.</p>
    pub fn set_auto_enable_new_account(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DataLakeAutoEnableNewAccountConfiguration>>,
    ) -> Self {
        self.inner = self.inner.set_auto_enable_new_account(input);
        self
    }
    /// <p>Turns off automatic enablement of Security Lake for member accounts that are added to an organization.</p>
    pub fn get_auto_enable_new_account(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DataLakeAutoEnableNewAccountConfiguration>> {
        self.inner.get_auto_enable_new_account()
    }
}
