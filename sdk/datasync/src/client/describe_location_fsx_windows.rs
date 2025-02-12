// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeLocationFsxWindows`](crate::operation::describe_location_fsx_windows::builders::DescribeLocationFsxWindowsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`location_arn(impl Into<String>)`](crate::operation::describe_location_fsx_windows::builders::DescribeLocationFsxWindowsFluentBuilder::location_arn) / [`set_location_arn(Option<String>)`](crate::operation::describe_location_fsx_windows::builders::DescribeLocationFsxWindowsFluentBuilder::set_location_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the FSx for Windows File Server location to describe.</p><br>
    /// - On success, responds with [`DescribeLocationFsxWindowsOutput`](crate::operation::describe_location_fsx_windows::DescribeLocationFsxWindowsOutput) with field(s):
    ///   - [`location_arn(Option<String>)`](crate::operation::describe_location_fsx_windows::DescribeLocationFsxWindowsOutput::location_arn): <p>The Amazon Resource Name (ARN) of the FSx for Windows File Server location that was described.</p>
    ///   - [`location_uri(Option<String>)`](crate::operation::describe_location_fsx_windows::DescribeLocationFsxWindowsOutput::location_uri): <p>The URL of the FSx for Windows File Server location that was described.</p>
    ///   - [`security_group_arns(Option<Vec::<String>>)`](crate::operation::describe_location_fsx_windows::DescribeLocationFsxWindowsOutput::security_group_arns): <p>The Amazon Resource Names (ARNs) of the security groups that are configured for the FSx for Windows File Server file system.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::operation::describe_location_fsx_windows::DescribeLocationFsxWindowsOutput::creation_time): <p>The time that the FSx for Windows File Server location was created.</p>
    ///   - [`user(Option<String>)`](crate::operation::describe_location_fsx_windows::DescribeLocationFsxWindowsOutput::user): <p>The user who has the permissions to access files and folders in the FSx for Windows File Server file system.</p>
    ///   - [`domain(Option<String>)`](crate::operation::describe_location_fsx_windows::DescribeLocationFsxWindowsOutput::domain): <p>The name of the Windows domain that the FSx for Windows File Server belongs to.</p>
    /// - On failure, responds with [`SdkError<DescribeLocationFsxWindowsError>`](crate::operation::describe_location_fsx_windows::DescribeLocationFsxWindowsError)
    pub fn describe_location_fsx_windows(
        &self,
    ) -> crate::operation::describe_location_fsx_windows::builders::DescribeLocationFsxWindowsFluentBuilder {
        crate::operation::describe_location_fsx_windows::builders::DescribeLocationFsxWindowsFluentBuilder::new(self.handle.clone())
    }
}
