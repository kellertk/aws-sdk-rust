// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_batch_get_named_query_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchGetNamedQueryInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.named_query_ids {
        let mut array_2 = object.key("NamedQueryIds").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_get_prepared_statement_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchGetPreparedStatementInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_4) = &input.prepared_statement_names {
        let mut array_5 = object.key("PreparedStatementNames").start_array();
        for item_6 in var_4 {
            {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.work_group {
        object.key("WorkGroup").string(var_7.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_get_query_execution_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchGetQueryExecutionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_8) = &input.query_execution_ids {
        let mut array_9 = object.key("QueryExecutionIds").start_array();
        for item_10 in var_8 {
            {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_data_catalog_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDataCatalogInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_11) = &input.name {
        object.key("Name").string(var_11.as_str());
    }
    if let Some(var_12) = &input.r#type {
        object.key("Type").string(var_12.as_str());
    }
    if let Some(var_13) = &input.description {
        object.key("Description").string(var_13.as_str());
    }
    if let Some(var_14) = &input.parameters {
        let mut object_15 = object.key("Parameters").start_object();
        for (key_16, value_17) in var_14 {
            {
                object_15.key(key_16).string(value_17.as_str());
            }
        }
        object_15.finish();
    }
    if let Some(var_18) = &input.tags {
        let mut array_19 = object.key("Tags").start_array();
        for item_20 in var_18 {
            {
                let mut object_21 = array_19.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_21, item_20)?;
                object_21.finish();
            }
        }
        array_19.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_named_query_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateNamedQueryInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_22) = &input.name {
        object.key("Name").string(var_22.as_str());
    }
    if let Some(var_23) = &input.description {
        object.key("Description").string(var_23.as_str());
    }
    if let Some(var_24) = &input.database {
        object.key("Database").string(var_24.as_str());
    }
    if let Some(var_25) = &input.query_string {
        object.key("QueryString").string(var_25.as_str());
    }
    if let Some(var_26) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_26.as_str());
    }
    if let Some(var_27) = &input.work_group {
        object.key("WorkGroup").string(var_27.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_prepared_statement_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreatePreparedStatementInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_28) = &input.statement_name {
        object.key("StatementName").string(var_28.as_str());
    }
    if let Some(var_29) = &input.work_group {
        object.key("WorkGroup").string(var_29.as_str());
    }
    if let Some(var_30) = &input.query_statement {
        object.key("QueryStatement").string(var_30.as_str());
    }
    if let Some(var_31) = &input.description {
        object.key("Description").string(var_31.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_work_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateWorkGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_32) = &input.name {
        object.key("Name").string(var_32.as_str());
    }
    if let Some(var_33) = &input.configuration {
        let mut object_34 = object.key("Configuration").start_object();
        crate::json_ser::serialize_structure_crate_model_work_group_configuration(
            &mut object_34,
            var_33,
        )?;
        object_34.finish();
    }
    if let Some(var_35) = &input.description {
        object.key("Description").string(var_35.as_str());
    }
    if let Some(var_36) = &input.tags {
        let mut array_37 = object.key("Tags").start_array();
        for item_38 in var_36 {
            {
                let mut object_39 = array_37.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_39, item_38)?;
                object_39.finish();
            }
        }
        array_37.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_data_catalog_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteDataCatalogInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_40) = &input.name {
        object.key("Name").string(var_40.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_named_query_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteNamedQueryInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_41) = &input.named_query_id {
        object.key("NamedQueryId").string(var_41.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_prepared_statement_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeletePreparedStatementInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_42) = &input.statement_name {
        object.key("StatementName").string(var_42.as_str());
    }
    if let Some(var_43) = &input.work_group {
        object.key("WorkGroup").string(var_43.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_work_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteWorkGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.work_group {
        object.key("WorkGroup").string(var_44.as_str());
    }
    if let Some(var_45) = &input.recursive_delete_option {
        object.key("RecursiveDeleteOption").boolean(*var_45);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_database_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDatabaseInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_46) = &input.catalog_name {
        object.key("CatalogName").string(var_46.as_str());
    }
    if let Some(var_47) = &input.database_name {
        object.key("DatabaseName").string(var_47.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_data_catalog_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDataCatalogInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_48) = &input.name {
        object.key("Name").string(var_48.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_named_query_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetNamedQueryInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_49) = &input.named_query_id {
        object.key("NamedQueryId").string(var_49.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_prepared_statement_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetPreparedStatementInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_50) = &input.statement_name {
        object.key("StatementName").string(var_50.as_str());
    }
    if let Some(var_51) = &input.work_group {
        object.key("WorkGroup").string(var_51.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_query_execution_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetQueryExecutionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_52) = &input.query_execution_id {
        object.key("QueryExecutionId").string(var_52.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_query_results_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetQueryResultsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_53) = &input.query_execution_id {
        object.key("QueryExecutionId").string(var_53.as_str());
    }
    if let Some(var_54) = &input.next_token {
        object.key("NextToken").string(var_54.as_str());
    }
    if let Some(var_55) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_55).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_table_metadata_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetTableMetadataInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_56) = &input.catalog_name {
        object.key("CatalogName").string(var_56.as_str());
    }
    if let Some(var_57) = &input.database_name {
        object.key("DatabaseName").string(var_57.as_str());
    }
    if let Some(var_58) = &input.table_name {
        object.key("TableName").string(var_58.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_work_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetWorkGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_59) = &input.work_group {
        object.key("WorkGroup").string(var_59.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_databases_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListDatabasesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_60) = &input.catalog_name {
        object.key("CatalogName").string(var_60.as_str());
    }
    if let Some(var_61) = &input.next_token {
        object.key("NextToken").string(var_61.as_str());
    }
    if let Some(var_62) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_62).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_data_catalogs_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListDataCatalogsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_63) = &input.next_token {
        object.key("NextToken").string(var_63.as_str());
    }
    if let Some(var_64) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_64).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_engine_versions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListEngineVersionsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_65) = &input.next_token {
        object.key("NextToken").string(var_65.as_str());
    }
    if let Some(var_66) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_66).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_named_queries_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListNamedQueriesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_67) = &input.next_token {
        object.key("NextToken").string(var_67.as_str());
    }
    if let Some(var_68) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_68).into()),
        );
    }
    if let Some(var_69) = &input.work_group {
        object.key("WorkGroup").string(var_69.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_prepared_statements_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListPreparedStatementsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_70) = &input.work_group {
        object.key("WorkGroup").string(var_70.as_str());
    }
    if let Some(var_71) = &input.next_token {
        object.key("NextToken").string(var_71.as_str());
    }
    if let Some(var_72) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_72).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_query_executions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListQueryExecutionsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_73) = &input.next_token {
        object.key("NextToken").string(var_73.as_str());
    }
    if let Some(var_74) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_74).into()),
        );
    }
    if let Some(var_75) = &input.work_group {
        object.key("WorkGroup").string(var_75.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_table_metadata_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTableMetadataInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_76) = &input.catalog_name {
        object.key("CatalogName").string(var_76.as_str());
    }
    if let Some(var_77) = &input.database_name {
        object.key("DatabaseName").string(var_77.as_str());
    }
    if let Some(var_78) = &input.expression {
        object.key("Expression").string(var_78.as_str());
    }
    if let Some(var_79) = &input.next_token {
        object.key("NextToken").string(var_79.as_str());
    }
    if let Some(var_80) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_80).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_81) = &input.resource_arn {
        object.key("ResourceARN").string(var_81.as_str());
    }
    if let Some(var_82) = &input.next_token {
        object.key("NextToken").string(var_82.as_str());
    }
    if let Some(var_83) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_83).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_work_groups_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListWorkGroupsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_84) = &input.next_token {
        object.key("NextToken").string(var_84.as_str());
    }
    if let Some(var_85) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_85).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_query_execution_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartQueryExecutionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_86) = &input.query_string {
        object.key("QueryString").string(var_86.as_str());
    }
    if let Some(var_87) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_87.as_str());
    }
    if let Some(var_88) = &input.query_execution_context {
        let mut object_89 = object.key("QueryExecutionContext").start_object();
        crate::json_ser::serialize_structure_crate_model_query_execution_context(
            &mut object_89,
            var_88,
        )?;
        object_89.finish();
    }
    if let Some(var_90) = &input.result_configuration {
        let mut object_91 = object.key("ResultConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_result_configuration(
            &mut object_91,
            var_90,
        )?;
        object_91.finish();
    }
    if let Some(var_92) = &input.work_group {
        object.key("WorkGroup").string(var_92.as_str());
    }
    if let Some(var_93) = &input.execution_parameters {
        let mut array_94 = object.key("ExecutionParameters").start_array();
        for item_95 in var_93 {
            {
                array_94.value().string(item_95.as_str());
            }
        }
        array_94.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_stop_query_execution_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopQueryExecutionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_96) = &input.query_execution_id {
        object.key("QueryExecutionId").string(var_96.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_97) = &input.resource_arn {
        object.key("ResourceARN").string(var_97.as_str());
    }
    if let Some(var_98) = &input.tags {
        let mut array_99 = object.key("Tags").start_array();
        for item_100 in var_98 {
            {
                let mut object_101 = array_99.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_101, item_100)?;
                object_101.finish();
            }
        }
        array_99.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_102) = &input.resource_arn {
        object.key("ResourceARN").string(var_102.as_str());
    }
    if let Some(var_103) = &input.tag_keys {
        let mut array_104 = object.key("TagKeys").start_array();
        for item_105 in var_103 {
            {
                array_104.value().string(item_105.as_str());
            }
        }
        array_104.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_data_catalog_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDataCatalogInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_106) = &input.name {
        object.key("Name").string(var_106.as_str());
    }
    if let Some(var_107) = &input.r#type {
        object.key("Type").string(var_107.as_str());
    }
    if let Some(var_108) = &input.description {
        object.key("Description").string(var_108.as_str());
    }
    if let Some(var_109) = &input.parameters {
        let mut object_110 = object.key("Parameters").start_object();
        for (key_111, value_112) in var_109 {
            {
                object_110.key(key_111).string(value_112.as_str());
            }
        }
        object_110.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_named_query_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateNamedQueryInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_113) = &input.named_query_id {
        object.key("NamedQueryId").string(var_113.as_str());
    }
    if let Some(var_114) = &input.name {
        object.key("Name").string(var_114.as_str());
    }
    if let Some(var_115) = &input.description {
        object.key("Description").string(var_115.as_str());
    }
    if let Some(var_116) = &input.query_string {
        object.key("QueryString").string(var_116.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_prepared_statement_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdatePreparedStatementInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_117) = &input.statement_name {
        object.key("StatementName").string(var_117.as_str());
    }
    if let Some(var_118) = &input.work_group {
        object.key("WorkGroup").string(var_118.as_str());
    }
    if let Some(var_119) = &input.query_statement {
        object.key("QueryStatement").string(var_119.as_str());
    }
    if let Some(var_120) = &input.description {
        object.key("Description").string(var_120.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_work_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateWorkGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_121) = &input.work_group {
        object.key("WorkGroup").string(var_121.as_str());
    }
    if let Some(var_122) = &input.description {
        object.key("Description").string(var_122.as_str());
    }
    if let Some(var_123) = &input.configuration_updates {
        let mut object_124 = object.key("ConfigurationUpdates").start_object();
        crate::json_ser::serialize_structure_crate_model_work_group_configuration_updates(
            &mut object_124,
            var_123,
        )?;
        object_124.finish();
    }
    if let Some(var_125) = &input.state {
        object.key("State").string(var_125.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_126) = &input.key {
        object.key("Key").string(var_126.as_str());
    }
    if let Some(var_127) = &input.value {
        object.key("Value").string(var_127.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_work_group_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::WorkGroupConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_128) = &input.result_configuration {
        let mut object_129 = object.key("ResultConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_result_configuration(
            &mut object_129,
            var_128,
        )?;
        object_129.finish();
    }
    if let Some(var_130) = &input.enforce_work_group_configuration {
        object
            .key("EnforceWorkGroupConfiguration")
            .boolean(*var_130);
    }
    if let Some(var_131) = &input.publish_cloud_watch_metrics_enabled {
        object
            .key("PublishCloudWatchMetricsEnabled")
            .boolean(*var_131);
    }
    if let Some(var_132) = &input.bytes_scanned_cutoff_per_query {
        object.key("BytesScannedCutoffPerQuery").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_132).into()),
        );
    }
    if let Some(var_133) = &input.requester_pays_enabled {
        object.key("RequesterPaysEnabled").boolean(*var_133);
    }
    if let Some(var_134) = &input.engine_version {
        let mut object_135 = object.key("EngineVersion").start_object();
        crate::json_ser::serialize_structure_crate_model_engine_version(&mut object_135, var_134)?;
        object_135.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_query_execution_context(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::QueryExecutionContext,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_136) = &input.database {
        object.key("Database").string(var_136.as_str());
    }
    if let Some(var_137) = &input.catalog {
        object.key("Catalog").string(var_137.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_result_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ResultConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_138) = &input.output_location {
        object.key("OutputLocation").string(var_138.as_str());
    }
    if let Some(var_139) = &input.encryption_configuration {
        let mut object_140 = object.key("EncryptionConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption_configuration(
            &mut object_140,
            var_139,
        )?;
        object_140.finish();
    }
    if let Some(var_141) = &input.expected_bucket_owner {
        object.key("ExpectedBucketOwner").string(var_141.as_str());
    }
    if let Some(var_142) = &input.acl_configuration {
        let mut object_143 = object.key("AclConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_acl_configuration(
            &mut object_143,
            var_142,
        )?;
        object_143.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_work_group_configuration_updates(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::WorkGroupConfigurationUpdates,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_144) = &input.enforce_work_group_configuration {
        object
            .key("EnforceWorkGroupConfiguration")
            .boolean(*var_144);
    }
    if let Some(var_145) = &input.result_configuration_updates {
        let mut object_146 = object.key("ResultConfigurationUpdates").start_object();
        crate::json_ser::serialize_structure_crate_model_result_configuration_updates(
            &mut object_146,
            var_145,
        )?;
        object_146.finish();
    }
    if let Some(var_147) = &input.publish_cloud_watch_metrics_enabled {
        object
            .key("PublishCloudWatchMetricsEnabled")
            .boolean(*var_147);
    }
    if let Some(var_148) = &input.bytes_scanned_cutoff_per_query {
        object.key("BytesScannedCutoffPerQuery").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_148).into()),
        );
    }
    if let Some(var_149) = &input.remove_bytes_scanned_cutoff_per_query {
        object
            .key("RemoveBytesScannedCutoffPerQuery")
            .boolean(*var_149);
    }
    if let Some(var_150) = &input.requester_pays_enabled {
        object.key("RequesterPaysEnabled").boolean(*var_150);
    }
    if let Some(var_151) = &input.engine_version {
        let mut object_152 = object.key("EngineVersion").start_object();
        crate::json_ser::serialize_structure_crate_model_engine_version(&mut object_152, var_151)?;
        object_152.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_engine_version(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EngineVersion,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_153) = &input.selected_engine_version {
        object.key("SelectedEngineVersion").string(var_153.as_str());
    }
    if let Some(var_154) = &input.effective_engine_version {
        object
            .key("EffectiveEngineVersion")
            .string(var_154.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_encryption_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EncryptionConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_155) = &input.encryption_option {
        object.key("EncryptionOption").string(var_155.as_str());
    }
    if let Some(var_156) = &input.kms_key {
        object.key("KmsKey").string(var_156.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_acl_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AclConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_157) = &input.s3_acl_option {
        object.key("S3AclOption").string(var_157.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_result_configuration_updates(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ResultConfigurationUpdates,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_158) = &input.output_location {
        object.key("OutputLocation").string(var_158.as_str());
    }
    if let Some(var_159) = &input.remove_output_location {
        object.key("RemoveOutputLocation").boolean(*var_159);
    }
    if let Some(var_160) = &input.encryption_configuration {
        let mut object_161 = object.key("EncryptionConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption_configuration(
            &mut object_161,
            var_160,
        )?;
        object_161.finish();
    }
    if let Some(var_162) = &input.remove_encryption_configuration {
        object
            .key("RemoveEncryptionConfiguration")
            .boolean(*var_162);
    }
    if let Some(var_163) = &input.expected_bucket_owner {
        object.key("ExpectedBucketOwner").string(var_163.as_str());
    }
    if let Some(var_164) = &input.remove_expected_bucket_owner {
        object.key("RemoveExpectedBucketOwner").boolean(*var_164);
    }
    if let Some(var_165) = &input.acl_configuration {
        let mut object_166 = object.key("AclConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_acl_configuration(
            &mut object_166,
            var_165,
        )?;
        object_166.finish();
    }
    if let Some(var_167) = &input.remove_acl_configuration {
        object.key("RemoveAclConfiguration").boolean(*var_167);
    }
    Ok(())
}
