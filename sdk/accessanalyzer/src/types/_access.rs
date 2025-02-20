// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about actions that define permissions to check against a policy.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Access {
    /// <p>A list of actions for the access permissions.</p>
    pub actions: ::std::vec::Vec<::std::string::String>,
}
impl Access {
    /// <p>A list of actions for the access permissions.</p>
    pub fn actions(&self) -> &[::std::string::String] {
        use std::ops::Deref;
        self.actions.deref()
    }
}
impl Access {
    /// Creates a new builder-style object to manufacture [`Access`](crate::types::Access).
    pub fn builder() -> crate::types::builders::AccessBuilder {
        crate::types::builders::AccessBuilder::default()
    }
}

/// A builder for [`Access`](crate::types::Access).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AccessBuilder {
    pub(crate) actions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl AccessBuilder {
    /// Appends an item to `actions`.
    ///
    /// To override the contents of this collection use [`set_actions`](Self::set_actions).
    ///
    /// <p>A list of actions for the access permissions.</p>
    pub fn actions(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.actions.unwrap_or_default();
        v.push(input.into());
        self.actions = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of actions for the access permissions.</p>
    pub fn set_actions(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.actions = input;
        self
    }
    /// <p>A list of actions for the access permissions.</p>
    pub fn get_actions(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.actions
    }
    /// Consumes the builder and constructs a [`Access`](crate::types::Access).
    /// This method will fail if any of the following fields are not set:
    /// - [`actions`](crate::types::builders::AccessBuilder::actions)
    pub fn build(self) -> ::std::result::Result<crate::types::Access, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::Access {
            actions: self.actions.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "actions",
                    "actions was not specified but it is required when building Access",
                )
            })?,
        })
    }
}
