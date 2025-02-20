// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Input for Publish action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PublishInput {
    /// <p>The topic you want to publish to.</p>
    /// <p>If you don't specify a value for the <code>TopicArn</code> parameter, you must specify a value for the <code>PhoneNumber</code> or <code>TargetArn</code> parameters.</p>
    pub topic_arn: ::std::option::Option<::std::string::String>,
    /// <p>If you don't specify a value for the <code>TargetArn</code> parameter, you must specify a value for the <code>PhoneNumber</code> or <code>TopicArn</code> parameters.</p>
    pub target_arn: ::std::option::Option<::std::string::String>,
    /// <p>The phone number to which you want to deliver an SMS message. Use E.164 format.</p>
    /// <p>If you don't specify a value for the <code>PhoneNumber</code> parameter, you must specify a value for the <code>TargetArn</code> or <code>TopicArn</code> parameters.</p>
    pub phone_number: ::std::option::Option<::std::string::String>,
    /// <p>The message you want to send.</p>
    /// <p>If you are publishing to a topic and you want to send the same message to all transport protocols, include the text of the message as a String value. If you want to send different messages for each transport protocol, set the value of the <code>MessageStructure</code> parameter to <code>json</code> and use a JSON object for the <code>Message</code> parameter.</p>
    /// <p></p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>With the exception of SMS, messages must be UTF-8 encoded strings and at most 256 KB in size (262,144 bytes, not 262,144 characters).</p></li>
    /// <li>
    /// <p>For SMS, each message can contain up to 140 characters. This character limit depends on the encoding schema. For example, an SMS message can contain 160 GSM characters, 140 ASCII characters, or 70 UCS-2 characters.</p>
    /// <p>If you publish a message that exceeds this size limit, Amazon SNS sends the message as multiple messages, each fitting within the size limit. Messages aren't truncated mid-word but are cut off at whole-word boundaries.</p>
    /// <p>The total size limit for a single SMS <code>Publish</code> action is 1,600 characters.</p></li>
    /// </ul>
    /// <p>JSON-specific constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Keys in the JSON object that correspond to supported transport protocols must have simple JSON string values.</p></li>
    /// <li>
    /// <p>The values will be parsed (unescaped) before they are used in outgoing messages.</p></li>
    /// <li>
    /// <p>Outbound notifications are JSON encoded (meaning that the characters will be reescaped for sending).</p></li>
    /// <li>
    /// <p>Values have a minimum length of 0 (the empty string, "", is allowed).</p></li>
    /// <li>
    /// <p>Values have a maximum length bounded by the overall message size (so, including multiple protocols may limit message sizes).</p></li>
    /// <li>
    /// <p>Non-string values will cause the key to be ignored.</p></li>
    /// <li>
    /// <p>Keys that do not correspond to supported transport protocols are ignored.</p></li>
    /// <li>
    /// <p>Duplicate keys are not allowed.</p></li>
    /// <li>
    /// <p>Failure to parse or validate any key or value in the message will cause the <code>Publish</code> call to return an error (no partial delivery).</p></li>
    /// </ul>
    pub message: ::std::option::Option<::std::string::String>,
    /// <p>Optional parameter to be used as the "Subject" line when the message is delivered to email endpoints. This field will also be included, if present, in the standard JSON messages delivered to other endpoints.</p>
    /// <p>Constraints: Subjects must be ASCII text that begins with a letter, number, or punctuation mark; must not include line breaks or control characters; and must be less than 100 characters long.</p>
    pub subject: ::std::option::Option<::std::string::String>,
    /// <p>Set <code>MessageStructure</code> to <code>json</code> if you want to send a different message for each protocol. For example, using one publish action, you can send a short message to your SMS subscribers and a longer message to your email subscribers. If you set <code>MessageStructure</code> to <code>json</code>, the value of the <code>Message</code> parameter must:</p>
    /// <ul>
    /// <li>
    /// <p>be a syntactically valid JSON object; and</p></li>
    /// <li>
    /// <p>contain at least a top-level JSON key of "default" with a value that is a string.</p></li>
    /// </ul>
    /// <p>You can define other top-level keys that define the message you want to send to a specific transport protocol (e.g., "http").</p>
    /// <p>Valid value: <code>json</code></p>
    pub message_structure: ::std::option::Option<::std::string::String>,
    /// <p>Message attributes for Publish action.</p>
    pub message_attributes: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::MessageAttributeValue>>,
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics. The <code>MessageDeduplicationId</code> can contain up to 128 alphanumeric characters <code>(a-z, A-Z, 0-9)</code> and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~)</code>.</p>
    /// <p>Every message must have a unique <code>MessageDeduplicationId</code>, which is a token used for deduplication of sent messages. If a message with a particular <code>MessageDeduplicationId</code> is sent successfully, any message sent with the same <code>MessageDeduplicationId</code> during the 5-minute deduplication interval is treated as a duplicate.</p>
    /// <p>If the topic has <code>ContentBasedDeduplication</code> set, the system generates a <code>MessageDeduplicationId</code> based on the contents of the message. Your <code>MessageDeduplicationId</code> overrides the generated one.</p>
    pub message_deduplication_id: ::std::option::Option<::std::string::String>,
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics. The <code>MessageGroupId</code> can contain up to 128 alphanumeric characters <code>(a-z, A-Z, 0-9)</code> and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~)</code>.</p>
    /// <p>The <code>MessageGroupId</code> is a tag that specifies that a message belongs to a specific message group. Messages that belong to the same message group are processed in a FIFO manner (however, messages in different message groups might be processed out of order). Every message must include a <code>MessageGroupId</code>.</p>
    pub message_group_id: ::std::option::Option<::std::string::String>,
}
impl PublishInput {
    /// <p>The topic you want to publish to.</p>
    /// <p>If you don't specify a value for the <code>TopicArn</code> parameter, you must specify a value for the <code>PhoneNumber</code> or <code>TargetArn</code> parameters.</p>
    pub fn topic_arn(&self) -> ::std::option::Option<&str> {
        self.topic_arn.as_deref()
    }
    /// <p>If you don't specify a value for the <code>TargetArn</code> parameter, you must specify a value for the <code>PhoneNumber</code> or <code>TopicArn</code> parameters.</p>
    pub fn target_arn(&self) -> ::std::option::Option<&str> {
        self.target_arn.as_deref()
    }
    /// <p>The phone number to which you want to deliver an SMS message. Use E.164 format.</p>
    /// <p>If you don't specify a value for the <code>PhoneNumber</code> parameter, you must specify a value for the <code>TargetArn</code> or <code>TopicArn</code> parameters.</p>
    pub fn phone_number(&self) -> ::std::option::Option<&str> {
        self.phone_number.as_deref()
    }
    /// <p>The message you want to send.</p>
    /// <p>If you are publishing to a topic and you want to send the same message to all transport protocols, include the text of the message as a String value. If you want to send different messages for each transport protocol, set the value of the <code>MessageStructure</code> parameter to <code>json</code> and use a JSON object for the <code>Message</code> parameter.</p>
    /// <p></p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>With the exception of SMS, messages must be UTF-8 encoded strings and at most 256 KB in size (262,144 bytes, not 262,144 characters).</p></li>
    /// <li>
    /// <p>For SMS, each message can contain up to 140 characters. This character limit depends on the encoding schema. For example, an SMS message can contain 160 GSM characters, 140 ASCII characters, or 70 UCS-2 characters.</p>
    /// <p>If you publish a message that exceeds this size limit, Amazon SNS sends the message as multiple messages, each fitting within the size limit. Messages aren't truncated mid-word but are cut off at whole-word boundaries.</p>
    /// <p>The total size limit for a single SMS <code>Publish</code> action is 1,600 characters.</p></li>
    /// </ul>
    /// <p>JSON-specific constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Keys in the JSON object that correspond to supported transport protocols must have simple JSON string values.</p></li>
    /// <li>
    /// <p>The values will be parsed (unescaped) before they are used in outgoing messages.</p></li>
    /// <li>
    /// <p>Outbound notifications are JSON encoded (meaning that the characters will be reescaped for sending).</p></li>
    /// <li>
    /// <p>Values have a minimum length of 0 (the empty string, "", is allowed).</p></li>
    /// <li>
    /// <p>Values have a maximum length bounded by the overall message size (so, including multiple protocols may limit message sizes).</p></li>
    /// <li>
    /// <p>Non-string values will cause the key to be ignored.</p></li>
    /// <li>
    /// <p>Keys that do not correspond to supported transport protocols are ignored.</p></li>
    /// <li>
    /// <p>Duplicate keys are not allowed.</p></li>
    /// <li>
    /// <p>Failure to parse or validate any key or value in the message will cause the <code>Publish</code> call to return an error (no partial delivery).</p></li>
    /// </ul>
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
    /// <p>Optional parameter to be used as the "Subject" line when the message is delivered to email endpoints. This field will also be included, if present, in the standard JSON messages delivered to other endpoints.</p>
    /// <p>Constraints: Subjects must be ASCII text that begins with a letter, number, or punctuation mark; must not include line breaks or control characters; and must be less than 100 characters long.</p>
    pub fn subject(&self) -> ::std::option::Option<&str> {
        self.subject.as_deref()
    }
    /// <p>Set <code>MessageStructure</code> to <code>json</code> if you want to send a different message for each protocol. For example, using one publish action, you can send a short message to your SMS subscribers and a longer message to your email subscribers. If you set <code>MessageStructure</code> to <code>json</code>, the value of the <code>Message</code> parameter must:</p>
    /// <ul>
    /// <li>
    /// <p>be a syntactically valid JSON object; and</p></li>
    /// <li>
    /// <p>contain at least a top-level JSON key of "default" with a value that is a string.</p></li>
    /// </ul>
    /// <p>You can define other top-level keys that define the message you want to send to a specific transport protocol (e.g., "http").</p>
    /// <p>Valid value: <code>json</code></p>
    pub fn message_structure(&self) -> ::std::option::Option<&str> {
        self.message_structure.as_deref()
    }
    /// <p>Message attributes for Publish action.</p>
    pub fn message_attributes(
        &self,
    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, crate::types::MessageAttributeValue>> {
        self.message_attributes.as_ref()
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics. The <code>MessageDeduplicationId</code> can contain up to 128 alphanumeric characters <code>(a-z, A-Z, 0-9)</code> and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~)</code>.</p>
    /// <p>Every message must have a unique <code>MessageDeduplicationId</code>, which is a token used for deduplication of sent messages. If a message with a particular <code>MessageDeduplicationId</code> is sent successfully, any message sent with the same <code>MessageDeduplicationId</code> during the 5-minute deduplication interval is treated as a duplicate.</p>
    /// <p>If the topic has <code>ContentBasedDeduplication</code> set, the system generates a <code>MessageDeduplicationId</code> based on the contents of the message. Your <code>MessageDeduplicationId</code> overrides the generated one.</p>
    pub fn message_deduplication_id(&self) -> ::std::option::Option<&str> {
        self.message_deduplication_id.as_deref()
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics. The <code>MessageGroupId</code> can contain up to 128 alphanumeric characters <code>(a-z, A-Z, 0-9)</code> and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~)</code>.</p>
    /// <p>The <code>MessageGroupId</code> is a tag that specifies that a message belongs to a specific message group. Messages that belong to the same message group are processed in a FIFO manner (however, messages in different message groups might be processed out of order). Every message must include a <code>MessageGroupId</code>.</p>
    pub fn message_group_id(&self) -> ::std::option::Option<&str> {
        self.message_group_id.as_deref()
    }
}
impl PublishInput {
    /// Creates a new builder-style object to manufacture [`PublishInput`](crate::operation::publish::PublishInput).
    pub fn builder() -> crate::operation::publish::builders::PublishInputBuilder {
        crate::operation::publish::builders::PublishInputBuilder::default()
    }
}

/// A builder for [`PublishInput`](crate::operation::publish::PublishInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PublishInputBuilder {
    pub(crate) topic_arn: ::std::option::Option<::std::string::String>,
    pub(crate) target_arn: ::std::option::Option<::std::string::String>,
    pub(crate) phone_number: ::std::option::Option<::std::string::String>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
    pub(crate) subject: ::std::option::Option<::std::string::String>,
    pub(crate) message_structure: ::std::option::Option<::std::string::String>,
    pub(crate) message_attributes: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::MessageAttributeValue>>,
    pub(crate) message_deduplication_id: ::std::option::Option<::std::string::String>,
    pub(crate) message_group_id: ::std::option::Option<::std::string::String>,
}
impl PublishInputBuilder {
    /// <p>The topic you want to publish to.</p>
    /// <p>If you don't specify a value for the <code>TopicArn</code> parameter, you must specify a value for the <code>PhoneNumber</code> or <code>TargetArn</code> parameters.</p>
    pub fn topic_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.topic_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The topic you want to publish to.</p>
    /// <p>If you don't specify a value for the <code>TopicArn</code> parameter, you must specify a value for the <code>PhoneNumber</code> or <code>TargetArn</code> parameters.</p>
    pub fn set_topic_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.topic_arn = input;
        self
    }
    /// <p>The topic you want to publish to.</p>
    /// <p>If you don't specify a value for the <code>TopicArn</code> parameter, you must specify a value for the <code>PhoneNumber</code> or <code>TargetArn</code> parameters.</p>
    pub fn get_topic_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.topic_arn
    }
    /// <p>If you don't specify a value for the <code>TargetArn</code> parameter, you must specify a value for the <code>PhoneNumber</code> or <code>TopicArn</code> parameters.</p>
    pub fn target_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.target_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If you don't specify a value for the <code>TargetArn</code> parameter, you must specify a value for the <code>PhoneNumber</code> or <code>TopicArn</code> parameters.</p>
    pub fn set_target_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.target_arn = input;
        self
    }
    /// <p>If you don't specify a value for the <code>TargetArn</code> parameter, you must specify a value for the <code>PhoneNumber</code> or <code>TopicArn</code> parameters.</p>
    pub fn get_target_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.target_arn
    }
    /// <p>The phone number to which you want to deliver an SMS message. Use E.164 format.</p>
    /// <p>If you don't specify a value for the <code>PhoneNumber</code> parameter, you must specify a value for the <code>TargetArn</code> or <code>TopicArn</code> parameters.</p>
    pub fn phone_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.phone_number = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The phone number to which you want to deliver an SMS message. Use E.164 format.</p>
    /// <p>If you don't specify a value for the <code>PhoneNumber</code> parameter, you must specify a value for the <code>TargetArn</code> or <code>TopicArn</code> parameters.</p>
    pub fn set_phone_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.phone_number = input;
        self
    }
    /// <p>The phone number to which you want to deliver an SMS message. Use E.164 format.</p>
    /// <p>If you don't specify a value for the <code>PhoneNumber</code> parameter, you must specify a value for the <code>TargetArn</code> or <code>TopicArn</code> parameters.</p>
    pub fn get_phone_number(&self) -> &::std::option::Option<::std::string::String> {
        &self.phone_number
    }
    /// <p>The message you want to send.</p>
    /// <p>If you are publishing to a topic and you want to send the same message to all transport protocols, include the text of the message as a String value. If you want to send different messages for each transport protocol, set the value of the <code>MessageStructure</code> parameter to <code>json</code> and use a JSON object for the <code>Message</code> parameter.</p>
    /// <p></p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>With the exception of SMS, messages must be UTF-8 encoded strings and at most 256 KB in size (262,144 bytes, not 262,144 characters).</p></li>
    /// <li>
    /// <p>For SMS, each message can contain up to 140 characters. This character limit depends on the encoding schema. For example, an SMS message can contain 160 GSM characters, 140 ASCII characters, or 70 UCS-2 characters.</p>
    /// <p>If you publish a message that exceeds this size limit, Amazon SNS sends the message as multiple messages, each fitting within the size limit. Messages aren't truncated mid-word but are cut off at whole-word boundaries.</p>
    /// <p>The total size limit for a single SMS <code>Publish</code> action is 1,600 characters.</p></li>
    /// </ul>
    /// <p>JSON-specific constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Keys in the JSON object that correspond to supported transport protocols must have simple JSON string values.</p></li>
    /// <li>
    /// <p>The values will be parsed (unescaped) before they are used in outgoing messages.</p></li>
    /// <li>
    /// <p>Outbound notifications are JSON encoded (meaning that the characters will be reescaped for sending).</p></li>
    /// <li>
    /// <p>Values have a minimum length of 0 (the empty string, "", is allowed).</p></li>
    /// <li>
    /// <p>Values have a maximum length bounded by the overall message size (so, including multiple protocols may limit message sizes).</p></li>
    /// <li>
    /// <p>Non-string values will cause the key to be ignored.</p></li>
    /// <li>
    /// <p>Keys that do not correspond to supported transport protocols are ignored.</p></li>
    /// <li>
    /// <p>Duplicate keys are not allowed.</p></li>
    /// <li>
    /// <p>Failure to parse or validate any key or value in the message will cause the <code>Publish</code> call to return an error (no partial delivery).</p></li>
    /// </ul>
    /// This field is required.
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The message you want to send.</p>
    /// <p>If you are publishing to a topic and you want to send the same message to all transport protocols, include the text of the message as a String value. If you want to send different messages for each transport protocol, set the value of the <code>MessageStructure</code> parameter to <code>json</code> and use a JSON object for the <code>Message</code> parameter.</p>
    /// <p></p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>With the exception of SMS, messages must be UTF-8 encoded strings and at most 256 KB in size (262,144 bytes, not 262,144 characters).</p></li>
    /// <li>
    /// <p>For SMS, each message can contain up to 140 characters. This character limit depends on the encoding schema. For example, an SMS message can contain 160 GSM characters, 140 ASCII characters, or 70 UCS-2 characters.</p>
    /// <p>If you publish a message that exceeds this size limit, Amazon SNS sends the message as multiple messages, each fitting within the size limit. Messages aren't truncated mid-word but are cut off at whole-word boundaries.</p>
    /// <p>The total size limit for a single SMS <code>Publish</code> action is 1,600 characters.</p></li>
    /// </ul>
    /// <p>JSON-specific constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Keys in the JSON object that correspond to supported transport protocols must have simple JSON string values.</p></li>
    /// <li>
    /// <p>The values will be parsed (unescaped) before they are used in outgoing messages.</p></li>
    /// <li>
    /// <p>Outbound notifications are JSON encoded (meaning that the characters will be reescaped for sending).</p></li>
    /// <li>
    /// <p>Values have a minimum length of 0 (the empty string, "", is allowed).</p></li>
    /// <li>
    /// <p>Values have a maximum length bounded by the overall message size (so, including multiple protocols may limit message sizes).</p></li>
    /// <li>
    /// <p>Non-string values will cause the key to be ignored.</p></li>
    /// <li>
    /// <p>Keys that do not correspond to supported transport protocols are ignored.</p></li>
    /// <li>
    /// <p>Duplicate keys are not allowed.</p></li>
    /// <li>
    /// <p>Failure to parse or validate any key or value in the message will cause the <code>Publish</code> call to return an error (no partial delivery).</p></li>
    /// </ul>
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// <p>The message you want to send.</p>
    /// <p>If you are publishing to a topic and you want to send the same message to all transport protocols, include the text of the message as a String value. If you want to send different messages for each transport protocol, set the value of the <code>MessageStructure</code> parameter to <code>json</code> and use a JSON object for the <code>Message</code> parameter.</p>
    /// <p></p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>With the exception of SMS, messages must be UTF-8 encoded strings and at most 256 KB in size (262,144 bytes, not 262,144 characters).</p></li>
    /// <li>
    /// <p>For SMS, each message can contain up to 140 characters. This character limit depends on the encoding schema. For example, an SMS message can contain 160 GSM characters, 140 ASCII characters, or 70 UCS-2 characters.</p>
    /// <p>If you publish a message that exceeds this size limit, Amazon SNS sends the message as multiple messages, each fitting within the size limit. Messages aren't truncated mid-word but are cut off at whole-word boundaries.</p>
    /// <p>The total size limit for a single SMS <code>Publish</code> action is 1,600 characters.</p></li>
    /// </ul>
    /// <p>JSON-specific constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Keys in the JSON object that correspond to supported transport protocols must have simple JSON string values.</p></li>
    /// <li>
    /// <p>The values will be parsed (unescaped) before they are used in outgoing messages.</p></li>
    /// <li>
    /// <p>Outbound notifications are JSON encoded (meaning that the characters will be reescaped for sending).</p></li>
    /// <li>
    /// <p>Values have a minimum length of 0 (the empty string, "", is allowed).</p></li>
    /// <li>
    /// <p>Values have a maximum length bounded by the overall message size (so, including multiple protocols may limit message sizes).</p></li>
    /// <li>
    /// <p>Non-string values will cause the key to be ignored.</p></li>
    /// <li>
    /// <p>Keys that do not correspond to supported transport protocols are ignored.</p></li>
    /// <li>
    /// <p>Duplicate keys are not allowed.</p></li>
    /// <li>
    /// <p>Failure to parse or validate any key or value in the message will cause the <code>Publish</code> call to return an error (no partial delivery).</p></li>
    /// </ul>
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.message
    }
    /// <p>Optional parameter to be used as the "Subject" line when the message is delivered to email endpoints. This field will also be included, if present, in the standard JSON messages delivered to other endpoints.</p>
    /// <p>Constraints: Subjects must be ASCII text that begins with a letter, number, or punctuation mark; must not include line breaks or control characters; and must be less than 100 characters long.</p>
    pub fn subject(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.subject = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Optional parameter to be used as the "Subject" line when the message is delivered to email endpoints. This field will also be included, if present, in the standard JSON messages delivered to other endpoints.</p>
    /// <p>Constraints: Subjects must be ASCII text that begins with a letter, number, or punctuation mark; must not include line breaks or control characters; and must be less than 100 characters long.</p>
    pub fn set_subject(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.subject = input;
        self
    }
    /// <p>Optional parameter to be used as the "Subject" line when the message is delivered to email endpoints. This field will also be included, if present, in the standard JSON messages delivered to other endpoints.</p>
    /// <p>Constraints: Subjects must be ASCII text that begins with a letter, number, or punctuation mark; must not include line breaks or control characters; and must be less than 100 characters long.</p>
    pub fn get_subject(&self) -> &::std::option::Option<::std::string::String> {
        &self.subject
    }
    /// <p>Set <code>MessageStructure</code> to <code>json</code> if you want to send a different message for each protocol. For example, using one publish action, you can send a short message to your SMS subscribers and a longer message to your email subscribers. If you set <code>MessageStructure</code> to <code>json</code>, the value of the <code>Message</code> parameter must:</p>
    /// <ul>
    /// <li>
    /// <p>be a syntactically valid JSON object; and</p></li>
    /// <li>
    /// <p>contain at least a top-level JSON key of "default" with a value that is a string.</p></li>
    /// </ul>
    /// <p>You can define other top-level keys that define the message you want to send to a specific transport protocol (e.g., "http").</p>
    /// <p>Valid value: <code>json</code></p>
    pub fn message_structure(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message_structure = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Set <code>MessageStructure</code> to <code>json</code> if you want to send a different message for each protocol. For example, using one publish action, you can send a short message to your SMS subscribers and a longer message to your email subscribers. If you set <code>MessageStructure</code> to <code>json</code>, the value of the <code>Message</code> parameter must:</p>
    /// <ul>
    /// <li>
    /// <p>be a syntactically valid JSON object; and</p></li>
    /// <li>
    /// <p>contain at least a top-level JSON key of "default" with a value that is a string.</p></li>
    /// </ul>
    /// <p>You can define other top-level keys that define the message you want to send to a specific transport protocol (e.g., "http").</p>
    /// <p>Valid value: <code>json</code></p>
    pub fn set_message_structure(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message_structure = input;
        self
    }
    /// <p>Set <code>MessageStructure</code> to <code>json</code> if you want to send a different message for each protocol. For example, using one publish action, you can send a short message to your SMS subscribers and a longer message to your email subscribers. If you set <code>MessageStructure</code> to <code>json</code>, the value of the <code>Message</code> parameter must:</p>
    /// <ul>
    /// <li>
    /// <p>be a syntactically valid JSON object; and</p></li>
    /// <li>
    /// <p>contain at least a top-level JSON key of "default" with a value that is a string.</p></li>
    /// </ul>
    /// <p>You can define other top-level keys that define the message you want to send to a specific transport protocol (e.g., "http").</p>
    /// <p>Valid value: <code>json</code></p>
    pub fn get_message_structure(&self) -> &::std::option::Option<::std::string::String> {
        &self.message_structure
    }
    /// Adds a key-value pair to `message_attributes`.
    ///
    /// To override the contents of this collection use [`set_message_attributes`](Self::set_message_attributes).
    ///
    /// <p>Message attributes for Publish action.</p>
    pub fn message_attributes(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::MessageAttributeValue) -> Self {
        let mut hash_map = self.message_attributes.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.message_attributes = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Message attributes for Publish action.</p>
    pub fn set_message_attributes(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::MessageAttributeValue>>,
    ) -> Self {
        self.message_attributes = input;
        self
    }
    /// <p>Message attributes for Publish action.</p>
    pub fn get_message_attributes(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::MessageAttributeValue>> {
        &self.message_attributes
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics. The <code>MessageDeduplicationId</code> can contain up to 128 alphanumeric characters <code>(a-z, A-Z, 0-9)</code> and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~)</code>.</p>
    /// <p>Every message must have a unique <code>MessageDeduplicationId</code>, which is a token used for deduplication of sent messages. If a message with a particular <code>MessageDeduplicationId</code> is sent successfully, any message sent with the same <code>MessageDeduplicationId</code> during the 5-minute deduplication interval is treated as a duplicate.</p>
    /// <p>If the topic has <code>ContentBasedDeduplication</code> set, the system generates a <code>MessageDeduplicationId</code> based on the contents of the message. Your <code>MessageDeduplicationId</code> overrides the generated one.</p>
    pub fn message_deduplication_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message_deduplication_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics. The <code>MessageDeduplicationId</code> can contain up to 128 alphanumeric characters <code>(a-z, A-Z, 0-9)</code> and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~)</code>.</p>
    /// <p>Every message must have a unique <code>MessageDeduplicationId</code>, which is a token used for deduplication of sent messages. If a message with a particular <code>MessageDeduplicationId</code> is sent successfully, any message sent with the same <code>MessageDeduplicationId</code> during the 5-minute deduplication interval is treated as a duplicate.</p>
    /// <p>If the topic has <code>ContentBasedDeduplication</code> set, the system generates a <code>MessageDeduplicationId</code> based on the contents of the message. Your <code>MessageDeduplicationId</code> overrides the generated one.</p>
    pub fn set_message_deduplication_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message_deduplication_id = input;
        self
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics. The <code>MessageDeduplicationId</code> can contain up to 128 alphanumeric characters <code>(a-z, A-Z, 0-9)</code> and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~)</code>.</p>
    /// <p>Every message must have a unique <code>MessageDeduplicationId</code>, which is a token used for deduplication of sent messages. If a message with a particular <code>MessageDeduplicationId</code> is sent successfully, any message sent with the same <code>MessageDeduplicationId</code> during the 5-minute deduplication interval is treated as a duplicate.</p>
    /// <p>If the topic has <code>ContentBasedDeduplication</code> set, the system generates a <code>MessageDeduplicationId</code> based on the contents of the message. Your <code>MessageDeduplicationId</code> overrides the generated one.</p>
    pub fn get_message_deduplication_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.message_deduplication_id
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics. The <code>MessageGroupId</code> can contain up to 128 alphanumeric characters <code>(a-z, A-Z, 0-9)</code> and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~)</code>.</p>
    /// <p>The <code>MessageGroupId</code> is a tag that specifies that a message belongs to a specific message group. Messages that belong to the same message group are processed in a FIFO manner (however, messages in different message groups might be processed out of order). Every message must include a <code>MessageGroupId</code>.</p>
    pub fn message_group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message_group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics. The <code>MessageGroupId</code> can contain up to 128 alphanumeric characters <code>(a-z, A-Z, 0-9)</code> and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~)</code>.</p>
    /// <p>The <code>MessageGroupId</code> is a tag that specifies that a message belongs to a specific message group. Messages that belong to the same message group are processed in a FIFO manner (however, messages in different message groups might be processed out of order). Every message must include a <code>MessageGroupId</code>.</p>
    pub fn set_message_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message_group_id = input;
        self
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics. The <code>MessageGroupId</code> can contain up to 128 alphanumeric characters <code>(a-z, A-Z, 0-9)</code> and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~)</code>.</p>
    /// <p>The <code>MessageGroupId</code> is a tag that specifies that a message belongs to a specific message group. Messages that belong to the same message group are processed in a FIFO manner (however, messages in different message groups might be processed out of order). Every message must include a <code>MessageGroupId</code>.</p>
    pub fn get_message_group_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.message_group_id
    }
    /// Consumes the builder and constructs a [`PublishInput`](crate::operation::publish::PublishInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::publish::PublishInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::publish::PublishInput {
            topic_arn: self.topic_arn,
            target_arn: self.target_arn,
            phone_number: self.phone_number,
            message: self.message,
            subject: self.subject,
            message_structure: self.message_structure,
            message_attributes: self.message_attributes,
            message_deduplication_id: self.message_deduplication_id,
            message_group_id: self.message_group_id,
        })
    }
}
