// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::terminate_sol_network_instance::_terminate_sol_network_instance_output::TerminateSolNetworkInstanceOutputBuilder;

pub use crate::operation::terminate_sol_network_instance::_terminate_sol_network_instance_input::TerminateSolNetworkInstanceInputBuilder;

impl TerminateSolNetworkInstanceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::terminate_sol_network_instance::TerminateSolNetworkInstanceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::terminate_sol_network_instance::TerminateSolNetworkInstanceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.terminate_sol_network_instance();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `TerminateSolNetworkInstance`.
///
/// <p>Terminates a network instance.</p>
/// <p>A network instance is a single network created in Amazon Web Services TNB that can be deployed and on which life-cycle operations (like terminate, update, and delete) can be performed.</p>
/// <p>You must terminate a network instance before you can delete it.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct TerminateSolNetworkInstanceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::terminate_sol_network_instance::builders::TerminateSolNetworkInstanceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::terminate_sol_network_instance::TerminateSolNetworkInstanceOutput,
        crate::operation::terminate_sol_network_instance::TerminateSolNetworkInstanceError,
    > for TerminateSolNetworkInstanceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::terminate_sol_network_instance::TerminateSolNetworkInstanceOutput,
            crate::operation::terminate_sol_network_instance::TerminateSolNetworkInstanceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl TerminateSolNetworkInstanceFluentBuilder {
    /// Creates a new `TerminateSolNetworkInstance`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the TerminateSolNetworkInstance as a reference.
    pub fn as_input(&self) -> &crate::operation::terminate_sol_network_instance::builders::TerminateSolNetworkInstanceInputBuilder {
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
        crate::operation::terminate_sol_network_instance::TerminateSolNetworkInstanceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::terminate_sol_network_instance::TerminateSolNetworkInstanceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::terminate_sol_network_instance::TerminateSolNetworkInstance::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::terminate_sol_network_instance::TerminateSolNetworkInstance::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::terminate_sol_network_instance::TerminateSolNetworkInstanceOutput,
        crate::operation::terminate_sol_network_instance::TerminateSolNetworkInstanceError,
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
    /// <p>ID of the network instance.</p>
    pub fn ns_instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.ns_instance_id(input.into());
        self
    }
    /// <p>ID of the network instance.</p>
    pub fn set_ns_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_ns_instance_id(input);
        self
    }
    /// <p>ID of the network instance.</p>
    pub fn get_ns_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_ns_instance_id()
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key and an optional value. When you use this API, the tags are transferred to the network operation that is created. Use tags to search and filter your resources or track your Amazon Web Services costs.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key and an optional value. When you use this API, the tags are transferred to the network operation that is created. Use tags to search and filter your resources or track your Amazon Web Services costs.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key and an optional value. When you use this API, the tags are transferred to the network operation that is created. Use tags to search and filter your resources or track your Amazon Web Services costs.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
