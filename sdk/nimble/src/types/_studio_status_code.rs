// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `StudioStatusCode`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let studiostatuscode = unimplemented!();
/// match studiostatuscode {
///     StudioStatusCode::AwsSsoAccessDenied => { /* ... */ },
///     StudioStatusCode::AwsSsoConfigurationRepaired => { /* ... */ },
///     StudioStatusCode::AwsSsoConfigurationRepairInProgress => { /* ... */ },
///     StudioStatusCode::AwsSsoNotEnabled => { /* ... */ },
///     StudioStatusCode::AwsStsRegionDisabled => { /* ... */ },
///     StudioStatusCode::EncryptionKeyAccessDenied => { /* ... */ },
///     StudioStatusCode::EncryptionKeyNotFound => { /* ... */ },
///     StudioStatusCode::InternalError => { /* ... */ },
///     StudioStatusCode::RoleCouldNotBeAssumed => { /* ... */ },
///     StudioStatusCode::RoleNotOwnedByStudioOwner => { /* ... */ },
///     StudioStatusCode::StudioCreated => { /* ... */ },
///     StudioStatusCode::StudioCreateInProgress => { /* ... */ },
///     StudioStatusCode::StudioDeleted => { /* ... */ },
///     StudioStatusCode::StudioDeleteInProgress => { /* ... */ },
///     StudioStatusCode::StudioUpdated => { /* ... */ },
///     StudioStatusCode::StudioUpdateInProgress => { /* ... */ },
///     StudioStatusCode::StudioWithLaunchProfilesNotDeleted => { /* ... */ },
///     StudioStatusCode::StudioWithStreamingImagesNotDeleted => { /* ... */ },
///     StudioStatusCode::StudioWithStudioComponentsNotDeleted => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `studiostatuscode` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `StudioStatusCode::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `StudioStatusCode::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `StudioStatusCode::NewFeature` is defined.
/// Specifically, when `studiostatuscode` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `StudioStatusCode::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
/// <p>The status code.</p>
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
)]
pub enum StudioStatusCode {
    #[allow(missing_docs)] // documentation missing in model
    AwsSsoAccessDenied,
    #[allow(missing_docs)] // documentation missing in model
    AwsSsoConfigurationRepaired,
    #[allow(missing_docs)] // documentation missing in model
    AwsSsoConfigurationRepairInProgress,
    #[allow(missing_docs)] // documentation missing in model
    AwsSsoNotEnabled,
    #[allow(missing_docs)] // documentation missing in model
    AwsStsRegionDisabled,
    #[allow(missing_docs)] // documentation missing in model
    EncryptionKeyAccessDenied,
    #[allow(missing_docs)] // documentation missing in model
    EncryptionKeyNotFound,
    #[allow(missing_docs)] // documentation missing in model
    InternalError,
    #[allow(missing_docs)] // documentation missing in model
    RoleCouldNotBeAssumed,
    #[allow(missing_docs)] // documentation missing in model
    RoleNotOwnedByStudioOwner,
    #[allow(missing_docs)] // documentation missing in model
    StudioCreated,
    #[allow(missing_docs)] // documentation missing in model
    StudioCreateInProgress,
    #[allow(missing_docs)] // documentation missing in model
    StudioDeleted,
    #[allow(missing_docs)] // documentation missing in model
    StudioDeleteInProgress,
    #[allow(missing_docs)] // documentation missing in model
    StudioUpdated,
    #[allow(missing_docs)] // documentation missing in model
    StudioUpdateInProgress,
    #[allow(missing_docs)] // documentation missing in model
    StudioWithLaunchProfilesNotDeleted,
    #[allow(missing_docs)] // documentation missing in model
    StudioWithStreamingImagesNotDeleted,
    #[allow(missing_docs)] // documentation missing in model
    StudioWithStudioComponentsNotDeleted,
    /// `Unknown` contains new variants that have been added since this code was generated.
    #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
    Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue),
}
impl ::std::convert::From<&str> for StudioStatusCode {
    fn from(s: &str) -> Self {
        match s {
            "AWS_SSO_ACCESS_DENIED" => StudioStatusCode::AwsSsoAccessDenied,
            "AWS_SSO_CONFIGURATION_REPAIRED" => StudioStatusCode::AwsSsoConfigurationRepaired,
            "AWS_SSO_CONFIGURATION_REPAIR_IN_PROGRESS" => StudioStatusCode::AwsSsoConfigurationRepairInProgress,
            "AWS_SSO_NOT_ENABLED" => StudioStatusCode::AwsSsoNotEnabled,
            "AWS_STS_REGION_DISABLED" => StudioStatusCode::AwsStsRegionDisabled,
            "ENCRYPTION_KEY_ACCESS_DENIED" => StudioStatusCode::EncryptionKeyAccessDenied,
            "ENCRYPTION_KEY_NOT_FOUND" => StudioStatusCode::EncryptionKeyNotFound,
            "INTERNAL_ERROR" => StudioStatusCode::InternalError,
            "ROLE_COULD_NOT_BE_ASSUMED" => StudioStatusCode::RoleCouldNotBeAssumed,
            "ROLE_NOT_OWNED_BY_STUDIO_OWNER" => StudioStatusCode::RoleNotOwnedByStudioOwner,
            "STUDIO_CREATED" => StudioStatusCode::StudioCreated,
            "STUDIO_CREATE_IN_PROGRESS" => StudioStatusCode::StudioCreateInProgress,
            "STUDIO_DELETED" => StudioStatusCode::StudioDeleted,
            "STUDIO_DELETE_IN_PROGRESS" => StudioStatusCode::StudioDeleteInProgress,
            "STUDIO_UPDATED" => StudioStatusCode::StudioUpdated,
            "STUDIO_UPDATE_IN_PROGRESS" => StudioStatusCode::StudioUpdateInProgress,
            "STUDIO_WITH_LAUNCH_PROFILES_NOT_DELETED" => StudioStatusCode::StudioWithLaunchProfilesNotDeleted,
            "STUDIO_WITH_STREAMING_IMAGES_NOT_DELETED" => StudioStatusCode::StudioWithStreamingImagesNotDeleted,
            "STUDIO_WITH_STUDIO_COMPONENTS_NOT_DELETED" => StudioStatusCode::StudioWithStudioComponentsNotDeleted,
            other => StudioStatusCode::Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl ::std::str::FromStr for StudioStatusCode {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(StudioStatusCode::from(s))
    }
}
impl StudioStatusCode {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            StudioStatusCode::AwsSsoAccessDenied => "AWS_SSO_ACCESS_DENIED",
            StudioStatusCode::AwsSsoConfigurationRepaired => "AWS_SSO_CONFIGURATION_REPAIRED",
            StudioStatusCode::AwsSsoConfigurationRepairInProgress => "AWS_SSO_CONFIGURATION_REPAIR_IN_PROGRESS",
            StudioStatusCode::AwsSsoNotEnabled => "AWS_SSO_NOT_ENABLED",
            StudioStatusCode::AwsStsRegionDisabled => "AWS_STS_REGION_DISABLED",
            StudioStatusCode::EncryptionKeyAccessDenied => "ENCRYPTION_KEY_ACCESS_DENIED",
            StudioStatusCode::EncryptionKeyNotFound => "ENCRYPTION_KEY_NOT_FOUND",
            StudioStatusCode::InternalError => "INTERNAL_ERROR",
            StudioStatusCode::RoleCouldNotBeAssumed => "ROLE_COULD_NOT_BE_ASSUMED",
            StudioStatusCode::RoleNotOwnedByStudioOwner => "ROLE_NOT_OWNED_BY_STUDIO_OWNER",
            StudioStatusCode::StudioCreated => "STUDIO_CREATED",
            StudioStatusCode::StudioCreateInProgress => "STUDIO_CREATE_IN_PROGRESS",
            StudioStatusCode::StudioDeleted => "STUDIO_DELETED",
            StudioStatusCode::StudioDeleteInProgress => "STUDIO_DELETE_IN_PROGRESS",
            StudioStatusCode::StudioUpdated => "STUDIO_UPDATED",
            StudioStatusCode::StudioUpdateInProgress => "STUDIO_UPDATE_IN_PROGRESS",
            StudioStatusCode::StudioWithLaunchProfilesNotDeleted => "STUDIO_WITH_LAUNCH_PROFILES_NOT_DELETED",
            StudioStatusCode::StudioWithStreamingImagesNotDeleted => "STUDIO_WITH_STREAMING_IMAGES_NOT_DELETED",
            StudioStatusCode::StudioWithStudioComponentsNotDeleted => "STUDIO_WITH_STUDIO_COMPONENTS_NOT_DELETED",
            StudioStatusCode::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "AWS_SSO_ACCESS_DENIED",
            "AWS_SSO_CONFIGURATION_REPAIRED",
            "AWS_SSO_CONFIGURATION_REPAIR_IN_PROGRESS",
            "AWS_SSO_NOT_ENABLED",
            "AWS_STS_REGION_DISABLED",
            "ENCRYPTION_KEY_ACCESS_DENIED",
            "ENCRYPTION_KEY_NOT_FOUND",
            "INTERNAL_ERROR",
            "ROLE_COULD_NOT_BE_ASSUMED",
            "ROLE_NOT_OWNED_BY_STUDIO_OWNER",
            "STUDIO_CREATED",
            "STUDIO_CREATE_IN_PROGRESS",
            "STUDIO_DELETED",
            "STUDIO_DELETE_IN_PROGRESS",
            "STUDIO_UPDATED",
            "STUDIO_UPDATE_IN_PROGRESS",
            "STUDIO_WITH_LAUNCH_PROFILES_NOT_DELETED",
            "STUDIO_WITH_STREAMING_IMAGES_NOT_DELETED",
            "STUDIO_WITH_STUDIO_COMPONENTS_NOT_DELETED",
        ]
    }
}
impl ::std::convert::AsRef<str> for StudioStatusCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl StudioStatusCode {
    /// Parses the enum value while disallowing unknown variants.
    ///
    /// Unknown variants will result in an error.
    pub fn try_parse(value: &str) -> ::std::result::Result<Self, crate::error::UnknownVariantError> {
        match Self::from(value) {
            #[allow(deprecated)]
            Self::Unknown(_) => ::std::result::Result::Err(crate::error::UnknownVariantError::new(value)),
            known => Ok(known),
        }
    }
}
