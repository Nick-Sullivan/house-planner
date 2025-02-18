
data "aws_ssm_parameter" "ecr_name" {
  name = "${local.prefix_parameter}/ECR/Name"
}

data "aws_ssm_parameter" "ecr_url" {
  name = "${local.prefix_parameter}/ECR/Url"
}
