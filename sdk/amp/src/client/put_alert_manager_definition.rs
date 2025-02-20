// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutAlertManagerDefinition`](crate::operation::put_alert_manager_definition::builders::PutAlertManagerDefinitionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`workspace_id(impl Into<String>)`](crate::operation::put_alert_manager_definition::builders::PutAlertManagerDefinitionFluentBuilder::workspace_id) / [`set_workspace_id(Option<String>)`](crate::operation::put_alert_manager_definition::builders::PutAlertManagerDefinitionFluentBuilder::set_workspace_id):<br>required: **true**<br>The ID of the workspace in which to update the alert manager definition.<br>
    ///   - [`data(Blob)`](crate::operation::put_alert_manager_definition::builders::PutAlertManagerDefinitionFluentBuilder::data) / [`set_data(Option<Blob>)`](crate::operation::put_alert_manager_definition::builders::PutAlertManagerDefinitionFluentBuilder::set_data):<br>required: **true**<br>The alert manager definition data.<br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::put_alert_manager_definition::builders::PutAlertManagerDefinitionFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::put_alert_manager_definition::builders::PutAlertManagerDefinitionFluentBuilder::set_client_token):<br>required: **false**<br>Optional, unique, case-sensitive, user-provided identifier to ensure the idempotency of the request.<br>
    /// - On success, responds with [`PutAlertManagerDefinitionOutput`](crate::operation::put_alert_manager_definition::PutAlertManagerDefinitionOutput) with field(s):
    ///   - [`status(Option<AlertManagerDefinitionStatus>)`](crate::operation::put_alert_manager_definition::PutAlertManagerDefinitionOutput::status): The status of alert manager definition.
    /// - On failure, responds with [`SdkError<PutAlertManagerDefinitionError>`](crate::operation::put_alert_manager_definition::PutAlertManagerDefinitionError)
    pub fn put_alert_manager_definition(&self) -> crate::operation::put_alert_manager_definition::builders::PutAlertManagerDefinitionFluentBuilder {
        crate::operation::put_alert_manager_definition::builders::PutAlertManagerDefinitionFluentBuilder::new(self.handle.clone())
    }
}
