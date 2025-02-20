// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_project::_create_project_output::CreateProjectOutputBuilder;

pub use crate::operation::create_project::_create_project_input::CreateProjectInputBuilder;

impl CreateProjectInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_project::CreateProjectOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_project::CreateProjectError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_project();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateProject`.
///
/// <p>Creates a project, which is the logical object in Evidently that can contain features, launches, and experiments. Use projects to group similar features together.</p>
/// <p>To update an existing project, use <a href="https://docs.aws.amazon.com/cloudwatchevidently/latest/APIReference/API_UpdateProject.html">UpdateProject</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateProjectFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_project::builders::CreateProjectInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_project::CreateProjectOutput,
        crate::operation::create_project::CreateProjectError,
    > for CreateProjectFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_project::CreateProjectOutput,
            crate::operation::create_project::CreateProjectError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateProjectFluentBuilder {
    /// Creates a new `CreateProject`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateProject as a reference.
    pub fn as_input(&self) -> &crate::operation::create_project::builders::CreateProjectInputBuilder {
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
        crate::operation::create_project::CreateProjectOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_project::CreateProjectError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_project::CreateProject::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_project::CreateProject::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_project::CreateProjectOutput,
        crate::operation::create_project::CreateProjectError,
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
    /// <p>The name for the project.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name for the project.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name for the project.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>An optional description of the project.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>An optional description of the project.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>An optional description of the project.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>A structure that contains information about where Evidently is to store evaluation events for longer term storage, if you choose to do so. If you choose not to store these events, Evidently deletes them after using them to produce metrics and other experiment results that you can view.</p>
    pub fn data_delivery(mut self, input: crate::types::ProjectDataDeliveryConfig) -> Self {
        self.inner = self.inner.data_delivery(input);
        self
    }
    /// <p>A structure that contains information about where Evidently is to store evaluation events for longer term storage, if you choose to do so. If you choose not to store these events, Evidently deletes them after using them to produce metrics and other experiment results that you can view.</p>
    pub fn set_data_delivery(mut self, input: ::std::option::Option<crate::types::ProjectDataDeliveryConfig>) -> Self {
        self.inner = self.inner.set_data_delivery(input);
        self
    }
    /// <p>A structure that contains information about where Evidently is to store evaluation events for longer term storage, if you choose to do so. If you choose not to store these events, Evidently deletes them after using them to produce metrics and other experiment results that you can view.</p>
    pub fn get_data_delivery(&self) -> &::std::option::Option<crate::types::ProjectDataDeliveryConfig> {
        self.inner.get_data_delivery()
    }
    /// <p>Use this parameter if the project will use <i>client-side evaluation powered by AppConfig</i>. Client-side evaluation allows your application to assign variations to user sessions locally instead of by calling the <a href="https://docs.aws.amazon.com/cloudwatchevidently/latest/APIReference/API_EvaluateFeature.html">EvaluateFeature</a> operation. This mitigates the latency and availability risks that come with an API call. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Evidently-client-side-evaluation.html"> Client-side evaluation - powered by AppConfig.</a></p>
    /// <p>This parameter is a structure that contains information about the AppConfig application and environment that will be used as for client-side evaluation.</p>
    /// <p>To create a project that uses client-side evaluation, you must have the <code>evidently:ExportProjectAsConfiguration</code> permission.</p>
    pub fn app_config_resource(mut self, input: crate::types::ProjectAppConfigResourceConfig) -> Self {
        self.inner = self.inner.app_config_resource(input);
        self
    }
    /// <p>Use this parameter if the project will use <i>client-side evaluation powered by AppConfig</i>. Client-side evaluation allows your application to assign variations to user sessions locally instead of by calling the <a href="https://docs.aws.amazon.com/cloudwatchevidently/latest/APIReference/API_EvaluateFeature.html">EvaluateFeature</a> operation. This mitigates the latency and availability risks that come with an API call. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Evidently-client-side-evaluation.html"> Client-side evaluation - powered by AppConfig.</a></p>
    /// <p>This parameter is a structure that contains information about the AppConfig application and environment that will be used as for client-side evaluation.</p>
    /// <p>To create a project that uses client-side evaluation, you must have the <code>evidently:ExportProjectAsConfiguration</code> permission.</p>
    pub fn set_app_config_resource(mut self, input: ::std::option::Option<crate::types::ProjectAppConfigResourceConfig>) -> Self {
        self.inner = self.inner.set_app_config_resource(input);
        self
    }
    /// <p>Use this parameter if the project will use <i>client-side evaluation powered by AppConfig</i>. Client-side evaluation allows your application to assign variations to user sessions locally instead of by calling the <a href="https://docs.aws.amazon.com/cloudwatchevidently/latest/APIReference/API_EvaluateFeature.html">EvaluateFeature</a> operation. This mitigates the latency and availability risks that come with an API call. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Evidently-client-side-evaluation.html"> Client-side evaluation - powered by AppConfig.</a></p>
    /// <p>This parameter is a structure that contains information about the AppConfig application and environment that will be used as for client-side evaluation.</p>
    /// <p>To create a project that uses client-side evaluation, you must have the <code>evidently:ExportProjectAsConfiguration</code> permission.</p>
    pub fn get_app_config_resource(&self) -> &::std::option::Option<crate::types::ProjectAppConfigResourceConfig> {
        self.inner.get_app_config_resource()
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Assigns one or more tags (key-value pairs) to the project.</p>
    /// <p>Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting a user permission to access or change only resources with certain tag values.</p>
    /// <p>Tags don't have any semantic meaning to Amazon Web Services and are interpreted strictly as strings of characters.</p>
    /// <p>You can associate as many as 50 tags with a project.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>Assigns one or more tags (key-value pairs) to the project.</p>
    /// <p>Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting a user permission to access or change only resources with certain tag values.</p>
    /// <p>Tags don't have any semantic meaning to Amazon Web Services and are interpreted strictly as strings of characters.</p>
    /// <p>You can associate as many as 50 tags with a project.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Assigns one or more tags (key-value pairs) to the project.</p>
    /// <p>Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting a user permission to access or change only resources with certain tag values.</p>
    /// <p>Tags don't have any semantic meaning to Amazon Web Services and are interpreted strictly as strings of characters.</p>
    /// <p>You can associate as many as 50 tags with a project.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
