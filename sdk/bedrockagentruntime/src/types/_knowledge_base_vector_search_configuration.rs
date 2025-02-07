// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Knowledge base vector search configuration
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct KnowledgeBaseVectorSearchConfiguration {
    /// Top-K results to retrieve from knowledge base.
    pub number_of_results: i32,
}
impl KnowledgeBaseVectorSearchConfiguration {
    /// Top-K results to retrieve from knowledge base.
    pub fn number_of_results(&self) -> i32 {
        self.number_of_results
    }
}
impl KnowledgeBaseVectorSearchConfiguration {
    /// Creates a new builder-style object to manufacture [`KnowledgeBaseVectorSearchConfiguration`](crate::types::KnowledgeBaseVectorSearchConfiguration).
    pub fn builder() -> crate::types::builders::KnowledgeBaseVectorSearchConfigurationBuilder {
        crate::types::builders::KnowledgeBaseVectorSearchConfigurationBuilder::default()
    }
}

/// A builder for [`KnowledgeBaseVectorSearchConfiguration`](crate::types::KnowledgeBaseVectorSearchConfiguration).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct KnowledgeBaseVectorSearchConfigurationBuilder {
    pub(crate) number_of_results: ::std::option::Option<i32>,
}
impl KnowledgeBaseVectorSearchConfigurationBuilder {
    /// Top-K results to retrieve from knowledge base.
    /// This field is required.
    pub fn number_of_results(mut self, input: i32) -> Self {
        self.number_of_results = ::std::option::Option::Some(input);
        self
    }
    /// Top-K results to retrieve from knowledge base.
    pub fn set_number_of_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.number_of_results = input;
        self
    }
    /// Top-K results to retrieve from knowledge base.
    pub fn get_number_of_results(&self) -> &::std::option::Option<i32> {
        &self.number_of_results
    }
    /// Consumes the builder and constructs a [`KnowledgeBaseVectorSearchConfiguration`](crate::types::KnowledgeBaseVectorSearchConfiguration).
    /// This method will fail if any of the following fields are not set:
    /// - [`number_of_results`](crate::types::builders::KnowledgeBaseVectorSearchConfigurationBuilder::number_of_results)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::KnowledgeBaseVectorSearchConfiguration, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::KnowledgeBaseVectorSearchConfiguration {
            number_of_results: self.number_of_results.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "number_of_results",
                    "number_of_results was not specified but it is required when building KnowledgeBaseVectorSearchConfiguration",
                )
            })?,
        })
    }
}
