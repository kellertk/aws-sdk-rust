// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_domain_contact_privacy_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_domain_contact_privacy::UpdateDomainContactPrivacyInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.domain_name {
        object.key("DomainName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.admin_privacy {
        object.key("AdminPrivacy").boolean(*var_2);
    }
    if let Some(var_3) = &input.registrant_privacy {
        object.key("RegistrantPrivacy").boolean(*var_3);
    }
    if let Some(var_4) = &input.tech_privacy {
        object.key("TechPrivacy").boolean(*var_4);
    }
    Ok(())
}
