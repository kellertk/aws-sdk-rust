// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListDestinationsInput {
    /// Site ARN.
    pub site: ::std::option::Option<::std::string::String>,
    /// Maximum number of results to retrieve in a single call.
    pub max_results: ::std::option::Option<i32>,
    /// Pagination token returned when another page of data exists. Provide it in your next call to the API to receive the next page.
    pub next_token: ::std::option::Option<::std::string::String>,
    /// State of the destination.
    pub state: ::std::option::Option<crate::types::DestinationState>,
}
impl ListDestinationsInput {
    /// Site ARN.
    pub fn site(&self) -> ::std::option::Option<&str> {
        self.site.as_deref()
    }
    /// Maximum number of results to retrieve in a single call.
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// Pagination token returned when another page of data exists. Provide it in your next call to the API to receive the next page.
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// State of the destination.
    pub fn state(&self) -> ::std::option::Option<&crate::types::DestinationState> {
        self.state.as_ref()
    }
}
impl ListDestinationsInput {
    /// Creates a new builder-style object to manufacture [`ListDestinationsInput`](crate::operation::list_destinations::ListDestinationsInput).
    pub fn builder() -> crate::operation::list_destinations::builders::ListDestinationsInputBuilder {
        crate::operation::list_destinations::builders::ListDestinationsInputBuilder::default()
    }
}

/// A builder for [`ListDestinationsInput`](crate::operation::list_destinations::ListDestinationsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListDestinationsInputBuilder {
    pub(crate) site: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<crate::types::DestinationState>,
}
impl ListDestinationsInputBuilder {
    /// Site ARN.
    /// This field is required.
    pub fn site(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.site = ::std::option::Option::Some(input.into());
        self
    }
    /// Site ARN.
    pub fn set_site(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.site = input;
        self
    }
    /// Site ARN.
    pub fn get_site(&self) -> &::std::option::Option<::std::string::String> {
        &self.site
    }
    /// Maximum number of results to retrieve in a single call.
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// Maximum number of results to retrieve in a single call.
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Maximum number of results to retrieve in a single call.
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// Pagination token returned when another page of data exists. Provide it in your next call to the API to receive the next page.
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// Pagination token returned when another page of data exists. Provide it in your next call to the API to receive the next page.
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Pagination token returned when another page of data exists. Provide it in your next call to the API to receive the next page.
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// State of the destination.
    pub fn state(mut self, input: crate::types::DestinationState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// State of the destination.
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::DestinationState>) -> Self {
        self.state = input;
        self
    }
    /// State of the destination.
    pub fn get_state(&self) -> &::std::option::Option<crate::types::DestinationState> {
        &self.state
    }
    /// Consumes the builder and constructs a [`ListDestinationsInput`](crate::operation::list_destinations::ListDestinationsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::list_destinations::ListDestinationsInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::list_destinations::ListDestinationsInput {
            site: self.site,
            max_results: self.max_results,
            next_token: self.next_token,
            state: self.state,
        })
    }
}
