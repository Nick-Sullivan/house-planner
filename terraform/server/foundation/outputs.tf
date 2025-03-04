
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

resource "aws_ssm_parameter" "houses_table_name" {
  name  = "${local.prefix_parameter}/DynamoDB/HousesTable/Name"
  type  = "String"
  value = aws_dynamodb_table.houses.name
}

resource "aws_ssm_parameter" "houses_table_arn" {
  name  = "${local.prefix_parameter}/DynamoDB/HousesTable/Arn"
  type  = "String"
  value = aws_dynamodb_table.houses.arn
}
