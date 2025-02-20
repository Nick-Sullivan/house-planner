
data "aws_ssm_parameter" "ecr_name" {
  name = "${local.prefix_parameter}/ECR/Name"
}

data "aws_ssm_parameter" "ecr_url" {
  name = "${local.prefix_parameter}/ECR/Url"
}

data "aws_ssm_parameter" "spatial_distances_table_name" {
  name = "${local.prefix_parameter}/DynamoDB/SpatialDistancesTable/Name"
}

data "aws_ssm_parameter" "requirements_table_name" {
  name = "${local.prefix_parameter}/DynamoDB/RequirementsTable/Name"
}
