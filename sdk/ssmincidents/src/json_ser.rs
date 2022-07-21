// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_replication_set_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateReplicationSetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.regions {
        let mut object_3 = object.key("regions").start_object();
        for (key_4, value_5) in var_2 {
            {
                let mut object_6 = object_3.key(key_4).start_object();
                crate::json_ser::serialize_structure_crate_model_region_map_input_value(
                    &mut object_6,
                    value_5,
                )?;
                object_6.finish();
            }
        }
        object_3.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_response_plan_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateResponsePlanInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_7) = &input.actions {
        let mut array_8 = object.key("actions").start_array();
        for item_9 in var_7 {
            {
                let mut object_10 = array_8.value().start_object();
                crate::json_ser::serialize_union_crate_model_action(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    if let Some(var_11) = &input.chat_channel {
        let mut object_12 = object.key("chatChannel").start_object();
        crate::json_ser::serialize_union_crate_model_chat_channel(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.client_token {
        object.key("clientToken").string(var_13.as_str());
    }
    if let Some(var_14) = &input.display_name {
        object.key("displayName").string(var_14.as_str());
    }
    if let Some(var_15) = &input.engagements {
        let mut array_16 = object.key("engagements").start_array();
        for item_17 in var_15 {
            {
                array_16.value().string(item_17.as_str());
            }
        }
        array_16.finish();
    }
    if let Some(var_18) = &input.incident_template {
        let mut object_19 = object.key("incidentTemplate").start_object();
        crate::json_ser::serialize_structure_crate_model_incident_template(&mut object_19, var_18)?;
        object_19.finish();
    }
    if let Some(var_20) = &input.name {
        object.key("name").string(var_20.as_str());
    }
    if let Some(var_21) = &input.tags {
        let mut object_22 = object.key("tags").start_object();
        for (key_23, value_24) in var_21 {
            {
                object_22.key(key_23).string(value_24.as_str());
            }
        }
        object_22.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_timeline_event_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateTimelineEventInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_25) = &input.client_token {
        object.key("clientToken").string(var_25.as_str());
    }
    if let Some(var_26) = &input.event_data {
        object.key("eventData").string(var_26.as_str());
    }
    if let Some(var_27) = &input.event_time {
        object
            .key("eventTime")
            .date_time(var_27, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_28) = &input.event_type {
        object.key("eventType").string(var_28.as_str());
    }
    if let Some(var_29) = &input.incident_record_arn {
        object.key("incidentRecordArn").string(var_29.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_incident_record_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteIncidentRecordInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_30) = &input.arn {
        object.key("arn").string(var_30.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_resource_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteResourcePolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_31) = &input.policy_id {
        object.key("policyId").string(var_31.as_str());
    }
    if let Some(var_32) = &input.resource_arn {
        object.key("resourceArn").string(var_32.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_response_plan_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteResponsePlanInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_33) = &input.arn {
        object.key("arn").string(var_33.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_timeline_event_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteTimelineEventInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_34) = &input.event_id {
        object.key("eventId").string(var_34.as_str());
    }
    if let Some(var_35) = &input.incident_record_arn {
        object.key("incidentRecordArn").string(var_35.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_resource_policies_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetResourcePoliciesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_36) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_36).into()),
        );
    }
    if let Some(var_37) = &input.next_token {
        object.key("nextToken").string(var_37.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_incident_records_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListIncidentRecordsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_38) = &input.filters {
        let mut array_39 = object.key("filters").start_array();
        for item_40 in var_38 {
            {
                let mut object_41 = array_39.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter(&mut object_41, item_40)?;
                object_41.finish();
            }
        }
        array_39.finish();
    }
    if let Some(var_42) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_42).into()),
        );
    }
    if let Some(var_43) = &input.next_token {
        object.key("nextToken").string(var_43.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_related_items_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListRelatedItemsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.incident_record_arn {
        object.key("incidentRecordArn").string(var_44.as_str());
    }
    if let Some(var_45) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_45).into()),
        );
    }
    if let Some(var_46) = &input.next_token {
        object.key("nextToken").string(var_46.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_replication_sets_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListReplicationSetsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_47) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_47).into()),
        );
    }
    if let Some(var_48) = &input.next_token {
        object.key("nextToken").string(var_48.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_response_plans_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListResponsePlansInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_49) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_49).into()),
        );
    }
    if let Some(var_50) = &input.next_token {
        object.key("nextToken").string(var_50.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_timeline_events_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTimelineEventsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_51) = &input.filters {
        let mut array_52 = object.key("filters").start_array();
        for item_53 in var_51 {
            {
                let mut object_54 = array_52.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter(&mut object_54, item_53)?;
                object_54.finish();
            }
        }
        array_52.finish();
    }
    if let Some(var_55) = &input.incident_record_arn {
        object.key("incidentRecordArn").string(var_55.as_str());
    }
    if let Some(var_56) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_56).into()),
        );
    }
    if let Some(var_57) = &input.next_token {
        object.key("nextToken").string(var_57.as_str());
    }
    if let Some(var_58) = &input.sort_by {
        object.key("sortBy").string(var_58.as_str());
    }
    if let Some(var_59) = &input.sort_order {
        object.key("sortOrder").string(var_59.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_resource_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutResourcePolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_60) = &input.policy {
        object.key("policy").string(var_60.as_str());
    }
    if let Some(var_61) = &input.resource_arn {
        object.key("resourceArn").string(var_61.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_incident_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartIncidentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_62) = &input.client_token {
        object.key("clientToken").string(var_62.as_str());
    }
    if let Some(var_63) = &input.impact {
        object.key("impact").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_63).into()),
        );
    }
    if let Some(var_64) = &input.related_items {
        let mut array_65 = object.key("relatedItems").start_array();
        for item_66 in var_64 {
            {
                let mut object_67 = array_65.value().start_object();
                crate::json_ser::serialize_structure_crate_model_related_item(
                    &mut object_67,
                    item_66,
                )?;
                object_67.finish();
            }
        }
        array_65.finish();
    }
    if let Some(var_68) = &input.response_plan_arn {
        object.key("responsePlanArn").string(var_68.as_str());
    }
    if let Some(var_69) = &input.title {
        object.key("title").string(var_69.as_str());
    }
    if let Some(var_70) = &input.trigger_details {
        let mut object_71 = object.key("triggerDetails").start_object();
        crate::json_ser::serialize_structure_crate_model_trigger_details(&mut object_71, var_70)?;
        object_71.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_72) = &input.tags {
        let mut object_73 = object.key("tags").start_object();
        for (key_74, value_75) in var_72 {
            {
                object_73.key(key_74).string(value_75.as_str());
            }
        }
        object_73.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_deletion_protection_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDeletionProtectionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_76) = &input.arn {
        object.key("arn").string(var_76.as_str());
    }
    if let Some(var_77) = &input.client_token {
        object.key("clientToken").string(var_77.as_str());
    }
    if let Some(var_78) = &input.deletion_protected {
        object.key("deletionProtected").boolean(*var_78);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_incident_record_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateIncidentRecordInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_79) = &input.arn {
        object.key("arn").string(var_79.as_str());
    }
    if let Some(var_80) = &input.chat_channel {
        let mut object_81 = object.key("chatChannel").start_object();
        crate::json_ser::serialize_union_crate_model_chat_channel(&mut object_81, var_80)?;
        object_81.finish();
    }
    if let Some(var_82) = &input.client_token {
        object.key("clientToken").string(var_82.as_str());
    }
    if let Some(var_83) = &input.impact {
        object.key("impact").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_83).into()),
        );
    }
    if let Some(var_84) = &input.notification_targets {
        let mut array_85 = object.key("notificationTargets").start_array();
        for item_86 in var_84 {
            {
                let mut object_87 = array_85.value().start_object();
                crate::json_ser::serialize_union_crate_model_notification_target_item(
                    &mut object_87,
                    item_86,
                )?;
                object_87.finish();
            }
        }
        array_85.finish();
    }
    if let Some(var_88) = &input.status {
        object.key("status").string(var_88.as_str());
    }
    if let Some(var_89) = &input.summary {
        object.key("summary").string(var_89.as_str());
    }
    if let Some(var_90) = &input.title {
        object.key("title").string(var_90.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_related_items_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateRelatedItemsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_91) = &input.client_token {
        object.key("clientToken").string(var_91.as_str());
    }
    if let Some(var_92) = &input.incident_record_arn {
        object.key("incidentRecordArn").string(var_92.as_str());
    }
    if let Some(var_93) = &input.related_items_update {
        let mut object_94 = object.key("relatedItemsUpdate").start_object();
        crate::json_ser::serialize_union_crate_model_related_items_update(&mut object_94, var_93)?;
        object_94.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_replication_set_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateReplicationSetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_95) = &input.actions {
        let mut array_96 = object.key("actions").start_array();
        for item_97 in var_95 {
            {
                let mut object_98 = array_96.value().start_object();
                crate::json_ser::serialize_union_crate_model_update_replication_set_action(
                    &mut object_98,
                    item_97,
                )?;
                object_98.finish();
            }
        }
        array_96.finish();
    }
    if let Some(var_99) = &input.arn {
        object.key("arn").string(var_99.as_str());
    }
    if let Some(var_100) = &input.client_token {
        object.key("clientToken").string(var_100.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_response_plan_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateResponsePlanInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_101) = &input.actions {
        let mut array_102 = object.key("actions").start_array();
        for item_103 in var_101 {
            {
                let mut object_104 = array_102.value().start_object();
                crate::json_ser::serialize_union_crate_model_action(&mut object_104, item_103)?;
                object_104.finish();
            }
        }
        array_102.finish();
    }
    if let Some(var_105) = &input.arn {
        object.key("arn").string(var_105.as_str());
    }
    if let Some(var_106) = &input.chat_channel {
        let mut object_107 = object.key("chatChannel").start_object();
        crate::json_ser::serialize_union_crate_model_chat_channel(&mut object_107, var_106)?;
        object_107.finish();
    }
    if let Some(var_108) = &input.client_token {
        object.key("clientToken").string(var_108.as_str());
    }
    if let Some(var_109) = &input.display_name {
        object.key("displayName").string(var_109.as_str());
    }
    if let Some(var_110) = &input.engagements {
        let mut array_111 = object.key("engagements").start_array();
        for item_112 in var_110 {
            {
                array_111.value().string(item_112.as_str());
            }
        }
        array_111.finish();
    }
    if let Some(var_113) = &input.incident_template_dedupe_string {
        object
            .key("incidentTemplateDedupeString")
            .string(var_113.as_str());
    }
    if let Some(var_114) = &input.incident_template_impact {
        object.key("incidentTemplateImpact").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_114).into()),
        );
    }
    if let Some(var_115) = &input.incident_template_notification_targets {
        let mut array_116 = object
            .key("incidentTemplateNotificationTargets")
            .start_array();
        for item_117 in var_115 {
            {
                let mut object_118 = array_116.value().start_object();
                crate::json_ser::serialize_union_crate_model_notification_target_item(
                    &mut object_118,
                    item_117,
                )?;
                object_118.finish();
            }
        }
        array_116.finish();
    }
    if let Some(var_119) = &input.incident_template_summary {
        object
            .key("incidentTemplateSummary")
            .string(var_119.as_str());
    }
    if let Some(var_120) = &input.incident_template_tags {
        let mut object_121 = object.key("incidentTemplateTags").start_object();
        for (key_122, value_123) in var_120 {
            {
                object_121.key(key_122).string(value_123.as_str());
            }
        }
        object_121.finish();
    }
    if let Some(var_124) = &input.incident_template_title {
        object.key("incidentTemplateTitle").string(var_124.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_timeline_event_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateTimelineEventInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_125) = &input.client_token {
        object.key("clientToken").string(var_125.as_str());
    }
    if let Some(var_126) = &input.event_data {
        object.key("eventData").string(var_126.as_str());
    }
    if let Some(var_127) = &input.event_id {
        object.key("eventId").string(var_127.as_str());
    }
    if let Some(var_128) = &input.event_time {
        object
            .key("eventTime")
            .date_time(var_128, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_129) = &input.event_type {
        object.key("eventType").string(var_129.as_str());
    }
    if let Some(var_130) = &input.incident_record_arn {
        object.key("incidentRecordArn").string(var_130.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_region_map_input_value(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RegionMapInputValue,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_131) = &input.sse_kms_key_id {
        object.key("sseKmsKeyId").string(var_131.as_str());
    }
    Ok(())
}

pub fn serialize_union_crate_model_action(
    object_10: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Action,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::Action::SsmAutomation(inner) => {
            let mut object_132 = object_10.key("ssmAutomation").start_object();
            crate::json_ser::serialize_structure_crate_model_ssm_automation(
                &mut object_132,
                inner,
            )?;
            object_132.finish();
        }
        crate::model::Action::Unknown => {
            return Err(aws_smithy_http::operation::SerializationError::unknown_variant("Action"))
        }
    }
    Ok(())
}

pub fn serialize_union_crate_model_chat_channel(
    object_12: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ChatChannel,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::ChatChannel::Empty(inner) => {
            let mut object_133 = object_12.key("empty").start_object();
            crate::json_ser::serialize_structure_crate_model_empty_chat_channel(
                &mut object_133,
                inner,
            )?;
            object_133.finish();
        }
        crate::model::ChatChannel::ChatbotSns(inner) => {
            let mut array_134 = object_12.key("chatbotSns").start_array();
            for item_135 in inner {
                {
                    array_134.value().string(item_135.as_str());
                }
            }
            array_134.finish();
        }
        crate::model::ChatChannel::Unknown => {
            return Err(
                aws_smithy_http::operation::SerializationError::unknown_variant("ChatChannel"),
            )
        }
    }
    Ok(())
}

pub fn serialize_structure_crate_model_incident_template(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IncidentTemplate,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_136) = &input.title {
        object.key("title").string(var_136.as_str());
    }
    if let Some(var_137) = &input.impact {
        object.key("impact").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_137).into()),
        );
    }
    if let Some(var_138) = &input.summary {
        object.key("summary").string(var_138.as_str());
    }
    if let Some(var_139) = &input.dedupe_string {
        object.key("dedupeString").string(var_139.as_str());
    }
    if let Some(var_140) = &input.notification_targets {
        let mut array_141 = object.key("notificationTargets").start_array();
        for item_142 in var_140 {
            {
                let mut object_143 = array_141.value().start_object();
                crate::json_ser::serialize_union_crate_model_notification_target_item(
                    &mut object_143,
                    item_142,
                )?;
                object_143.finish();
            }
        }
        array_141.finish();
    }
    if let Some(var_144) = &input.incident_tags {
        let mut object_145 = object.key("incidentTags").start_object();
        for (key_146, value_147) in var_144 {
            {
                object_145.key(key_146).string(value_147.as_str());
            }
        }
        object_145.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Filter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_148) = &input.key {
        object.key("key").string(var_148.as_str());
    }
    if let Some(var_149) = &input.condition {
        let mut object_150 = object.key("condition").start_object();
        crate::json_ser::serialize_union_crate_model_condition(&mut object_150, var_149)?;
        object_150.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_related_item(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RelatedItem,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_151) = &input.identifier {
        let mut object_152 = object.key("identifier").start_object();
        crate::json_ser::serialize_structure_crate_model_item_identifier(&mut object_152, var_151)?;
        object_152.finish();
    }
    if let Some(var_153) = &input.title {
        object.key("title").string(var_153.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_trigger_details(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TriggerDetails,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_154) = &input.source {
        object.key("source").string(var_154.as_str());
    }
    if let Some(var_155) = &input.trigger_arn {
        object.key("triggerArn").string(var_155.as_str());
    }
    if let Some(var_156) = &input.timestamp {
        object
            .key("timestamp")
            .date_time(var_156, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_157) = &input.raw_data {
        object.key("rawData").string(var_157.as_str());
    }
    Ok(())
}

pub fn serialize_union_crate_model_notification_target_item(
    object_87: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NotificationTargetItem,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::NotificationTargetItem::SnsTopicArn(inner) => {
            object_87.key("snsTopicArn").string(inner.as_str());
        }
        crate::model::NotificationTargetItem::Unknown => {
            return Err(
                aws_smithy_http::operation::SerializationError::unknown_variant(
                    "NotificationTargetItem",
                ),
            )
        }
    }
    Ok(())
}

pub fn serialize_union_crate_model_related_items_update(
    object_94: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RelatedItemsUpdate,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::RelatedItemsUpdate::ItemToAdd(inner) => {
            let mut object_158 = object_94.key("itemToAdd").start_object();
            crate::json_ser::serialize_structure_crate_model_related_item(&mut object_158, inner)?;
            object_158.finish();
        }
        crate::model::RelatedItemsUpdate::ItemToRemove(inner) => {
            let mut object_159 = object_94.key("itemToRemove").start_object();
            crate::json_ser::serialize_structure_crate_model_item_identifier(
                &mut object_159,
                inner,
            )?;
            object_159.finish();
        }
        crate::model::RelatedItemsUpdate::Unknown => {
            return Err(
                aws_smithy_http::operation::SerializationError::unknown_variant(
                    "RelatedItemsUpdate",
                ),
            )
        }
    }
    Ok(())
}

pub fn serialize_union_crate_model_update_replication_set_action(
    object_98: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateReplicationSetAction,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::UpdateReplicationSetAction::AddRegionAction(inner) => {
            let mut object_160 = object_98.key("addRegionAction").start_object();
            crate::json_ser::serialize_structure_crate_model_add_region_action(
                &mut object_160,
                inner,
            )?;
            object_160.finish();
        }
        crate::model::UpdateReplicationSetAction::DeleteRegionAction(inner) => {
            let mut object_161 = object_98.key("deleteRegionAction").start_object();
            crate::json_ser::serialize_structure_crate_model_delete_region_action(
                &mut object_161,
                inner,
            )?;
            object_161.finish();
        }
        crate::model::UpdateReplicationSetAction::Unknown => {
            return Err(
                aws_smithy_http::operation::SerializationError::unknown_variant(
                    "UpdateReplicationSetAction",
                ),
            )
        }
    }
    Ok(())
}

pub fn serialize_structure_crate_model_ssm_automation(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SsmAutomation,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_162) = &input.role_arn {
        object.key("roleArn").string(var_162.as_str());
    }
    if let Some(var_163) = &input.document_name {
        object.key("documentName").string(var_163.as_str());
    }
    if let Some(var_164) = &input.document_version {
        object.key("documentVersion").string(var_164.as_str());
    }
    if let Some(var_165) = &input.target_account {
        object.key("targetAccount").string(var_165.as_str());
    }
    if let Some(var_166) = &input.parameters {
        let mut object_167 = object.key("parameters").start_object();
        for (key_168, value_169) in var_166 {
            {
                let mut array_170 = object_167.key(key_168).start_array();
                for item_171 in value_169 {
                    {
                        array_170.value().string(item_171.as_str());
                    }
                }
                array_170.finish();
            }
        }
        object_167.finish();
    }
    if let Some(var_172) = &input.dynamic_parameters {
        let mut object_173 = object.key("dynamicParameters").start_object();
        for (key_174, value_175) in var_172 {
            {
                let mut object_176 = object_173.key(key_174).start_object();
                crate::json_ser::serialize_union_crate_model_dynamic_ssm_parameter_value(
                    &mut object_176,
                    value_175,
                )?;
                object_176.finish();
            }
        }
        object_173.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_empty_chat_channel(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EmptyChatChannel,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    let (_, _) = (object, input);
    Ok(())
}

pub fn serialize_union_crate_model_condition(
    object_150: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Condition,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::Condition::Before(inner) => {
            object_150
                .key("before")
                .date_time(inner, aws_smithy_types::date_time::Format::EpochSeconds)?;
        }
        crate::model::Condition::After(inner) => {
            object_150
                .key("after")
                .date_time(inner, aws_smithy_types::date_time::Format::EpochSeconds)?;
        }
        crate::model::Condition::Equals(inner) => {
            let mut object_177 = object_150.key("equals").start_object();
            crate::json_ser::serialize_union_crate_model_attribute_value_list(
                &mut object_177,
                inner,
            )?;
            object_177.finish();
        }
        crate::model::Condition::Unknown => {
            return Err(
                aws_smithy_http::operation::SerializationError::unknown_variant("Condition"),
            )
        }
    }
    Ok(())
}

pub fn serialize_structure_crate_model_item_identifier(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ItemIdentifier,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_178) = &input.value {
        let mut object_179 = object.key("value").start_object();
        crate::json_ser::serialize_union_crate_model_item_value(&mut object_179, var_178)?;
        object_179.finish();
    }
    if let Some(var_180) = &input.r#type {
        object.key("type").string(var_180.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_add_region_action(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AddRegionAction,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_181) = &input.region_name {
        object.key("regionName").string(var_181.as_str());
    }
    if let Some(var_182) = &input.sse_kms_key_id {
        object.key("sseKmsKeyId").string(var_182.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_delete_region_action(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeleteRegionAction,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_183) = &input.region_name {
        object.key("regionName").string(var_183.as_str());
    }
    Ok(())
}

pub fn serialize_union_crate_model_dynamic_ssm_parameter_value(
    object_176: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DynamicSsmParameterValue,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::DynamicSsmParameterValue::Variable(inner) => {
            object_176.key("variable").string(inner.as_str());
        }
        crate::model::DynamicSsmParameterValue::Unknown => {
            return Err(
                aws_smithy_http::operation::SerializationError::unknown_variant(
                    "DynamicSsmParameterValue",
                ),
            )
        }
    }
    Ok(())
}

pub fn serialize_union_crate_model_attribute_value_list(
    object_177: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AttributeValueList,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::AttributeValueList::StringValues(inner) => {
            let mut array_184 = object_177.key("stringValues").start_array();
            for item_185 in inner {
                {
                    array_184.value().string(item_185.as_str());
                }
            }
            array_184.finish();
        }
        crate::model::AttributeValueList::IntegerValues(inner) => {
            let mut array_186 = object_177.key("integerValues").start_array();
            for item_187 in inner {
                {
                    array_186.value().number(
                        #[allow(clippy::useless_conversion)]
                        aws_smithy_types::Number::NegInt((*item_187).into()),
                    );
                }
            }
            array_186.finish();
        }
        crate::model::AttributeValueList::Unknown => {
            return Err(
                aws_smithy_http::operation::SerializationError::unknown_variant(
                    "AttributeValueList",
                ),
            )
        }
    }
    Ok(())
}

pub fn serialize_union_crate_model_item_value(
    object_179: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ItemValue,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::ItemValue::Arn(inner) => {
            object_179.key("arn").string(inner.as_str());
        }
        crate::model::ItemValue::Url(inner) => {
            object_179.key("url").string(inner.as_str());
        }
        crate::model::ItemValue::MetricDefinition(inner) => {
            object_179.key("metricDefinition").string(inner.as_str());
        }
        crate::model::ItemValue::Unknown => {
            return Err(
                aws_smithy_http::operation::SerializationError::unknown_variant("ItemValue"),
            )
        }
    }
    Ok(())
}
