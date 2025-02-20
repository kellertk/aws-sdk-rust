// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Create Action Group Response
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateAgentActionGroupOutput {
    /// Contains the information of an Agent Action Group
    pub agent_action_group: ::std::option::Option<crate::types::AgentActionGroup>,
    _request_id: Option<String>,
}
impl CreateAgentActionGroupOutput {
    /// Contains the information of an Agent Action Group
    pub fn agent_action_group(&self) -> ::std::option::Option<&crate::types::AgentActionGroup> {
        self.agent_action_group.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for CreateAgentActionGroupOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateAgentActionGroupOutput {
    /// Creates a new builder-style object to manufacture [`CreateAgentActionGroupOutput`](crate::operation::create_agent_action_group::CreateAgentActionGroupOutput).
    pub fn builder() -> crate::operation::create_agent_action_group::builders::CreateAgentActionGroupOutputBuilder {
        crate::operation::create_agent_action_group::builders::CreateAgentActionGroupOutputBuilder::default()
    }
}

/// A builder for [`CreateAgentActionGroupOutput`](crate::operation::create_agent_action_group::CreateAgentActionGroupOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateAgentActionGroupOutputBuilder {
    pub(crate) agent_action_group: ::std::option::Option<crate::types::AgentActionGroup>,
    _request_id: Option<String>,
}
impl CreateAgentActionGroupOutputBuilder {
    /// Contains the information of an Agent Action Group
    /// This field is required.
    pub fn agent_action_group(mut self, input: crate::types::AgentActionGroup) -> Self {
        self.agent_action_group = ::std::option::Option::Some(input);
        self
    }
    /// Contains the information of an Agent Action Group
    pub fn set_agent_action_group(mut self, input: ::std::option::Option<crate::types::AgentActionGroup>) -> Self {
        self.agent_action_group = input;
        self
    }
    /// Contains the information of an Agent Action Group
    pub fn get_agent_action_group(&self) -> &::std::option::Option<crate::types::AgentActionGroup> {
        &self.agent_action_group
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateAgentActionGroupOutput`](crate::operation::create_agent_action_group::CreateAgentActionGroupOutput).
    pub fn build(self) -> crate::operation::create_agent_action_group::CreateAgentActionGroupOutput {
        crate::operation::create_agent_action_group::CreateAgentActionGroupOutput {
            agent_action_group: self.agent_action_group,
            _request_id: self._request_id,
        }
    }
}
