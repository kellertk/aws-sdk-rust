// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateKeyValueStore`](crate::operation::update_key_value_store::builders::UpdateKeyValueStoreFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::update_key_value_store::builders::UpdateKeyValueStoreFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_key_value_store::builders::UpdateKeyValueStoreFluentBuilder::set_name):<br>required: **true**<br><p>The name of the Key Value Store to update.</p><br>
    ///   - [`comment(impl Into<String>)`](crate::operation::update_key_value_store::builders::UpdateKeyValueStoreFluentBuilder::comment) / [`set_comment(Option<String>)`](crate::operation::update_key_value_store::builders::UpdateKeyValueStoreFluentBuilder::set_comment):<br>required: **true**<br><p>The comment of the Key Value Store to update.</p><br>
    ///   - [`if_match(impl Into<String>)`](crate::operation::update_key_value_store::builders::UpdateKeyValueStoreFluentBuilder::if_match) / [`set_if_match(Option<String>)`](crate::operation::update_key_value_store::builders::UpdateKeyValueStoreFluentBuilder::set_if_match):<br>required: **true**<br><p>The Key Value Store to update, if a match occurs.</p><br>
    /// - On success, responds with [`UpdateKeyValueStoreOutput`](crate::operation::update_key_value_store::UpdateKeyValueStoreOutput) with field(s):
    ///   - [`key_value_store(Option<KeyValueStore>)`](crate::operation::update_key_value_store::UpdateKeyValueStoreOutput::key_value_store): <p>The resulting Key Value Store to update.</p>
    ///   - [`e_tag(Option<String>)`](crate::operation::update_key_value_store::UpdateKeyValueStoreOutput::e_tag): <p>The ETag of the resulting Key Value Store.</p>
    /// - On failure, responds with [`SdkError<UpdateKeyValueStoreError>`](crate::operation::update_key_value_store::UpdateKeyValueStoreError)
    pub fn update_key_value_store(&self) -> crate::operation::update_key_value_store::builders::UpdateKeyValueStoreFluentBuilder {
        crate::operation::update_key_value_store::builders::UpdateKeyValueStoreFluentBuilder::new(self.handle.clone())
    }
}
