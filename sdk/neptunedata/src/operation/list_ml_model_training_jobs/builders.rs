// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_ml_model_training_jobs::_list_ml_model_training_jobs_output::ListMlModelTrainingJobsOutputBuilder;

pub use crate::operation::list_ml_model_training_jobs::_list_ml_model_training_jobs_input::ListMlModelTrainingJobsInputBuilder;

impl ListMlModelTrainingJobsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_ml_model_training_jobs::ListMlModelTrainingJobsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_ml_model_training_jobs::ListMLModelTrainingJobsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_ml_model_training_jobs();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListMLModelTrainingJobs`.
///
/// <p>Lists Neptune ML model-training jobs. See <a href="https://docs.aws.amazon.com/neptune/latest/userguide/machine-learning-api-modeltraining.html">Model training using the <code>modeltraining</code> command</a>.</p>
/// <p>When invoking this operation in a Neptune cluster that has IAM authentication enabled, the IAM user or role making the request must have a policy attached that allows the <a href="https://docs.aws.amazon.com/neptune/latest/userguide/iam-dp-actions.html#neptune-db:listmlmodeltrainingjobs">neptune-db:neptune-db:ListMLModelTrainingJobs</a> IAM action in that cluster.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListMLModelTrainingJobsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_ml_model_training_jobs::builders::ListMlModelTrainingJobsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_ml_model_training_jobs::ListMlModelTrainingJobsOutput,
        crate::operation::list_ml_model_training_jobs::ListMLModelTrainingJobsError,
    > for ListMLModelTrainingJobsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_ml_model_training_jobs::ListMlModelTrainingJobsOutput,
            crate::operation::list_ml_model_training_jobs::ListMLModelTrainingJobsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListMLModelTrainingJobsFluentBuilder {
    /// Creates a new `ListMLModelTrainingJobs`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListMLModelTrainingJobs as a reference.
    pub fn as_input(&self) -> &crate::operation::list_ml_model_training_jobs::builders::ListMlModelTrainingJobsInputBuilder {
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
        crate::operation::list_ml_model_training_jobs::ListMlModelTrainingJobsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_ml_model_training_jobs::ListMLModelTrainingJobsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_ml_model_training_jobs::ListMLModelTrainingJobs::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_ml_model_training_jobs::ListMLModelTrainingJobs::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_ml_model_training_jobs::ListMlModelTrainingJobsOutput,
        crate::operation::list_ml_model_training_jobs::ListMLModelTrainingJobsError,
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
    /// <p>The maximum number of items to return (from 1 to 1024; the default is 10).</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.inner = self.inner.max_items(input);
        self
    }
    /// <p>The maximum number of items to return (from 1 to 1024; the default is 10).</p>
    pub fn set_max_items(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_items(input);
        self
    }
    /// <p>The maximum number of items to return (from 1 to 1024; the default is 10).</p>
    pub fn get_max_items(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_items()
    }
    /// <p>The ARN of an IAM role that provides Neptune access to SageMaker and Amazon S3 resources. This must be listed in your DB cluster parameter group or an error will occur.</p>
    pub fn neptune_iam_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.neptune_iam_role_arn(input.into());
        self
    }
    /// <p>The ARN of an IAM role that provides Neptune access to SageMaker and Amazon S3 resources. This must be listed in your DB cluster parameter group or an error will occur.</p>
    pub fn set_neptune_iam_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_neptune_iam_role_arn(input);
        self
    }
    /// <p>The ARN of an IAM role that provides Neptune access to SageMaker and Amazon S3 resources. This must be listed in your DB cluster parameter group or an error will occur.</p>
    pub fn get_neptune_iam_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_neptune_iam_role_arn()
    }
}
