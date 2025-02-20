// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A user input field in an plugin action review payload.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ActionReviewPayloadField {
    /// <p>The name of the field.</p>
    pub display_name: ::std::option::Option<::std::string::String>,
    /// <p>The display order of fields in a payload.</p>
    pub display_order: ::std::option::Option<i32>,
    /// <p>The type of field.</p>
    pub r#type: ::std::option::Option<crate::types::ActionPayloadFieldType>,
    /// <p>The field value.</p>
    pub value: ::std::option::Option<::aws_smithy_types::Document>,
    /// <p>Information about the field values that an end user can use to provide to Amazon Q for Amazon Q to perform the requested plugin action.</p>
    pub allowed_values: ::std::option::Option<::std::vec::Vec<crate::types::ActionReviewPayloadFieldAllowedValue>>,
    /// <p>Information about whether the field is required.</p>
    pub required: ::std::option::Option<bool>,
}
impl ActionReviewPayloadField {
    /// <p>The name of the field.</p>
    pub fn display_name(&self) -> ::std::option::Option<&str> {
        self.display_name.as_deref()
    }
    /// <p>The display order of fields in a payload.</p>
    pub fn display_order(&self) -> ::std::option::Option<i32> {
        self.display_order
    }
    /// <p>The type of field.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::ActionPayloadFieldType> {
        self.r#type.as_ref()
    }
    /// <p>The field value.</p>
    pub fn value(&self) -> ::std::option::Option<&::aws_smithy_types::Document> {
        self.value.as_ref()
    }
    /// <p>Information about the field values that an end user can use to provide to Amazon Q for Amazon Q to perform the requested plugin action.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.allowed_values.is_none()`.
    pub fn allowed_values(&self) -> &[crate::types::ActionReviewPayloadFieldAllowedValue] {
        self.allowed_values.as_deref().unwrap_or_default()
    }
    /// <p>Information about whether the field is required.</p>
    pub fn required(&self) -> ::std::option::Option<bool> {
        self.required
    }
}
impl ActionReviewPayloadField {
    /// Creates a new builder-style object to manufacture [`ActionReviewPayloadField`](crate::types::ActionReviewPayloadField).
    pub fn builder() -> crate::types::builders::ActionReviewPayloadFieldBuilder {
        crate::types::builders::ActionReviewPayloadFieldBuilder::default()
    }
}

/// A builder for [`ActionReviewPayloadField`](crate::types::ActionReviewPayloadField).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ActionReviewPayloadFieldBuilder {
    pub(crate) display_name: ::std::option::Option<::std::string::String>,
    pub(crate) display_order: ::std::option::Option<i32>,
    pub(crate) r#type: ::std::option::Option<crate::types::ActionPayloadFieldType>,
    pub(crate) value: ::std::option::Option<::aws_smithy_types::Document>,
    pub(crate) allowed_values: ::std::option::Option<::std::vec::Vec<crate::types::ActionReviewPayloadFieldAllowedValue>>,
    pub(crate) required: ::std::option::Option<bool>,
}
impl ActionReviewPayloadFieldBuilder {
    /// <p>The name of the field.</p>
    pub fn display_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.display_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the field.</p>
    pub fn set_display_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.display_name = input;
        self
    }
    /// <p>The name of the field.</p>
    pub fn get_display_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.display_name
    }
    /// <p>The display order of fields in a payload.</p>
    pub fn display_order(mut self, input: i32) -> Self {
        self.display_order = ::std::option::Option::Some(input);
        self
    }
    /// <p>The display order of fields in a payload.</p>
    pub fn set_display_order(mut self, input: ::std::option::Option<i32>) -> Self {
        self.display_order = input;
        self
    }
    /// <p>The display order of fields in a payload.</p>
    pub fn get_display_order(&self) -> &::std::option::Option<i32> {
        &self.display_order
    }
    /// <p>The type of field.</p>
    pub fn r#type(mut self, input: crate::types::ActionPayloadFieldType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of field.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::ActionPayloadFieldType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The type of field.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::ActionPayloadFieldType> {
        &self.r#type
    }
    /// <p>The field value.</p>
    pub fn value(mut self, input: ::aws_smithy_types::Document) -> Self {
        self.value = ::std::option::Option::Some(input);
        self
    }
    /// <p>The field value.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::aws_smithy_types::Document>) -> Self {
        self.value = input;
        self
    }
    /// <p>The field value.</p>
    pub fn get_value(&self) -> &::std::option::Option<::aws_smithy_types::Document> {
        &self.value
    }
    /// Appends an item to `allowed_values`.
    ///
    /// To override the contents of this collection use [`set_allowed_values`](Self::set_allowed_values).
    ///
    /// <p>Information about the field values that an end user can use to provide to Amazon Q for Amazon Q to perform the requested plugin action.</p>
    pub fn allowed_values(mut self, input: crate::types::ActionReviewPayloadFieldAllowedValue) -> Self {
        let mut v = self.allowed_values.unwrap_or_default();
        v.push(input);
        self.allowed_values = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the field values that an end user can use to provide to Amazon Q for Amazon Q to perform the requested plugin action.</p>
    pub fn set_allowed_values(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ActionReviewPayloadFieldAllowedValue>>) -> Self {
        self.allowed_values = input;
        self
    }
    /// <p>Information about the field values that an end user can use to provide to Amazon Q for Amazon Q to perform the requested plugin action.</p>
    pub fn get_allowed_values(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ActionReviewPayloadFieldAllowedValue>> {
        &self.allowed_values
    }
    /// <p>Information about whether the field is required.</p>
    pub fn required(mut self, input: bool) -> Self {
        self.required = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about whether the field is required.</p>
    pub fn set_required(mut self, input: ::std::option::Option<bool>) -> Self {
        self.required = input;
        self
    }
    /// <p>Information about whether the field is required.</p>
    pub fn get_required(&self) -> &::std::option::Option<bool> {
        &self.required
    }
    /// Consumes the builder and constructs a [`ActionReviewPayloadField`](crate::types::ActionReviewPayloadField).
    pub fn build(self) -> crate::types::ActionReviewPayloadField {
        crate::types::ActionReviewPayloadField {
            display_name: self.display_name,
            display_order: self.display_order,
            r#type: self.r#type,
            value: self.value,
            allowed_values: self.allowed_values,
            required: self.required,
        }
    }
}
