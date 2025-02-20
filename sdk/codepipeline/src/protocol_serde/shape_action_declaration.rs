// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_action_declaration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ActionDeclaration,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("name").string(input.name.as_str());
    }
    if let Some(var_1) = &input.action_type_id {
        #[allow(unused_mut)]
        let mut object_2 = object.key("actionTypeId").start_object();
        crate::protocol_serde::shape_action_type_id::ser_action_type_id(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.run_order {
        object.key("runOrder").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.configuration {
        #[allow(unused_mut)]
        let mut object_5 = object.key("configuration").start_object();
        for (key_6, value_7) in var_4 {
            {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    if let Some(var_8) = &input.output_artifacts {
        let mut array_9 = object.key("outputArtifacts").start_array();
        for item_10 in var_8 {
            {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::protocol_serde::shape_output_artifact::ser_output_artifact(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    if let Some(var_12) = &input.input_artifacts {
        let mut array_13 = object.key("inputArtifacts").start_array();
        for item_14 in var_12 {
            {
                #[allow(unused_mut)]
                let mut object_15 = array_13.value().start_object();
                crate::protocol_serde::shape_input_artifact::ser_input_artifact(&mut object_15, item_14)?;
                object_15.finish();
            }
        }
        array_13.finish();
    }
    if let Some(var_16) = &input.role_arn {
        object.key("roleArn").string(var_16.as_str());
    }
    if let Some(var_17) = &input.region {
        object.key("region").string(var_17.as_str());
    }
    if let Some(var_18) = &input.namespace {
        object.key("namespace").string(var_18.as_str());
    }
    Ok(())
}

pub(crate) fn de_action_declaration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::ActionDeclaration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ActionDeclarationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "name" => {
                            builder = builder.set_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "actionTypeId" => {
                            builder = builder.set_action_type_id(crate::protocol_serde::shape_action_type_id::de_action_type_id(tokens)?);
                        }
                        "runOrder" => {
                            builder = builder.set_run_order(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "configuration" => {
                            builder = builder.set_configuration(crate::protocol_serde::shape_action_configuration_map::de_action_configuration_map(
                                tokens,
                            )?);
                        }
                        "outputArtifacts" => {
                            builder =
                                builder.set_output_artifacts(crate::protocol_serde::shape_output_artifact_list::de_output_artifact_list(tokens)?);
                        }
                        "inputArtifacts" => {
                            builder = builder.set_input_artifacts(crate::protocol_serde::shape_input_artifact_list::de_input_artifact_list(tokens)?);
                        }
                        "roleArn" => {
                            builder = builder.set_role_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "region" => {
                            builder = builder.set_region(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "namespace" => {
                            builder = builder.set_namespace(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(crate::serde_util::action_declaration_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
