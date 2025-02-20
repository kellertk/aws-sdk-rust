// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct AdminUpdateAuthEventFeedbackInput {
    /// <p>The user pool ID.</p>
    pub user_pool_id: ::std::option::Option<::std::string::String>,
    /// <p>The username of the user that you want to query or modify. The value of this parameter is typically your user's username, but it can be any of their alias attributes. If <code>username</code> isn't an alias attribute in your user pool, you can also use their <code>sub</code> in this request.</p>
    pub username: ::std::option::Option<::std::string::String>,
    /// <p>The authentication event ID.</p>
    pub event_id: ::std::option::Option<::std::string::String>,
    /// <p>The authentication event feedback value. When you provide a <code>FeedbackValue</code> value of <code>valid</code>, you tell Amazon Cognito that you trust a user session where Amazon Cognito has evaluated some level of risk. When you provide a <code>FeedbackValue</code> value of <code>invalid</code>, you tell Amazon Cognito that you don't trust a user session, or you don't believe that Amazon Cognito evaluated a high-enough risk level.</p>
    pub feedback_value: ::std::option::Option<crate::types::FeedbackValueType>,
}
impl AdminUpdateAuthEventFeedbackInput {
    /// <p>The user pool ID.</p>
    pub fn user_pool_id(&self) -> ::std::option::Option<&str> {
        self.user_pool_id.as_deref()
    }
    /// <p>The username of the user that you want to query or modify. The value of this parameter is typically your user's username, but it can be any of their alias attributes. If <code>username</code> isn't an alias attribute in your user pool, you can also use their <code>sub</code> in this request.</p>
    pub fn username(&self) -> ::std::option::Option<&str> {
        self.username.as_deref()
    }
    /// <p>The authentication event ID.</p>
    pub fn event_id(&self) -> ::std::option::Option<&str> {
        self.event_id.as_deref()
    }
    /// <p>The authentication event feedback value. When you provide a <code>FeedbackValue</code> value of <code>valid</code>, you tell Amazon Cognito that you trust a user session where Amazon Cognito has evaluated some level of risk. When you provide a <code>FeedbackValue</code> value of <code>invalid</code>, you tell Amazon Cognito that you don't trust a user session, or you don't believe that Amazon Cognito evaluated a high-enough risk level.</p>
    pub fn feedback_value(&self) -> ::std::option::Option<&crate::types::FeedbackValueType> {
        self.feedback_value.as_ref()
    }
}
impl ::std::fmt::Debug for AdminUpdateAuthEventFeedbackInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AdminUpdateAuthEventFeedbackInput");
        formatter.field("user_pool_id", &self.user_pool_id);
        formatter.field("username", &"*** Sensitive Data Redacted ***");
        formatter.field("event_id", &self.event_id);
        formatter.field("feedback_value", &self.feedback_value);
        formatter.finish()
    }
}
impl AdminUpdateAuthEventFeedbackInput {
    /// Creates a new builder-style object to manufacture [`AdminUpdateAuthEventFeedbackInput`](crate::operation::admin_update_auth_event_feedback::AdminUpdateAuthEventFeedbackInput).
    pub fn builder() -> crate::operation::admin_update_auth_event_feedback::builders::AdminUpdateAuthEventFeedbackInputBuilder {
        crate::operation::admin_update_auth_event_feedback::builders::AdminUpdateAuthEventFeedbackInputBuilder::default()
    }
}

/// A builder for [`AdminUpdateAuthEventFeedbackInput`](crate::operation::admin_update_auth_event_feedback::AdminUpdateAuthEventFeedbackInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct AdminUpdateAuthEventFeedbackInputBuilder {
    pub(crate) user_pool_id: ::std::option::Option<::std::string::String>,
    pub(crate) username: ::std::option::Option<::std::string::String>,
    pub(crate) event_id: ::std::option::Option<::std::string::String>,
    pub(crate) feedback_value: ::std::option::Option<crate::types::FeedbackValueType>,
}
impl AdminUpdateAuthEventFeedbackInputBuilder {
    /// <p>The user pool ID.</p>
    /// This field is required.
    pub fn user_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_pool_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The user pool ID.</p>
    pub fn set_user_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_pool_id = input;
        self
    }
    /// <p>The user pool ID.</p>
    pub fn get_user_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.user_pool_id
    }
    /// <p>The username of the user that you want to query or modify. The value of this parameter is typically your user's username, but it can be any of their alias attributes. If <code>username</code> isn't an alias attribute in your user pool, you can also use their <code>sub</code> in this request.</p>
    /// This field is required.
    pub fn username(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.username = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The username of the user that you want to query or modify. The value of this parameter is typically your user's username, but it can be any of their alias attributes. If <code>username</code> isn't an alias attribute in your user pool, you can also use their <code>sub</code> in this request.</p>
    pub fn set_username(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.username = input;
        self
    }
    /// <p>The username of the user that you want to query or modify. The value of this parameter is typically your user's username, but it can be any of their alias attributes. If <code>username</code> isn't an alias attribute in your user pool, you can also use their <code>sub</code> in this request.</p>
    pub fn get_username(&self) -> &::std::option::Option<::std::string::String> {
        &self.username
    }
    /// <p>The authentication event ID.</p>
    /// This field is required.
    pub fn event_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.event_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The authentication event ID.</p>
    pub fn set_event_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.event_id = input;
        self
    }
    /// <p>The authentication event ID.</p>
    pub fn get_event_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.event_id
    }
    /// <p>The authentication event feedback value. When you provide a <code>FeedbackValue</code> value of <code>valid</code>, you tell Amazon Cognito that you trust a user session where Amazon Cognito has evaluated some level of risk. When you provide a <code>FeedbackValue</code> value of <code>invalid</code>, you tell Amazon Cognito that you don't trust a user session, or you don't believe that Amazon Cognito evaluated a high-enough risk level.</p>
    /// This field is required.
    pub fn feedback_value(mut self, input: crate::types::FeedbackValueType) -> Self {
        self.feedback_value = ::std::option::Option::Some(input);
        self
    }
    /// <p>The authentication event feedback value. When you provide a <code>FeedbackValue</code> value of <code>valid</code>, you tell Amazon Cognito that you trust a user session where Amazon Cognito has evaluated some level of risk. When you provide a <code>FeedbackValue</code> value of <code>invalid</code>, you tell Amazon Cognito that you don't trust a user session, or you don't believe that Amazon Cognito evaluated a high-enough risk level.</p>
    pub fn set_feedback_value(mut self, input: ::std::option::Option<crate::types::FeedbackValueType>) -> Self {
        self.feedback_value = input;
        self
    }
    /// <p>The authentication event feedback value. When you provide a <code>FeedbackValue</code> value of <code>valid</code>, you tell Amazon Cognito that you trust a user session where Amazon Cognito has evaluated some level of risk. When you provide a <code>FeedbackValue</code> value of <code>invalid</code>, you tell Amazon Cognito that you don't trust a user session, or you don't believe that Amazon Cognito evaluated a high-enough risk level.</p>
    pub fn get_feedback_value(&self) -> &::std::option::Option<crate::types::FeedbackValueType> {
        &self.feedback_value
    }
    /// Consumes the builder and constructs a [`AdminUpdateAuthEventFeedbackInput`](crate::operation::admin_update_auth_event_feedback::AdminUpdateAuthEventFeedbackInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::admin_update_auth_event_feedback::AdminUpdateAuthEventFeedbackInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::admin_update_auth_event_feedback::AdminUpdateAuthEventFeedbackInput {
            user_pool_id: self.user_pool_id,
            username: self.username,
            event_id: self.event_id,
            feedback_value: self.feedback_value,
        })
    }
}
impl ::std::fmt::Debug for AdminUpdateAuthEventFeedbackInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AdminUpdateAuthEventFeedbackInputBuilder");
        formatter.field("user_pool_id", &self.user_pool_id);
        formatter.field("username", &"*** Sensitive Data Redacted ***");
        formatter.field("event_id", &self.event_id);
        formatter.field("feedback_value", &self.feedback_value);
        formatter.finish()
    }
}
