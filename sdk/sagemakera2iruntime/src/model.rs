// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Attributes of the data specified by the customer. Use these to describe the data to be labeled.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct HumanLoopDataAttributes {
    /// <p>Declares that your content is free of personally identifiable information or adult content.</p>
    /// <p>Amazon SageMaker can restrict the Amazon Mechanical Turk workers who can view your task based on this information.</p>
    #[doc(hidden)]
    pub content_classifiers: std::option::Option<std::vec::Vec<crate::model::ContentClassifier>>,
}
impl HumanLoopDataAttributes {
    /// <p>Declares that your content is free of personally identifiable information or adult content.</p>
    /// <p>Amazon SageMaker can restrict the Amazon Mechanical Turk workers who can view your task based on this information.</p>
    pub fn content_classifiers(&self) -> std::option::Option<&[crate::model::ContentClassifier]> {
        self.content_classifiers.as_deref()
    }
}
impl std::fmt::Debug for HumanLoopDataAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("HumanLoopDataAttributes");
        formatter.field("content_classifiers", &self.content_classifiers);
        formatter.finish()
    }
}
/// See [`HumanLoopDataAttributes`](crate::model::HumanLoopDataAttributes).
pub mod human_loop_data_attributes {

    /// A builder for [`HumanLoopDataAttributes`](crate::model::HumanLoopDataAttributes).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) content_classifiers:
            std::option::Option<std::vec::Vec<crate::model::ContentClassifier>>,
    }
    impl Builder {
        /// Appends an item to `content_classifiers`.
        ///
        /// To override the contents of this collection use [`set_content_classifiers`](Self::set_content_classifiers).
        ///
        /// <p>Declares that your content is free of personally identifiable information or adult content.</p>
        /// <p>Amazon SageMaker can restrict the Amazon Mechanical Turk workers who can view your task based on this information.</p>
        pub fn content_classifiers(mut self, input: crate::model::ContentClassifier) -> Self {
            let mut v = self.content_classifiers.unwrap_or_default();
            v.push(input);
            self.content_classifiers = Some(v);
            self
        }
        /// <p>Declares that your content is free of personally identifiable information or adult content.</p>
        /// <p>Amazon SageMaker can restrict the Amazon Mechanical Turk workers who can view your task based on this information.</p>
        pub fn set_content_classifiers(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ContentClassifier>>,
        ) -> Self {
            self.content_classifiers = input;
            self
        }
        /// Consumes the builder and constructs a [`HumanLoopDataAttributes`](crate::model::HumanLoopDataAttributes).
        pub fn build(self) -> crate::model::HumanLoopDataAttributes {
            crate::model::HumanLoopDataAttributes {
                content_classifiers: self.content_classifiers,
            }
        }
    }
}
impl HumanLoopDataAttributes {
    /// Creates a new builder-style object to manufacture [`HumanLoopDataAttributes`](crate::model::HumanLoopDataAttributes).
    pub fn builder() -> crate::model::human_loop_data_attributes::Builder {
        crate::model::human_loop_data_attributes::Builder::default()
    }
}

/// When writing a match expression against `ContentClassifier`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let contentclassifier = unimplemented!();
/// match contentclassifier {
///     ContentClassifier::FreeOfAdultContent => { /* ... */ },
///     ContentClassifier::FreeOfPersonallyIdentifiableInformation => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `contentclassifier` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `ContentClassifier::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `ContentClassifier::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `ContentClassifier::NewFeature` is defined.
/// Specifically, when `contentclassifier` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `ContentClassifier::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum ContentClassifier {
    #[allow(missing_docs)] // documentation missing in model
    FreeOfAdultContent,
    #[allow(missing_docs)] // documentation missing in model
    FreeOfPersonallyIdentifiableInformation,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue),
}
impl std::convert::From<&str> for ContentClassifier {
    fn from(s: &str) -> Self {
        match s {
            "FreeOfAdultContent" => ContentClassifier::FreeOfAdultContent,
            "FreeOfPersonallyIdentifiableInformation" => {
                ContentClassifier::FreeOfPersonallyIdentifiableInformation
            }
            other => {
                ContentClassifier::Unknown(crate::types::UnknownVariantValue(other.to_owned()))
            }
        }
    }
}
impl std::str::FromStr for ContentClassifier {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ContentClassifier::from(s))
    }
}
impl ContentClassifier {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ContentClassifier::FreeOfAdultContent => "FreeOfAdultContent",
            ContentClassifier::FreeOfPersonallyIdentifiableInformation => {
                "FreeOfPersonallyIdentifiableInformation"
            }
            ContentClassifier::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "FreeOfAdultContent",
            "FreeOfPersonallyIdentifiableInformation",
        ]
    }
}
impl AsRef<str> for ContentClassifier {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>An object containing the human loop input in JSON format.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct HumanLoopInput {
    /// <p>Serialized input from the human loop. The input must be a string representation of a file in JSON format.</p>
    #[doc(hidden)]
    pub input_content: std::option::Option<std::string::String>,
}
impl HumanLoopInput {
    /// <p>Serialized input from the human loop. The input must be a string representation of a file in JSON format.</p>
    pub fn input_content(&self) -> std::option::Option<&str> {
        self.input_content.as_deref()
    }
}
impl std::fmt::Debug for HumanLoopInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("HumanLoopInput");
        formatter.field("input_content", &self.input_content);
        formatter.finish()
    }
}
/// See [`HumanLoopInput`](crate::model::HumanLoopInput).
pub mod human_loop_input {

    /// A builder for [`HumanLoopInput`](crate::model::HumanLoopInput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) input_content: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>Serialized input from the human loop. The input must be a string representation of a file in JSON format.</p>
        pub fn input_content(mut self, input: impl Into<std::string::String>) -> Self {
            self.input_content = Some(input.into());
            self
        }
        /// <p>Serialized input from the human loop. The input must be a string representation of a file in JSON format.</p>
        pub fn set_input_content(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.input_content = input;
            self
        }
        /// Consumes the builder and constructs a [`HumanLoopInput`](crate::model::HumanLoopInput).
        pub fn build(self) -> crate::model::HumanLoopInput {
            crate::model::HumanLoopInput {
                input_content: self.input_content,
            }
        }
    }
}
impl HumanLoopInput {
    /// Creates a new builder-style object to manufacture [`HumanLoopInput`](crate::model::HumanLoopInput).
    pub fn builder() -> crate::model::human_loop_input::Builder {
        crate::model::human_loop_input::Builder::default()
    }
}

/// <p>Summary information about the human loop.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct HumanLoopSummary {
    /// <p>The name of the human loop.</p>
    #[doc(hidden)]
    pub human_loop_name: std::option::Option<std::string::String>,
    /// <p>The status of the human loop. </p>
    #[doc(hidden)]
    pub human_loop_status: std::option::Option<crate::model::HumanLoopStatus>,
    /// <p>When Amazon Augmented AI created the human loop.</p>
    #[doc(hidden)]
    pub creation_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The reason why the human loop failed. A failure reason is returned when the status of the human loop is <code>Failed</code>.</p>
    #[doc(hidden)]
    pub failure_reason: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the flow definition used to configure the human loop.</p>
    #[doc(hidden)]
    pub flow_definition_arn: std::option::Option<std::string::String>,
}
impl HumanLoopSummary {
    /// <p>The name of the human loop.</p>
    pub fn human_loop_name(&self) -> std::option::Option<&str> {
        self.human_loop_name.as_deref()
    }
    /// <p>The status of the human loop. </p>
    pub fn human_loop_status(&self) -> std::option::Option<&crate::model::HumanLoopStatus> {
        self.human_loop_status.as_ref()
    }
    /// <p>When Amazon Augmented AI created the human loop.</p>
    pub fn creation_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p>The reason why the human loop failed. A failure reason is returned when the status of the human loop is <code>Failed</code>.</p>
    pub fn failure_reason(&self) -> std::option::Option<&str> {
        self.failure_reason.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the flow definition used to configure the human loop.</p>
    pub fn flow_definition_arn(&self) -> std::option::Option<&str> {
        self.flow_definition_arn.as_deref()
    }
}
impl std::fmt::Debug for HumanLoopSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("HumanLoopSummary");
        formatter.field("human_loop_name", &self.human_loop_name);
        formatter.field("human_loop_status", &self.human_loop_status);
        formatter.field("creation_time", &self.creation_time);
        formatter.field("failure_reason", &self.failure_reason);
        formatter.field("flow_definition_arn", &self.flow_definition_arn);
        formatter.finish()
    }
}
/// See [`HumanLoopSummary`](crate::model::HumanLoopSummary).
pub mod human_loop_summary {

    /// A builder for [`HumanLoopSummary`](crate::model::HumanLoopSummary).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) human_loop_name: std::option::Option<std::string::String>,
        pub(crate) human_loop_status: std::option::Option<crate::model::HumanLoopStatus>,
        pub(crate) creation_time: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) failure_reason: std::option::Option<std::string::String>,
        pub(crate) flow_definition_arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The name of the human loop.</p>
        pub fn human_loop_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.human_loop_name = Some(input.into());
            self
        }
        /// <p>The name of the human loop.</p>
        pub fn set_human_loop_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.human_loop_name = input;
            self
        }
        /// <p>The status of the human loop. </p>
        pub fn human_loop_status(mut self, input: crate::model::HumanLoopStatus) -> Self {
            self.human_loop_status = Some(input);
            self
        }
        /// <p>The status of the human loop. </p>
        pub fn set_human_loop_status(
            mut self,
            input: std::option::Option<crate::model::HumanLoopStatus>,
        ) -> Self {
            self.human_loop_status = input;
            self
        }
        /// <p>When Amazon Augmented AI created the human loop.</p>
        pub fn creation_time(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.creation_time = Some(input);
            self
        }
        /// <p>When Amazon Augmented AI created the human loop.</p>
        pub fn set_creation_time(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.creation_time = input;
            self
        }
        /// <p>The reason why the human loop failed. A failure reason is returned when the status of the human loop is <code>Failed</code>.</p>
        pub fn failure_reason(mut self, input: impl Into<std::string::String>) -> Self {
            self.failure_reason = Some(input.into());
            self
        }
        /// <p>The reason why the human loop failed. A failure reason is returned when the status of the human loop is <code>Failed</code>.</p>
        pub fn set_failure_reason(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.failure_reason = input;
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the flow definition used to configure the human loop.</p>
        pub fn flow_definition_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.flow_definition_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the flow definition used to configure the human loop.</p>
        pub fn set_flow_definition_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.flow_definition_arn = input;
            self
        }
        /// Consumes the builder and constructs a [`HumanLoopSummary`](crate::model::HumanLoopSummary).
        pub fn build(self) -> crate::model::HumanLoopSummary {
            crate::model::HumanLoopSummary {
                human_loop_name: self.human_loop_name,
                human_loop_status: self.human_loop_status,
                creation_time: self.creation_time,
                failure_reason: self.failure_reason,
                flow_definition_arn: self.flow_definition_arn,
            }
        }
    }
}
impl HumanLoopSummary {
    /// Creates a new builder-style object to manufacture [`HumanLoopSummary`](crate::model::HumanLoopSummary).
    pub fn builder() -> crate::model::human_loop_summary::Builder {
        crate::model::human_loop_summary::Builder::default()
    }
}

/// When writing a match expression against `HumanLoopStatus`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let humanloopstatus = unimplemented!();
/// match humanloopstatus {
///     HumanLoopStatus::Completed => { /* ... */ },
///     HumanLoopStatus::Failed => { /* ... */ },
///     HumanLoopStatus::InProgress => { /* ... */ },
///     HumanLoopStatus::Stopped => { /* ... */ },
///     HumanLoopStatus::Stopping => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `humanloopstatus` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `HumanLoopStatus::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `HumanLoopStatus::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `HumanLoopStatus::NewFeature` is defined.
/// Specifically, when `humanloopstatus` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `HumanLoopStatus::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum HumanLoopStatus {
    #[allow(missing_docs)] // documentation missing in model
    Completed,
    #[allow(missing_docs)] // documentation missing in model
    Failed,
    #[allow(missing_docs)] // documentation missing in model
    InProgress,
    #[allow(missing_docs)] // documentation missing in model
    Stopped,
    #[allow(missing_docs)] // documentation missing in model
    Stopping,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue),
}
impl std::convert::From<&str> for HumanLoopStatus {
    fn from(s: &str) -> Self {
        match s {
            "Completed" => HumanLoopStatus::Completed,
            "Failed" => HumanLoopStatus::Failed,
            "InProgress" => HumanLoopStatus::InProgress,
            "Stopped" => HumanLoopStatus::Stopped,
            "Stopping" => HumanLoopStatus::Stopping,
            other => HumanLoopStatus::Unknown(crate::types::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl std::str::FromStr for HumanLoopStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(HumanLoopStatus::from(s))
    }
}
impl HumanLoopStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            HumanLoopStatus::Completed => "Completed",
            HumanLoopStatus::Failed => "Failed",
            HumanLoopStatus::InProgress => "InProgress",
            HumanLoopStatus::Stopped => "Stopped",
            HumanLoopStatus::Stopping => "Stopping",
            HumanLoopStatus::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["Completed", "Failed", "InProgress", "Stopped", "Stopping"]
    }
}
impl AsRef<str> for HumanLoopStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// When writing a match expression against `SortOrder`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let sortorder = unimplemented!();
/// match sortorder {
///     SortOrder::Ascending => { /* ... */ },
///     SortOrder::Descending => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `sortorder` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `SortOrder::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `SortOrder::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `SortOrder::NewFeature` is defined.
/// Specifically, when `sortorder` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `SortOrder::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum SortOrder {
    #[allow(missing_docs)] // documentation missing in model
    Ascending,
    #[allow(missing_docs)] // documentation missing in model
    Descending,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue),
}
impl std::convert::From<&str> for SortOrder {
    fn from(s: &str) -> Self {
        match s {
            "Ascending" => SortOrder::Ascending,
            "Descending" => SortOrder::Descending,
            other => SortOrder::Unknown(crate::types::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl std::str::FromStr for SortOrder {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(SortOrder::from(s))
    }
}
impl SortOrder {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            SortOrder::Ascending => "Ascending",
            SortOrder::Descending => "Descending",
            SortOrder::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["Ascending", "Descending"]
    }
}
impl AsRef<str> for SortOrder {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Information about where the human output will be stored.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct HumanLoopOutput {
    /// <p>The location of the Amazon S3 object where Amazon Augmented AI stores your human loop output.</p>
    #[doc(hidden)]
    pub output_s3_uri: std::option::Option<std::string::String>,
}
impl HumanLoopOutput {
    /// <p>The location of the Amazon S3 object where Amazon Augmented AI stores your human loop output.</p>
    pub fn output_s3_uri(&self) -> std::option::Option<&str> {
        self.output_s3_uri.as_deref()
    }
}
impl std::fmt::Debug for HumanLoopOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("HumanLoopOutput");
        formatter.field("output_s3_uri", &self.output_s3_uri);
        formatter.finish()
    }
}
/// See [`HumanLoopOutput`](crate::model::HumanLoopOutput).
pub mod human_loop_output {

    /// A builder for [`HumanLoopOutput`](crate::model::HumanLoopOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) output_s3_uri: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The location of the Amazon S3 object where Amazon Augmented AI stores your human loop output.</p>
        pub fn output_s3_uri(mut self, input: impl Into<std::string::String>) -> Self {
            self.output_s3_uri = Some(input.into());
            self
        }
        /// <p>The location of the Amazon S3 object where Amazon Augmented AI stores your human loop output.</p>
        pub fn set_output_s3_uri(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.output_s3_uri = input;
            self
        }
        /// Consumes the builder and constructs a [`HumanLoopOutput`](crate::model::HumanLoopOutput).
        pub fn build(self) -> crate::model::HumanLoopOutput {
            crate::model::HumanLoopOutput {
                output_s3_uri: self.output_s3_uri,
            }
        }
    }
}
impl HumanLoopOutput {
    /// Creates a new builder-style object to manufacture [`HumanLoopOutput`](crate::model::HumanLoopOutput).
    pub fn builder() -> crate::model::human_loop_output::Builder {
        crate::model::human_loop_output::Builder::default()
    }
}
