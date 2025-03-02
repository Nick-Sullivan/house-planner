
resource "aws_ssm_parameter" "spatial_distances_table_name" {
  name  = "${local.prefix_parameter}/DynamoDB/SpatialDistancesTable/Name"
  type  = "String"
  value = aws_dynamodb_table.spatial_distances.name
}

resource "aws_ssm_parameter" "spatial_distances_table_arn" {
  name  = "${local.prefix_parameter}/DynamoDB/SpatialDistancesTable/Arn"
  type  = "String"
  value = aws_dynamodb_table.spatial_distances.arn
}

resource "aws_ssm_parameter" "requirements_table_name" {
  name  = "${local.prefix_parameter}/DynamoDB/RequirementsTable/Name"
  type  = "String"
  value = aws_dynamodb_table.requirements.name
}

resource "aws_ssm_parameter" "requirements_table_arn" {
  name  = "${local.prefix_parameter}/DynamoDB/RequirementsTable/Arn"
  type  = "String"
  value = aws_dynamodb_table.requirements.arn
}
