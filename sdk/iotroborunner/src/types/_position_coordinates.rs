// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Supported coordinates for worker position.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub enum PositionCoordinates {
    /// Cartesian coordinates.
    CartesianCoordinates(crate::types::CartesianCoordinates),
    /// The `Unknown` variant represents cases where new union variant was received. Consider upgrading the SDK to the latest available version.
    /// An unknown enum variant
    ///
    /// _Note: If you encounter this error, consider upgrading your SDK to the latest version._
    /// The `Unknown` variant represents cases where the server sent a value that wasn't recognized
    /// by the client. This can happen when the server adds new functionality, but the client has not been updated.
    /// To investigate this, consider turning on debug logging to print the raw HTTP response.
    #[non_exhaustive]
    Unknown,
}
impl PositionCoordinates {
    #[allow(irrefutable_let_patterns)]
    /// Tries to convert the enum instance into [`CartesianCoordinates`](crate::types::PositionCoordinates::CartesianCoordinates), extracting the inner [`CartesianCoordinates`](crate::types::CartesianCoordinates).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_cartesian_coordinates(&self) -> ::std::result::Result<&crate::types::CartesianCoordinates, &Self> {
        if let PositionCoordinates::CartesianCoordinates(val) = &self {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Returns true if this is a [`CartesianCoordinates`](crate::types::PositionCoordinates::CartesianCoordinates).
    pub fn is_cartesian_coordinates(&self) -> bool {
        self.as_cartesian_coordinates().is_ok()
    }
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}
