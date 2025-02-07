// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeLakeFormationIdentityCenterConfiguration`](crate::operation::describe_lake_formation_identity_center_configuration::builders::DescribeLakeFormationIdentityCenterConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`catalog_id(impl Into<String>)`](crate::operation::describe_lake_formation_identity_center_configuration::builders::DescribeLakeFormationIdentityCenterConfigurationFluentBuilder::catalog_id) / [`set_catalog_id(Option<String>)`](crate::operation::describe_lake_formation_identity_center_configuration::builders::DescribeLakeFormationIdentityCenterConfigurationFluentBuilder::set_catalog_id):<br>required: **false**<br><p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment.</p><br>
    /// - On success, responds with [`DescribeLakeFormationIdentityCenterConfigurationOutput`](crate::operation::describe_lake_formation_identity_center_configuration::DescribeLakeFormationIdentityCenterConfigurationOutput) with field(s):
    ///   - [`catalog_id(Option<String>)`](crate::operation::describe_lake_formation_identity_center_configuration::DescribeLakeFormationIdentityCenterConfigurationOutput::catalog_id): <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment.</p>
    ///   - [`instance_arn(Option<String>)`](crate::operation::describe_lake_formation_identity_center_configuration::DescribeLakeFormationIdentityCenterConfigurationOutput::instance_arn): <p>The Amazon Resource Name (ARN) of the connection.</p>
    ///   - [`application_arn(Option<String>)`](crate::operation::describe_lake_formation_identity_center_configuration::DescribeLakeFormationIdentityCenterConfigurationOutput::application_arn): <p>The Amazon Resource Name (ARN) of the integrated application.</p>
    ///   - [`external_filtering(Option<ExternalFilteringConfiguration>)`](crate::operation::describe_lake_formation_identity_center_configuration::DescribeLakeFormationIdentityCenterConfigurationOutput::external_filtering): <p>Indicates if external filtering is enabled.</p>
    /// - On failure, responds with [`SdkError<DescribeLakeFormationIdentityCenterConfigurationError>`](crate::operation::describe_lake_formation_identity_center_configuration::DescribeLakeFormationIdentityCenterConfigurationError)
    pub fn describe_lake_formation_identity_center_configuration(&self) -> crate::operation::describe_lake_formation_identity_center_configuration::builders::DescribeLakeFormationIdentityCenterConfigurationFluentBuilder{
        crate::operation::describe_lake_formation_identity_center_configuration::builders::DescribeLakeFormationIdentityCenterConfigurationFluentBuilder::new(self.handle.clone())
    }
}
