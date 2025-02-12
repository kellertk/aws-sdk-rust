// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListIngestionJobs`](crate::operation::list_ingestion_jobs::builders::ListIngestionJobsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_ingestion_jobs::builders::ListIngestionJobsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`knowledge_base_id(impl Into<String>)`](crate::operation::list_ingestion_jobs::builders::ListIngestionJobsFluentBuilder::knowledge_base_id) / [`set_knowledge_base_id(Option<String>)`](crate::operation::list_ingestion_jobs::builders::ListIngestionJobsFluentBuilder::set_knowledge_base_id):<br>required: **true**<br>Identifier for a resource.<br>
    ///   - [`data_source_id(impl Into<String>)`](crate::operation::list_ingestion_jobs::builders::ListIngestionJobsFluentBuilder::data_source_id) / [`set_data_source_id(Option<String>)`](crate::operation::list_ingestion_jobs::builders::ListIngestionJobsFluentBuilder::set_data_source_id):<br>required: **true**<br>Identifier for a resource.<br>
    ///   - [`filters(IngestionJobFilter)`](crate::operation::list_ingestion_jobs::builders::ListIngestionJobsFluentBuilder::filters) / [`set_filters(Option<Vec::<IngestionJobFilter>>)`](crate::operation::list_ingestion_jobs::builders::ListIngestionJobsFluentBuilder::set_filters):<br>required: **false**<br>List of IngestionJobFilters<br>
    ///   - [`sort_by(IngestionJobSortBy)`](crate::operation::list_ingestion_jobs::builders::ListIngestionJobsFluentBuilder::sort_by) / [`set_sort_by(Option<IngestionJobSortBy>)`](crate::operation::list_ingestion_jobs::builders::ListIngestionJobsFluentBuilder::set_sort_by):<br>required: **false**<br>Sorts the response returned by ListIngestionJobs operation.<br>
    ///   - [`max_results(i32)`](crate::operation::list_ingestion_jobs::builders::ListIngestionJobsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_ingestion_jobs::builders::ListIngestionJobsFluentBuilder::set_max_results):<br>required: **false**<br>Max Results.<br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_ingestion_jobs::builders::ListIngestionJobsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_ingestion_jobs::builders::ListIngestionJobsFluentBuilder::set_next_token):<br>required: **false**<br>Opaque continuation token of previous paginated response.<br>
    /// - On success, responds with [`ListIngestionJobsOutput`](crate::operation::list_ingestion_jobs::ListIngestionJobsOutput) with field(s):
    ///   - [`ingestion_job_summaries(Vec::<IngestionJobSummary>)`](crate::operation::list_ingestion_jobs::ListIngestionJobsOutput::ingestion_job_summaries): List of IngestionJobSummaries
    ///   - [`next_token(Option<String>)`](crate::operation::list_ingestion_jobs::ListIngestionJobsOutput::next_token): Opaque continuation token of previous paginated response.
    /// - On failure, responds with [`SdkError<ListIngestionJobsError>`](crate::operation::list_ingestion_jobs::ListIngestionJobsError)
    pub fn list_ingestion_jobs(&self) -> crate::operation::list_ingestion_jobs::builders::ListIngestionJobsFluentBuilder {
        crate::operation::list_ingestion_jobs::builders::ListIngestionJobsFluentBuilder::new(self.handle.clone())
    }
}
