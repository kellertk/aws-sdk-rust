// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteDomain`](crate::operation::delete_domain::builders::DeleteDomainFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`identifier(impl Into<String>)`](crate::operation::delete_domain::builders::DeleteDomainFluentBuilder::identifier) / [`set_identifier(Option<String>)`](crate::operation::delete_domain::builders::DeleteDomainFluentBuilder::set_identifier):<br>required: **true**<br><p>The identifier of the Amazon Web Services domain that is to be deleted.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::delete_domain::builders::DeleteDomainFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::delete_domain::builders::DeleteDomainFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique, case-sensitive identifier that is provided to ensure the idempotency of the request.</p><br>
    /// - On success, responds with [`DeleteDomainOutput`](crate::operation::delete_domain::DeleteDomainOutput) with field(s):
    ///   - [`status(DomainStatus)`](crate::operation::delete_domain::DeleteDomainOutput::status): <p>The status of the domain.</p>
    /// - On failure, responds with [`SdkError<DeleteDomainError>`](crate::operation::delete_domain::DeleteDomainError)
    pub fn delete_domain(&self) -> crate::operation::delete_domain::builders::DeleteDomainFluentBuilder {
        crate::operation::delete_domain::builders::DeleteDomainFluentBuilder::new(self.handle.clone())
    }
}
