// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn add_headers_create_extension(
    input: &crate::input::CreateExtensionInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_1) = &input.latest_version_number {
        let mut encoder = aws_smithy_types::primitive::Encoder::from(*inner_1);
        let formatted_2 = encoder.encode();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "latest_version_number",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("Latest-Version-Number", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_hosted_configuration_version(
    input: &crate::input::CreateHostedConfigurationVersionInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_3) = &input.description {
        let formatted_4 = AsRef::<str>::as_ref(inner_3);
        if !formatted_4.is_empty() {
            let header_value = formatted_4;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "description",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("Description", header_value);
        }
    }
    if let Some(inner_5) = &input.content_type {
        let formatted_6 = AsRef::<str>::as_ref(inner_5);
        if !formatted_6.is_empty() {
            let header_value = formatted_6;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "content_type",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("Content-Type", header_value);
        }
    }
    if let Some(inner_7) = &input.latest_version_number {
        let mut encoder = aws_smithy_types::primitive::Encoder::from(*inner_7);
        let formatted_8 = encoder.encode();
        if !formatted_8.is_empty() {
            let header_value = formatted_8;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "latest_version_number",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("Latest-Version-Number", header_value);
        }
    }
    Ok(builder)
}

pub fn deser_header_create_hosted_configuration_version_create_hosted_configuration_version_output_application_id(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Application-Id").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_hosted_configuration_version_create_hosted_configuration_version_output_configuration_profile_id(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Configuration-Profile-Id").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_payload_create_hosted_configuration_version_create_hosted_configuration_version_output_content(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<aws_smithy_types::Blob>,
    crate::error::CreateHostedConfigurationVersionError,
> {
    (!body.is_empty())
        .then(|| Ok(aws_smithy_types::Blob::new(body)))
        .transpose()
}

pub fn deser_header_create_hosted_configuration_version_create_hosted_configuration_version_output_content_type(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Type").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_hosted_configuration_version_create_hosted_configuration_version_output_description(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Description").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_hosted_configuration_version_create_hosted_configuration_version_output_version_number(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Version-Number").iter();
    let var_9 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_9.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_9.len()),
        ))
    } else {
        let mut var_9 = var_9;
        Ok(var_9.pop())
    }
}

pub fn deser_header_get_configuration_get_configuration_output_configuration_version(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Configuration-Version").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_payload_get_configuration_get_configuration_output_content(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<aws_smithy_types::Blob>,
    crate::error::GetConfigurationError,
> {
    (!body.is_empty())
        .then(|| Ok(aws_smithy_types::Blob::new(body)))
        .transpose()
}

pub fn deser_header_get_configuration_get_configuration_output_content_type(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Type").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_hosted_configuration_version_get_hosted_configuration_version_output_application_id(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Application-Id").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_hosted_configuration_version_get_hosted_configuration_version_output_configuration_profile_id(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Configuration-Profile-Id").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_payload_get_hosted_configuration_version_get_hosted_configuration_version_output_content(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<aws_smithy_types::Blob>,
    crate::error::GetHostedConfigurationVersionError,
> {
    (!body.is_empty())
        .then(|| Ok(aws_smithy_types::Blob::new(body)))
        .transpose()
}

pub fn deser_header_get_hosted_configuration_version_get_hosted_configuration_version_output_content_type(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Type").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_hosted_configuration_version_get_hosted_configuration_version_output_description(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Description").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_hosted_configuration_version_get_hosted_configuration_version_output_version_number(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Version-Number").iter();
    let var_10 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_10.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_10.len()),
        ))
    } else {
        let mut var_10 = var_10;
        Ok(var_10.pop())
    }
}
