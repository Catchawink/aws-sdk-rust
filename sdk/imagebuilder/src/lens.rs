// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_component_build_versions_output_next_token(
    input: &crate::output::ListComponentBuildVersionsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_components_output_next_token(
    input: &crate::output::ListComponentsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_container_recipes_output_next_token(
    input: &crate::output::ListContainerRecipesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_distribution_configurations_output_next_token(
    input: &crate::output::ListDistributionConfigurationsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_image_build_versions_output_next_token(
    input: &crate::output::ListImageBuildVersionsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_image_packages_output_next_token(
    input: &crate::output::ListImagePackagesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_image_pipeline_images_output_next_token(
    input: &crate::output::ListImagePipelineImagesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_image_pipelines_output_next_token(
    input: &crate::output::ListImagePipelinesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_image_recipes_output_next_token(
    input: &crate::output::ListImageRecipesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_images_output_next_token(
    input: &crate::output::ListImagesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_infrastructure_configurations_output_next_token(
    input: &crate::output::ListInfrastructureConfigurationsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_component_build_versions_output_component_summary_list(
    input: crate::output::ListComponentBuildVersionsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ComponentSummary>> {
    let input = match input.component_summary_list {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_components_output_component_version_list(
    input: crate::output::ListComponentsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ComponentVersion>> {
    let input = match input.component_version_list {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_container_recipes_output_container_recipe_summary_list(
    input: crate::output::ListContainerRecipesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ContainerRecipeSummary>> {
    let input = match input.container_recipe_summary_list {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_distribution_configurations_output_distribution_configuration_summary_list(
    input: crate::output::ListDistributionConfigurationsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::DistributionConfigurationSummary>> {
    let input = match input.distribution_configuration_summary_list {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_image_build_versions_output_image_summary_list(
    input: crate::output::ListImageBuildVersionsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ImageSummary>> {
    let input = match input.image_summary_list {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_image_packages_output_image_package_list(
    input: crate::output::ListImagePackagesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ImagePackage>> {
    let input = match input.image_package_list {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_image_pipeline_images_output_image_summary_list(
    input: crate::output::ListImagePipelineImagesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ImageSummary>> {
    let input = match input.image_summary_list {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_image_pipelines_output_image_pipeline_list(
    input: crate::output::ListImagePipelinesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ImagePipeline>> {
    let input = match input.image_pipeline_list {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_image_recipes_output_image_recipe_summary_list(
    input: crate::output::ListImageRecipesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ImageRecipeSummary>> {
    let input = match input.image_recipe_summary_list {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_images_output_image_version_list(
    input: crate::output::ListImagesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ImageVersion>> {
    let input = match input.image_version_list {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_infrastructure_configurations_output_infrastructure_configuration_summary_list(
    input: crate::output::ListInfrastructureConfigurationsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::InfrastructureConfigurationSummary>> {
    let input = match input.infrastructure_configuration_summary_list {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
