// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateCustomPlugin`](crate::operation::create_custom_plugin::builders::CreateCustomPluginFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`content_type(CustomPluginContentType)`](crate::operation::create_custom_plugin::builders::CreateCustomPluginFluentBuilder::content_type) / [`set_content_type(Option<CustomPluginContentType>)`](crate::operation::create_custom_plugin::builders::CreateCustomPluginFluentBuilder::set_content_type):<br>required: **true**<br><p>The type of the plugin file.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_custom_plugin::builders::CreateCustomPluginFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_custom_plugin::builders::CreateCustomPluginFluentBuilder::set_description):<br>required: **false**<br><p>A summary description of the custom plugin.</p><br>
    ///   - [`location(CustomPluginLocation)`](crate::operation::create_custom_plugin::builders::CreateCustomPluginFluentBuilder::location) / [`set_location(Option<CustomPluginLocation>)`](crate::operation::create_custom_plugin::builders::CreateCustomPluginFluentBuilder::set_location):<br>required: **true**<br><p>Information about the location of a custom plugin.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::create_custom_plugin::builders::CreateCustomPluginFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_custom_plugin::builders::CreateCustomPluginFluentBuilder::set_name):<br>required: **true**<br><p>The name of the custom plugin.</p><br>
    /// - On success, responds with [`CreateCustomPluginOutput`](crate::operation::create_custom_plugin::CreateCustomPluginOutput) with field(s):
    ///   - [`custom_plugin_arn(Option<String>)`](crate::operation::create_custom_plugin::CreateCustomPluginOutput::custom_plugin_arn): <p>The Amazon Resource Name (ARN) that Amazon assigned to the custom plugin.</p>
    ///   - [`custom_plugin_state(Option<CustomPluginState>)`](crate::operation::create_custom_plugin::CreateCustomPluginOutput::custom_plugin_state): <p>The state of the custom plugin.</p>
    ///   - [`name(Option<String>)`](crate::operation::create_custom_plugin::CreateCustomPluginOutput::name): <p>The name of the custom plugin.</p>
    ///   - [`revision(i64)`](crate::operation::create_custom_plugin::CreateCustomPluginOutput::revision): <p>The revision of the custom plugin.</p>
    /// - On failure, responds with [`SdkError<CreateCustomPluginError>`](crate::operation::create_custom_plugin::CreateCustomPluginError)
    pub fn create_custom_plugin(&self) -> crate::operation::create_custom_plugin::builders::CreateCustomPluginFluentBuilder {
        crate::operation::create_custom_plugin::builders::CreateCustomPluginFluentBuilder::new(self.handle.clone())
    }
}
