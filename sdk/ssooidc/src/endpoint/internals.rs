// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(
    clippy::collapsible_if,
    clippy::bool_comparison,
    clippy::nonminimal_bool,
    clippy::comparison_to_empty,
    clippy::redundant_pattern_matching
)]
pub(super) fn resolve_endpoint(
    _params: &crate::endpoint::Params,
    _diagnostic_collector: &mut crate::endpoint_lib::diagnostic::DiagnosticCollector,
    partition_resolver: &crate::endpoint_lib::partition::PartitionResolver,
) -> aws_smithy_http::endpoint::Result {
    #[allow(unused)]
    let region = &_params.region;
    #[allow(unused)]
    let use_dual_stack = &_params.use_dual_stack;
    #[allow(unused)]
    let use_fips = &_params.use_fips;
    #[allow(unused)]
    let endpoint = &_params.endpoint;
    #[allow(unused)]
    if let Some(partition_result) =
        partition_resolver.resolve_partition(region, _diagnostic_collector)
    {
        #[allow(unused)]
        if let Some(endpoint) = endpoint {
            if (*use_fips) == (true) {
                return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
                    "Invalid Configuration: FIPS and custom endpoint are not supported".to_string(),
                ));
            }
            if (*use_dual_stack) == (true) {
                return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
                    "Invalid Configuration: Dualstack and custom endpoint are not supported"
                        .to_string(),
                ));
            }
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url(endpoint.to_owned())
                .build());
        }
        if (*use_fips) == (true) {
            if (*use_dual_stack) == (true) {
                if (true) == (partition_result.supports_fips()) {
                    if (true) == (partition_result.supports_dual_stack()) {
                        return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                            .url({
                                let mut out = String::new();
                                out.push_str("https://oidc-fips.");
                                #[allow(clippy::needless_borrow)]
                                out.push_str(&region);
                                out.push('.');
                                #[allow(clippy::needless_borrow)]
                                out.push_str(&partition_result.dual_stack_dns_suffix());
                                out
                            })
                            .build());
                    }
                }
                return Err(aws_smithy_http::endpoint::ResolveEndpointError::message("FIPS and DualStack are enabled, but this partition does not support one or both"
.to_string()));
            }
        }
        if (*use_fips) == (true) {
            if (true) == (partition_result.supports_fips()) {
                return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                    .url({
                        let mut out = String::new();
                        out.push_str("https://oidc-fips.");
                        #[allow(clippy::needless_borrow)]
                        out.push_str(&region);
                        out.push('.');
                        #[allow(clippy::needless_borrow)]
                        out.push_str(&partition_result.dns_suffix());
                        out
                    })
                    .build());
            }
            return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
                "FIPS is enabled but this partition does not support FIPS".to_string(),
            ));
        }
        if (*use_dual_stack) == (true) {
            if (true) == (partition_result.supports_dual_stack()) {
                return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                    .url({
                        let mut out = String::new();
                        out.push_str("https://oidc.");
                        #[allow(clippy::needless_borrow)]
                        out.push_str(&region);
                        out.push('.');
                        #[allow(clippy::needless_borrow)]
                        out.push_str(&partition_result.dual_stack_dns_suffix());
                        out
                    })
                    .build());
            }
            return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
                "DualStack is enabled but this partition does not support DualStack".to_string(),
            ));
        }
        if (region) == ("ap-east-1") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.ap-east-1.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("ap-northeast-1") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.ap-northeast-1.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("ap-northeast-2") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.ap-northeast-2.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("ap-northeast-3") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.ap-northeast-3.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("ap-south-1") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.ap-south-1.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("ap-southeast-1") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.ap-southeast-1.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("ap-southeast-2") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.ap-southeast-2.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("ca-central-1") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.ca-central-1.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("eu-central-1") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.eu-central-1.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("eu-north-1") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.eu-north-1.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("eu-south-1") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.eu-south-1.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("eu-west-1") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.eu-west-1.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("eu-west-2") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.eu-west-2.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("eu-west-3") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.eu-west-3.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("me-south-1") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.me-south-1.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("sa-east-1") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.sa-east-1.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("us-east-1") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.us-east-1.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("us-east-2") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.us-east-2.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("us-west-2") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.us-west-2.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("us-gov-east-1") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.us-gov-east-1.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("us-gov-west-1") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://oidc.us-gov-west-1.amazonaws.com".to_string())
                .build());
        }
        return Ok(aws_smithy_types::endpoint::Endpoint::builder()
            .url({
                let mut out = String::new();
                out.push_str("https://oidc.");
                #[allow(clippy::needless_borrow)]
                out.push_str(&region);
                out.push('.');
                #[allow(clippy::needless_borrow)]
                out.push_str(&partition_result.dns_suffix());
                out
            })
            .build());
    }
    #[allow(unreachable_code)]
    return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
        format!(
            "No rules matched these parameters. This is a bug. {:?}",
            _params
        ),
    ));
}
