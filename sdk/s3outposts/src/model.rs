// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Amazon S3 on Outposts Access Points simplify managing data access at scale for shared datasets in S3 on Outposts. S3 on Outposts uses endpoints to connect to Outposts buckets so that you can perform actions within your virtual private cloud (VPC). For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/WorkingWithS3Outposts.html"> Accessing S3 on Outposts using VPC-only access points</a> in the <i>Amazon Simple Storage Service User Guide</i>.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Endpoint {
    /// <p>The Amazon Resource Name (ARN) of the endpoint.</p>
    #[doc(hidden)]
    pub endpoint_arn: std::option::Option<std::string::String>,
    /// <p>The ID of the Outposts.</p>
    #[doc(hidden)]
    pub outposts_id: std::option::Option<std::string::String>,
    /// <p>The VPC CIDR committed by this endpoint.</p>
    #[doc(hidden)]
    pub cidr_block: std::option::Option<std::string::String>,
    /// <p>The status of the endpoint.</p>
    #[doc(hidden)]
    pub status: std::option::Option<crate::model::EndpointStatus>,
    /// <p>The time the endpoint was created.</p>
    #[doc(hidden)]
    pub creation_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The network interface of the endpoint.</p>
    #[doc(hidden)]
    pub network_interfaces: std::option::Option<std::vec::Vec<crate::model::NetworkInterface>>,
    /// <p>The ID of the VPC used for the endpoint.</p>
    #[doc(hidden)]
    pub vpc_id: std::option::Option<std::string::String>,
    /// <p>The ID of the subnet used for the endpoint.</p>
    #[doc(hidden)]
    pub subnet_id: std::option::Option<std::string::String>,
    /// <p>The ID of the security group used for the endpoint.</p>
    #[doc(hidden)]
    pub security_group_id: std::option::Option<std::string::String>,
    /// <p>The type of connectivity used to access the Amazon S3 on Outposts endpoint.</p>
    #[doc(hidden)]
    pub access_type: std::option::Option<crate::model::EndpointAccessType>,
    /// <p>The ID of the customer-owned IPv4 address pool used for the endpoint.</p>
    #[doc(hidden)]
    pub customer_owned_ipv4_pool: std::option::Option<std::string::String>,
}
impl Endpoint {
    /// <p>The Amazon Resource Name (ARN) of the endpoint.</p>
    pub fn endpoint_arn(&self) -> std::option::Option<&str> {
        self.endpoint_arn.as_deref()
    }
    /// <p>The ID of the Outposts.</p>
    pub fn outposts_id(&self) -> std::option::Option<&str> {
        self.outposts_id.as_deref()
    }
    /// <p>The VPC CIDR committed by this endpoint.</p>
    pub fn cidr_block(&self) -> std::option::Option<&str> {
        self.cidr_block.as_deref()
    }
    /// <p>The status of the endpoint.</p>
    pub fn status(&self) -> std::option::Option<&crate::model::EndpointStatus> {
        self.status.as_ref()
    }
    /// <p>The time the endpoint was created.</p>
    pub fn creation_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p>The network interface of the endpoint.</p>
    pub fn network_interfaces(&self) -> std::option::Option<&[crate::model::NetworkInterface]> {
        self.network_interfaces.as_deref()
    }
    /// <p>The ID of the VPC used for the endpoint.</p>
    pub fn vpc_id(&self) -> std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
    /// <p>The ID of the subnet used for the endpoint.</p>
    pub fn subnet_id(&self) -> std::option::Option<&str> {
        self.subnet_id.as_deref()
    }
    /// <p>The ID of the security group used for the endpoint.</p>
    pub fn security_group_id(&self) -> std::option::Option<&str> {
        self.security_group_id.as_deref()
    }
    /// <p>The type of connectivity used to access the Amazon S3 on Outposts endpoint.</p>
    pub fn access_type(&self) -> std::option::Option<&crate::model::EndpointAccessType> {
        self.access_type.as_ref()
    }
    /// <p>The ID of the customer-owned IPv4 address pool used for the endpoint.</p>
    pub fn customer_owned_ipv4_pool(&self) -> std::option::Option<&str> {
        self.customer_owned_ipv4_pool.as_deref()
    }
}
impl std::fmt::Debug for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Endpoint");
        formatter.field("endpoint_arn", &self.endpoint_arn);
        formatter.field("outposts_id", &self.outposts_id);
        formatter.field("cidr_block", &self.cidr_block);
        formatter.field("status", &self.status);
        formatter.field("creation_time", &self.creation_time);
        formatter.field("network_interfaces", &self.network_interfaces);
        formatter.field("vpc_id", &self.vpc_id);
        formatter.field("subnet_id", &self.subnet_id);
        formatter.field("security_group_id", &self.security_group_id);
        formatter.field("access_type", &self.access_type);
        formatter.field("customer_owned_ipv4_pool", &self.customer_owned_ipv4_pool);
        formatter.finish()
    }
}
/// See [`Endpoint`](crate::model::Endpoint).
pub mod endpoint {

    /// A builder for [`Endpoint`](crate::model::Endpoint).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) endpoint_arn: std::option::Option<std::string::String>,
        pub(crate) outposts_id: std::option::Option<std::string::String>,
        pub(crate) cidr_block: std::option::Option<std::string::String>,
        pub(crate) status: std::option::Option<crate::model::EndpointStatus>,
        pub(crate) creation_time: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) network_interfaces:
            std::option::Option<std::vec::Vec<crate::model::NetworkInterface>>,
        pub(crate) vpc_id: std::option::Option<std::string::String>,
        pub(crate) subnet_id: std::option::Option<std::string::String>,
        pub(crate) security_group_id: std::option::Option<std::string::String>,
        pub(crate) access_type: std::option::Option<crate::model::EndpointAccessType>,
        pub(crate) customer_owned_ipv4_pool: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) of the endpoint.</p>
        pub fn endpoint_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.endpoint_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the endpoint.</p>
        pub fn set_endpoint_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.endpoint_arn = input;
            self
        }
        /// <p>The ID of the Outposts.</p>
        pub fn outposts_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.outposts_id = Some(input.into());
            self
        }
        /// <p>The ID of the Outposts.</p>
        pub fn set_outposts_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.outposts_id = input;
            self
        }
        /// <p>The VPC CIDR committed by this endpoint.</p>
        pub fn cidr_block(mut self, input: impl Into<std::string::String>) -> Self {
            self.cidr_block = Some(input.into());
            self
        }
        /// <p>The VPC CIDR committed by this endpoint.</p>
        pub fn set_cidr_block(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.cidr_block = input;
            self
        }
        /// <p>The status of the endpoint.</p>
        pub fn status(mut self, input: crate::model::EndpointStatus) -> Self {
            self.status = Some(input);
            self
        }
        /// <p>The status of the endpoint.</p>
        pub fn set_status(
            mut self,
            input: std::option::Option<crate::model::EndpointStatus>,
        ) -> Self {
            self.status = input;
            self
        }
        /// <p>The time the endpoint was created.</p>
        pub fn creation_time(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.creation_time = Some(input);
            self
        }
        /// <p>The time the endpoint was created.</p>
        pub fn set_creation_time(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.creation_time = input;
            self
        }
        /// Appends an item to `network_interfaces`.
        ///
        /// To override the contents of this collection use [`set_network_interfaces`](Self::set_network_interfaces).
        ///
        /// <p>The network interface of the endpoint.</p>
        pub fn network_interfaces(mut self, input: crate::model::NetworkInterface) -> Self {
            let mut v = self.network_interfaces.unwrap_or_default();
            v.push(input);
            self.network_interfaces = Some(v);
            self
        }
        /// <p>The network interface of the endpoint.</p>
        pub fn set_network_interfaces(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::NetworkInterface>>,
        ) -> Self {
            self.network_interfaces = input;
            self
        }
        /// <p>The ID of the VPC used for the endpoint.</p>
        pub fn vpc_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.vpc_id = Some(input.into());
            self
        }
        /// <p>The ID of the VPC used for the endpoint.</p>
        pub fn set_vpc_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.vpc_id = input;
            self
        }
        /// <p>The ID of the subnet used for the endpoint.</p>
        pub fn subnet_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.subnet_id = Some(input.into());
            self
        }
        /// <p>The ID of the subnet used for the endpoint.</p>
        pub fn set_subnet_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.subnet_id = input;
            self
        }
        /// <p>The ID of the security group used for the endpoint.</p>
        pub fn security_group_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.security_group_id = Some(input.into());
            self
        }
        /// <p>The ID of the security group used for the endpoint.</p>
        pub fn set_security_group_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.security_group_id = input;
            self
        }
        /// <p>The type of connectivity used to access the Amazon S3 on Outposts endpoint.</p>
        pub fn access_type(mut self, input: crate::model::EndpointAccessType) -> Self {
            self.access_type = Some(input);
            self
        }
        /// <p>The type of connectivity used to access the Amazon S3 on Outposts endpoint.</p>
        pub fn set_access_type(
            mut self,
            input: std::option::Option<crate::model::EndpointAccessType>,
        ) -> Self {
            self.access_type = input;
            self
        }
        /// <p>The ID of the customer-owned IPv4 address pool used for the endpoint.</p>
        pub fn customer_owned_ipv4_pool(mut self, input: impl Into<std::string::String>) -> Self {
            self.customer_owned_ipv4_pool = Some(input.into());
            self
        }
        /// <p>The ID of the customer-owned IPv4 address pool used for the endpoint.</p>
        pub fn set_customer_owned_ipv4_pool(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.customer_owned_ipv4_pool = input;
            self
        }
        /// Consumes the builder and constructs a [`Endpoint`](crate::model::Endpoint).
        pub fn build(self) -> crate::model::Endpoint {
            crate::model::Endpoint {
                endpoint_arn: self.endpoint_arn,
                outposts_id: self.outposts_id,
                cidr_block: self.cidr_block,
                status: self.status,
                creation_time: self.creation_time,
                network_interfaces: self.network_interfaces,
                vpc_id: self.vpc_id,
                subnet_id: self.subnet_id,
                security_group_id: self.security_group_id,
                access_type: self.access_type,
                customer_owned_ipv4_pool: self.customer_owned_ipv4_pool,
            }
        }
    }
}
impl Endpoint {
    /// Creates a new builder-style object to manufacture [`Endpoint`](crate::model::Endpoint).
    pub fn builder() -> crate::model::endpoint::Builder {
        crate::model::endpoint::Builder::default()
    }
}

/// When writing a match expression against `EndpointAccessType`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let endpointaccesstype = unimplemented!();
/// match endpointaccesstype {
///     EndpointAccessType::CustomerOwnedIp => { /* ... */ },
///     EndpointAccessType::Private => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `endpointaccesstype` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `EndpointAccessType::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `EndpointAccessType::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `EndpointAccessType::NewFeature` is defined.
/// Specifically, when `endpointaccesstype` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `EndpointAccessType::NewFeature` also yielding `"NewFeature"`.
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
pub enum EndpointAccessType {
    #[allow(missing_docs)] // documentation missing in model
    CustomerOwnedIp,
    #[allow(missing_docs)] // documentation missing in model
    Private,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue),
}
impl std::convert::From<&str> for EndpointAccessType {
    fn from(s: &str) -> Self {
        match s {
            "CustomerOwnedIp" => EndpointAccessType::CustomerOwnedIp,
            "Private" => EndpointAccessType::Private,
            other => {
                EndpointAccessType::Unknown(crate::types::UnknownVariantValue(other.to_owned()))
            }
        }
    }
}
impl std::str::FromStr for EndpointAccessType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(EndpointAccessType::from(s))
    }
}
impl EndpointAccessType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            EndpointAccessType::CustomerOwnedIp => "CustomerOwnedIp",
            EndpointAccessType::Private => "Private",
            EndpointAccessType::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["CustomerOwnedIp", "Private"]
    }
}
impl AsRef<str> for EndpointAccessType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>The container for the network interface.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct NetworkInterface {
    /// <p>The ID for the network interface.</p>
    #[doc(hidden)]
    pub network_interface_id: std::option::Option<std::string::String>,
}
impl NetworkInterface {
    /// <p>The ID for the network interface.</p>
    pub fn network_interface_id(&self) -> std::option::Option<&str> {
        self.network_interface_id.as_deref()
    }
}
impl std::fmt::Debug for NetworkInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("NetworkInterface");
        formatter.field("network_interface_id", &self.network_interface_id);
        formatter.finish()
    }
}
/// See [`NetworkInterface`](crate::model::NetworkInterface).
pub mod network_interface {

    /// A builder for [`NetworkInterface`](crate::model::NetworkInterface).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) network_interface_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ID for the network interface.</p>
        pub fn network_interface_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.network_interface_id = Some(input.into());
            self
        }
        /// <p>The ID for the network interface.</p>
        pub fn set_network_interface_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.network_interface_id = input;
            self
        }
        /// Consumes the builder and constructs a [`NetworkInterface`](crate::model::NetworkInterface).
        pub fn build(self) -> crate::model::NetworkInterface {
            crate::model::NetworkInterface {
                network_interface_id: self.network_interface_id,
            }
        }
    }
}
impl NetworkInterface {
    /// Creates a new builder-style object to manufacture [`NetworkInterface`](crate::model::NetworkInterface).
    pub fn builder() -> crate::model::network_interface::Builder {
        crate::model::network_interface::Builder::default()
    }
}

/// When writing a match expression against `EndpointStatus`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let endpointstatus = unimplemented!();
/// match endpointstatus {
///     EndpointStatus::Available => { /* ... */ },
///     EndpointStatus::Deleting => { /* ... */ },
///     EndpointStatus::Pending => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `endpointstatus` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `EndpointStatus::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `EndpointStatus::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `EndpointStatus::NewFeature` is defined.
/// Specifically, when `endpointstatus` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `EndpointStatus::NewFeature` also yielding `"NewFeature"`.
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
pub enum EndpointStatus {
    #[allow(missing_docs)] // documentation missing in model
    Available,
    #[allow(missing_docs)] // documentation missing in model
    Deleting,
    #[allow(missing_docs)] // documentation missing in model
    Pending,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue),
}
impl std::convert::From<&str> for EndpointStatus {
    fn from(s: &str) -> Self {
        match s {
            "Available" => EndpointStatus::Available,
            "Deleting" => EndpointStatus::Deleting,
            "Pending" => EndpointStatus::Pending,
            other => EndpointStatus::Unknown(crate::types::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl std::str::FromStr for EndpointStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(EndpointStatus::from(s))
    }
}
impl EndpointStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            EndpointStatus::Available => "Available",
            EndpointStatus::Deleting => "Deleting",
            EndpointStatus::Pending => "Pending",
            EndpointStatus::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["Available", "Deleting", "Pending"]
    }
}
impl AsRef<str> for EndpointStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
