// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateWorkerConfiguration`](crate::operation::create_worker_configuration::builders::CreateWorkerConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`description(impl Into<String>)`](crate::operation::create_worker_configuration::builders::CreateWorkerConfigurationFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_worker_configuration::builders::CreateWorkerConfigurationFluentBuilder::set_description):<br>required: **false**<br><p>A summary description of the worker configuration.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::create_worker_configuration::builders::CreateWorkerConfigurationFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_worker_configuration::builders::CreateWorkerConfigurationFluentBuilder::set_name):<br>required: **true**<br><p>The name of the worker configuration.</p><br>
    ///   - [`properties_file_content(impl Into<String>)`](crate::operation::create_worker_configuration::builders::CreateWorkerConfigurationFluentBuilder::properties_file_content) / [`set_properties_file_content(Option<String>)`](crate::operation::create_worker_configuration::builders::CreateWorkerConfigurationFluentBuilder::set_properties_file_content):<br>required: **true**<br><p>Base64 encoded contents of connect-distributed.properties file.</p><br>
    /// - On success, responds with [`CreateWorkerConfigurationOutput`](crate::operation::create_worker_configuration::CreateWorkerConfigurationOutput) with field(s):
    ///   - [`creation_time(Option<DateTime>)`](crate::operation::create_worker_configuration::CreateWorkerConfigurationOutput::creation_time): <p>The time that the worker configuration was created.</p>
    ///   - [`latest_revision(Option<WorkerConfigurationRevisionSummary>)`](crate::operation::create_worker_configuration::CreateWorkerConfigurationOutput::latest_revision): <p>The latest revision of the worker configuration.</p>
    ///   - [`name(Option<String>)`](crate::operation::create_worker_configuration::CreateWorkerConfigurationOutput::name): <p>The name of the worker configuration.</p>
    ///   - [`worker_configuration_arn(Option<String>)`](crate::operation::create_worker_configuration::CreateWorkerConfigurationOutput::worker_configuration_arn): <p>The Amazon Resource Name (ARN) that Amazon assigned to the worker configuration.</p>
    /// - On failure, responds with [`SdkError<CreateWorkerConfigurationError>`](crate::operation::create_worker_configuration::CreateWorkerConfigurationError)
    pub fn create_worker_configuration(&self) -> crate::operation::create_worker_configuration::builders::CreateWorkerConfigurationFluentBuilder {
        crate::operation::create_worker_configuration::builders::CreateWorkerConfigurationFluentBuilder::new(self.handle.clone())
    }
}
