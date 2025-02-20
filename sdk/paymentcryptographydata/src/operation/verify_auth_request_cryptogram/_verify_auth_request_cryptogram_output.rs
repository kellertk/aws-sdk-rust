// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VerifyAuthRequestCryptogramOutput {
    /// <p>The <code>keyARN</code> of the major encryption key that Amazon Web Services Payment Cryptography uses for ARQC verification.</p>
    pub key_arn: ::std::string::String,
    /// <p>The key check value (KCV) of the encryption key. The KCV is used to check if all parties holding a given key have the same key or to detect that a key has changed. Amazon Web Services Payment Cryptography calculates the KCV by using standard algorithms, typically by encrypting 8 or 16 bytes or "00" or "01" and then truncating the result to the first 3 bytes, or 6 hex digits, of the resulting cryptogram.</p>
    pub key_check_value: ::std::string::String,
    /// <p>The result for ARQC verification or ARPC generation within Amazon Web Services Payment Cryptography.</p>
    pub auth_response_value: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl VerifyAuthRequestCryptogramOutput {
    /// <p>The <code>keyARN</code> of the major encryption key that Amazon Web Services Payment Cryptography uses for ARQC verification.</p>
    pub fn key_arn(&self) -> &str {
        use std::ops::Deref;
        self.key_arn.deref()
    }
    /// <p>The key check value (KCV) of the encryption key. The KCV is used to check if all parties holding a given key have the same key or to detect that a key has changed. Amazon Web Services Payment Cryptography calculates the KCV by using standard algorithms, typically by encrypting 8 or 16 bytes or "00" or "01" and then truncating the result to the first 3 bytes, or 6 hex digits, of the resulting cryptogram.</p>
    pub fn key_check_value(&self) -> &str {
        use std::ops::Deref;
        self.key_check_value.deref()
    }
    /// <p>The result for ARQC verification or ARPC generation within Amazon Web Services Payment Cryptography.</p>
    pub fn auth_response_value(&self) -> ::std::option::Option<&str> {
        self.auth_response_value.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for VerifyAuthRequestCryptogramOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl VerifyAuthRequestCryptogramOutput {
    /// Creates a new builder-style object to manufacture [`VerifyAuthRequestCryptogramOutput`](crate::operation::verify_auth_request_cryptogram::VerifyAuthRequestCryptogramOutput).
    pub fn builder() -> crate::operation::verify_auth_request_cryptogram::builders::VerifyAuthRequestCryptogramOutputBuilder {
        crate::operation::verify_auth_request_cryptogram::builders::VerifyAuthRequestCryptogramOutputBuilder::default()
    }
}

/// A builder for [`VerifyAuthRequestCryptogramOutput`](crate::operation::verify_auth_request_cryptogram::VerifyAuthRequestCryptogramOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct VerifyAuthRequestCryptogramOutputBuilder {
    pub(crate) key_arn: ::std::option::Option<::std::string::String>,
    pub(crate) key_check_value: ::std::option::Option<::std::string::String>,
    pub(crate) auth_response_value: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl VerifyAuthRequestCryptogramOutputBuilder {
    /// <p>The <code>keyARN</code> of the major encryption key that Amazon Web Services Payment Cryptography uses for ARQC verification.</p>
    /// This field is required.
    pub fn key_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The <code>keyARN</code> of the major encryption key that Amazon Web Services Payment Cryptography uses for ARQC verification.</p>
    pub fn set_key_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_arn = input;
        self
    }
    /// <p>The <code>keyARN</code> of the major encryption key that Amazon Web Services Payment Cryptography uses for ARQC verification.</p>
    pub fn get_key_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_arn
    }
    /// <p>The key check value (KCV) of the encryption key. The KCV is used to check if all parties holding a given key have the same key or to detect that a key has changed. Amazon Web Services Payment Cryptography calculates the KCV by using standard algorithms, typically by encrypting 8 or 16 bytes or "00" or "01" and then truncating the result to the first 3 bytes, or 6 hex digits, of the resulting cryptogram.</p>
    /// This field is required.
    pub fn key_check_value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_check_value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The key check value (KCV) of the encryption key. The KCV is used to check if all parties holding a given key have the same key or to detect that a key has changed. Amazon Web Services Payment Cryptography calculates the KCV by using standard algorithms, typically by encrypting 8 or 16 bytes or "00" or "01" and then truncating the result to the first 3 bytes, or 6 hex digits, of the resulting cryptogram.</p>
    pub fn set_key_check_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_check_value = input;
        self
    }
    /// <p>The key check value (KCV) of the encryption key. The KCV is used to check if all parties holding a given key have the same key or to detect that a key has changed. Amazon Web Services Payment Cryptography calculates the KCV by using standard algorithms, typically by encrypting 8 or 16 bytes or "00" or "01" and then truncating the result to the first 3 bytes, or 6 hex digits, of the resulting cryptogram.</p>
    pub fn get_key_check_value(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_check_value
    }
    /// <p>The result for ARQC verification or ARPC generation within Amazon Web Services Payment Cryptography.</p>
    pub fn auth_response_value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.auth_response_value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The result for ARQC verification or ARPC generation within Amazon Web Services Payment Cryptography.</p>
    pub fn set_auth_response_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.auth_response_value = input;
        self
    }
    /// <p>The result for ARQC verification or ARPC generation within Amazon Web Services Payment Cryptography.</p>
    pub fn get_auth_response_value(&self) -> &::std::option::Option<::std::string::String> {
        &self.auth_response_value
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`VerifyAuthRequestCryptogramOutput`](crate::operation::verify_auth_request_cryptogram::VerifyAuthRequestCryptogramOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`key_arn`](crate::operation::verify_auth_request_cryptogram::builders::VerifyAuthRequestCryptogramOutputBuilder::key_arn)
    /// - [`key_check_value`](crate::operation::verify_auth_request_cryptogram::builders::VerifyAuthRequestCryptogramOutputBuilder::key_check_value)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::verify_auth_request_cryptogram::VerifyAuthRequestCryptogramOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::verify_auth_request_cryptogram::VerifyAuthRequestCryptogramOutput {
            key_arn: self.key_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "key_arn",
                    "key_arn was not specified but it is required when building VerifyAuthRequestCryptogramOutput",
                )
            })?,
            key_check_value: self.key_check_value.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "key_check_value",
                    "key_check_value was not specified but it is required when building VerifyAuthRequestCryptogramOutput",
                )
            })?,
            auth_response_value: self.auth_response_value,
            _request_id: self._request_id,
        })
    }
}
