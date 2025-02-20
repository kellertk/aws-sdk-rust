// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateKnowledgeBase`](crate::operation::create_knowledge_base::builders::CreateKnowledgeBaseFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_knowledge_base::builders::CreateKnowledgeBaseFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_knowledge_base::builders::CreateKnowledgeBaseFluentBuilder::set_client_token):<br>required: **false**<br>Client specified token used for idempotency checks<br>
    ///   - [`name(impl Into<String>)`](crate::operation::create_knowledge_base::builders::CreateKnowledgeBaseFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_knowledge_base::builders::CreateKnowledgeBaseFluentBuilder::set_name):<br>required: **true**<br>Name for a resource.<br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_knowledge_base::builders::CreateKnowledgeBaseFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_knowledge_base::builders::CreateKnowledgeBaseFluentBuilder::set_description):<br>required: **false**<br>Description of the Resource.<br>
    ///   - [`role_arn(impl Into<String>)`](crate::operation::create_knowledge_base::builders::CreateKnowledgeBaseFluentBuilder::role_arn) / [`set_role_arn(Option<String>)`](crate::operation::create_knowledge_base::builders::CreateKnowledgeBaseFluentBuilder::set_role_arn):<br>required: **true**<br>ARN of a IAM role.<br>
    ///   - [`knowledge_base_configuration(KnowledgeBaseConfiguration)`](crate::operation::create_knowledge_base::builders::CreateKnowledgeBaseFluentBuilder::knowledge_base_configuration) / [`set_knowledge_base_configuration(Option<KnowledgeBaseConfiguration>)`](crate::operation::create_knowledge_base::builders::CreateKnowledgeBaseFluentBuilder::set_knowledge_base_configuration):<br>required: **true**<br>Configures a bedrock knowledge base.<br>
    ///   - [`storage_configuration(StorageConfiguration)`](crate::operation::create_knowledge_base::builders::CreateKnowledgeBaseFluentBuilder::storage_configuration) / [`set_storage_configuration(Option<StorageConfiguration>)`](crate::operation::create_knowledge_base::builders::CreateKnowledgeBaseFluentBuilder::set_storage_configuration):<br>required: **true**<br>Configures the physical storage of ingested data in a knowledge base.<br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_knowledge_base::builders::CreateKnowledgeBaseFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_knowledge_base::builders::CreateKnowledgeBaseFluentBuilder::set_tags):<br>required: **false**<br>A map of tag keys and values<br>
    /// - On success, responds with [`CreateKnowledgeBaseOutput`](crate::operation::create_knowledge_base::CreateKnowledgeBaseOutput) with field(s):
    ///   - [`knowledge_base(Option<KnowledgeBase>)`](crate::operation::create_knowledge_base::CreateKnowledgeBaseOutput::knowledge_base): Contains the information of a knowledge base.
    /// - On failure, responds with [`SdkError<CreateKnowledgeBaseError>`](crate::operation::create_knowledge_base::CreateKnowledgeBaseError)
    pub fn create_knowledge_base(&self) -> crate::operation::create_knowledge_base::builders::CreateKnowledgeBaseFluentBuilder {
        crate::operation::create_knowledge_base::builders::CreateKnowledgeBaseFluentBuilder::new(self.handle.clone())
    }
}
