// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeletePlatformApplication`](crate::operation::delete_platform_application::builders::DeletePlatformApplicationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`platform_application_arn(impl Into<String>)`](crate::operation::delete_platform_application::builders::DeletePlatformApplicationFluentBuilder::platform_application_arn) / [`set_platform_application_arn(Option<String>)`](crate::operation::delete_platform_application::builders::DeletePlatformApplicationFluentBuilder::set_platform_application_arn):<br>required: **true**<br><p>PlatformApplicationArn of platform application object to delete.</p><br>
    /// - On success, responds with [`DeletePlatformApplicationOutput`](crate::operation::delete_platform_application::DeletePlatformApplicationOutput)
    /// - On failure, responds with [`SdkError<DeletePlatformApplicationError>`](crate::operation::delete_platform_application::DeletePlatformApplicationError)
    pub fn delete_platform_application(&self) -> crate::operation::delete_platform_application::builders::DeletePlatformApplicationFluentBuilder {
        crate::operation::delete_platform_application::builders::DeletePlatformApplicationFluentBuilder::new(self.handle.clone())
    }
}
