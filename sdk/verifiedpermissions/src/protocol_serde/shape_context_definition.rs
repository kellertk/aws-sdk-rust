// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_context_definition(
    object_9: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ContextDefinition,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::ContextDefinition::ContextMap(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_9.key("contextMap").start_object();
            for (key_2, value_3) in inner {
                {
                    #[allow(unused_mut)]
                    let mut object_4 = object_1.key(key_2.as_str()).start_object();
                    crate::protocol_serde::shape_attribute_value::ser_attribute_value(&mut object_4, value_3)?;
                    object_4.finish();
                }
            }
            object_1.finish();
        }
        crate::types::ContextDefinition::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "ContextDefinition",
            ))
        }
    }
    Ok(())
}

pub(crate) fn de_context_definition<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::ContextDefinition>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    let mut variant = None;
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => loop {
            match tokens.next().transpose()? {
                Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                    let key = key.to_unescaped()?;
                    if key == "__type" {
                        ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                        continue;
                    }
                    if variant.is_some() {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                            "encountered mixed variants in union",
                        ));
                    }
                    variant = match key.as_ref() {
                        "contextMap" => Some(crate::types::ContextDefinition::ContextMap(
                            crate::protocol_serde::shape_context_map::de_context_map(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'contextMap' cannot be null")
                            })?,
                        )),
                        _ => {
                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                            Some(crate::types::ContextDefinition::Unknown)
                        }
                    };
                }
                other => {
                    return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )))
                }
            }
        },
        _ => {
            return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ))
        }
    }
    Ok(variant)
}
