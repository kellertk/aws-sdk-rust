// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::rebalance_slots_in_global_replication_group::_rebalance_slots_in_global_replication_group_output::RebalanceSlotsInGlobalReplicationGroupOutputBuilder;

pub use crate::operation::rebalance_slots_in_global_replication_group::_rebalance_slots_in_global_replication_group_input::RebalanceSlotsInGlobalReplicationGroupInputBuilder;

impl RebalanceSlotsInGlobalReplicationGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::rebalance_slots_in_global_replication_group::RebalanceSlotsInGlobalReplicationGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::rebalance_slots_in_global_replication_group::RebalanceSlotsInGlobalReplicationGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.rebalance_slots_in_global_replication_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RebalanceSlotsInGlobalReplicationGroup`.
///
/// <p>Redistribute slots to ensure uniform distribution across existing shards in the cluster.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RebalanceSlotsInGlobalReplicationGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::rebalance_slots_in_global_replication_group::builders::RebalanceSlotsInGlobalReplicationGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::rebalance_slots_in_global_replication_group::RebalanceSlotsInGlobalReplicationGroupOutput,
        crate::operation::rebalance_slots_in_global_replication_group::RebalanceSlotsInGlobalReplicationGroupError,
    > for RebalanceSlotsInGlobalReplicationGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::rebalance_slots_in_global_replication_group::RebalanceSlotsInGlobalReplicationGroupOutput,
            crate::operation::rebalance_slots_in_global_replication_group::RebalanceSlotsInGlobalReplicationGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RebalanceSlotsInGlobalReplicationGroupFluentBuilder {
    /// Creates a new `RebalanceSlotsInGlobalReplicationGroup`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RebalanceSlotsInGlobalReplicationGroup as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::rebalance_slots_in_global_replication_group::builders::RebalanceSlotsInGlobalReplicationGroupInputBuilder {
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
        crate::operation::rebalance_slots_in_global_replication_group::RebalanceSlotsInGlobalReplicationGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::rebalance_slots_in_global_replication_group::RebalanceSlotsInGlobalReplicationGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::rebalance_slots_in_global_replication_group::RebalanceSlotsInGlobalReplicationGroup::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::rebalance_slots_in_global_replication_group::RebalanceSlotsInGlobalReplicationGroup::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::rebalance_slots_in_global_replication_group::RebalanceSlotsInGlobalReplicationGroupOutput,
        crate::operation::rebalance_slots_in_global_replication_group::RebalanceSlotsInGlobalReplicationGroupError,
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
    /// <p>The name of the Global datastore</p>
    pub fn global_replication_group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.global_replication_group_id(input.into());
        self
    }
    /// <p>The name of the Global datastore</p>
    pub fn set_global_replication_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_global_replication_group_id(input);
        self
    }
    /// <p>The name of the Global datastore</p>
    pub fn get_global_replication_group_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_global_replication_group_id()
    }
    /// <p>If <code>True</code>, redistribution is applied immediately.</p>
    pub fn apply_immediately(mut self, input: bool) -> Self {
        self.inner = self.inner.apply_immediately(input);
        self
    }
    /// <p>If <code>True</code>, redistribution is applied immediately.</p>
    pub fn set_apply_immediately(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_apply_immediately(input);
        self
    }
    /// <p>If <code>True</code>, redistribution is applied immediately.</p>
    pub fn get_apply_immediately(&self) -> &::std::option::Option<bool> {
        self.inner.get_apply_immediately()
    }
}
