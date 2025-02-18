
resource "aws_ssm_parameter" "ecr_name" {
  name  = "${local.prefix_parameter}/ECR/Name"
  type  = "String"
  value = aws_ecr_repository.lambda.name
}

resource "aws_ssm_parameter" "ecr_url" {
  name  = "${local.prefix_parameter}/ECR/Url"
  type  = "String"
  value = aws_ecr_repository.lambda.repository_url
}
