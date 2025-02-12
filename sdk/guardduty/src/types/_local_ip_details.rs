// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about the local IP address of the connection.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LocalIpDetails {
    /// <p>The IPv4 local address of the connection.</p>
    pub ip_address_v4: ::std::option::Option<::std::string::String>,
}
impl LocalIpDetails {
    /// <p>The IPv4 local address of the connection.</p>
    pub fn ip_address_v4(&self) -> ::std::option::Option<&str> {
        self.ip_address_v4.as_deref()
    }
}
impl LocalIpDetails {
    /// Creates a new builder-style object to manufacture [`LocalIpDetails`](crate::types::LocalIpDetails).
    pub fn builder() -> crate::types::builders::LocalIpDetailsBuilder {
        crate::types::builders::LocalIpDetailsBuilder::default()
    }
}

/// A builder for [`LocalIpDetails`](crate::types::LocalIpDetails).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct LocalIpDetailsBuilder {
    pub(crate) ip_address_v4: ::std::option::Option<::std::string::String>,
}
impl LocalIpDetailsBuilder {
    /// <p>The IPv4 local address of the connection.</p>
    pub fn ip_address_v4(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ip_address_v4 = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IPv4 local address of the connection.</p>
    pub fn set_ip_address_v4(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ip_address_v4 = input;
        self
    }
    /// <p>The IPv4 local address of the connection.</p>
    pub fn get_ip_address_v4(&self) -> &::std::option::Option<::std::string::String> {
        &self.ip_address_v4
    }
    /// Consumes the builder and constructs a [`LocalIpDetails`](crate::types::LocalIpDetails).
    pub fn build(self) -> crate::types::LocalIpDetails {
        crate::types::LocalIpDetails {
            ip_address_v4: self.ip_address_v4,
        }
    }
}
