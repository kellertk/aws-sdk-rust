// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_datasource_packages::_update_datasource_packages_output::UpdateDatasourcePackagesOutputBuilder;

pub use crate::operation::update_datasource_packages::_update_datasource_packages_input::UpdateDatasourcePackagesInputBuilder;

impl UpdateDatasourcePackagesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_datasource_packages::UpdateDatasourcePackagesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_datasource_packages::UpdateDatasourcePackagesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_datasource_packages();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateDatasourcePackages`.
///
/// <p>Starts a data source packages for the behavior graph.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateDatasourcePackagesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_datasource_packages::builders::UpdateDatasourcePackagesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_datasource_packages::UpdateDatasourcePackagesOutput,
        crate::operation::update_datasource_packages::UpdateDatasourcePackagesError,
    > for UpdateDatasourcePackagesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_datasource_packages::UpdateDatasourcePackagesOutput,
            crate::operation::update_datasource_packages::UpdateDatasourcePackagesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateDatasourcePackagesFluentBuilder {
    /// Creates a new `UpdateDatasourcePackages`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateDatasourcePackages as a reference.
    pub fn as_input(&self) -> &crate::operation::update_datasource_packages::builders::UpdateDatasourcePackagesInputBuilder {
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
        crate::operation::update_datasource_packages::UpdateDatasourcePackagesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_datasource_packages::UpdateDatasourcePackagesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_datasource_packages::UpdateDatasourcePackages::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_datasource_packages::UpdateDatasourcePackages::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_datasource_packages::UpdateDatasourcePackagesOutput,
        crate::operation::update_datasource_packages::UpdateDatasourcePackagesError,
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
    /// <p>The ARN of the behavior graph.</p>
    pub fn graph_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.graph_arn(input.into());
        self
    }
    /// <p>The ARN of the behavior graph.</p>
    pub fn set_graph_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_graph_arn(input);
        self
    }
    /// <p>The ARN of the behavior graph.</p>
    pub fn get_graph_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_graph_arn()
    }
    /// Appends an item to `DatasourcePackages`.
    ///
    /// To override the contents of this collection use [`set_datasource_packages`](Self::set_datasource_packages).
    ///
    /// <p>The data source package start for the behavior graph.</p>
    pub fn datasource_packages(mut self, input: crate::types::DatasourcePackage) -> Self {
        self.inner = self.inner.datasource_packages(input);
        self
    }
    /// <p>The data source package start for the behavior graph.</p>
    pub fn set_datasource_packages(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DatasourcePackage>>) -> Self {
        self.inner = self.inner.set_datasource_packages(input);
        self
    }
    /// <p>The data source package start for the behavior graph.</p>
    pub fn get_datasource_packages(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DatasourcePackage>> {
        self.inner.get_datasource_packages()
    }
}
