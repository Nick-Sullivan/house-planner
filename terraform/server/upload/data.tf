
data "aws_ssm_parameter" "ecr_name" {
  name = "${local.prefix_parameter}/ECR/Name"
}

data "aws_ssm_parameter" "ecr_url" {
  name = "${local.prefix_parameter}/ECR/Url"
}

data "aws_ssm_parameter" "spatial_distances_table_name" {
  name = "${local.prefix_parameter}/DynamoDB/SpatialDistancesTable/Name"
}

data "aws_ssm_parameter" "spatial_distances_table_arn" {
  name = "${local.prefix_parameter}/DynamoDB/SpatialDistancesTable/Arn"
}

data "aws_ssm_parameter" "requirements_table_name" {
  name = "${local.prefix_parameter}/DynamoDB/RequirementsTable/Name"
}

data "aws_ssm_parameter" "requirements_table_arn" {
  name = "${local.prefix_parameter}/DynamoDB/RequirementsTable/Arn"
}

data "aws_ssm_parameter" "houses_table_name" {
  name = "${local.prefix_parameter}/DynamoDB/HousesTable/Name"
}

data "aws_ssm_parameter" "houses_table_arn" {
  name = "${local.prefix_parameter}/DynamoDB/HousesTable/Arn"
}
