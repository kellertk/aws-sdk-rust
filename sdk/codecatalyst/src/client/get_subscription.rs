// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetSubscription`](crate::operation::get_subscription::builders::GetSubscriptionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`space_name(impl Into<String>)`](crate::operation::get_subscription::builders::GetSubscriptionFluentBuilder::space_name) / [`set_space_name(Option<String>)`](crate::operation::get_subscription::builders::GetSubscriptionFluentBuilder::set_space_name):<br>required: **true**<br><p>The name of the space.</p><br>
    /// - On success, responds with [`GetSubscriptionOutput`](crate::operation::get_subscription::GetSubscriptionOutput) with field(s):
    ///   - [`subscription_type(Option<String>)`](crate::operation::get_subscription::GetSubscriptionOutput::subscription_type): <p>The type of the billing plan for the space.</p>
    ///   - [`aws_account_name(Option<String>)`](crate::operation::get_subscription::GetSubscriptionOutput::aws_account_name): <p>The display name of the Amazon Web Services account used for billing for the space.</p>
    /// - On failure, responds with [`SdkError<GetSubscriptionError>`](crate::operation::get_subscription::GetSubscriptionError)
    pub fn get_subscription(&self) -> crate::operation::get_subscription::builders::GetSubscriptionFluentBuilder {
        crate::operation::get_subscription::builders::GetSubscriptionFluentBuilder::new(self.handle.clone())
    }
}
