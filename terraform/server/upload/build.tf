

resource "terraform_data" "lambda_build" {
  # always trigger a rebuild. In bigger projects we can split this
  # out into a deployment script instead of bundling it with the infrastructure.
  triggers_replace = [timestamp()]
  provisioner "local-exec" {
    working_dir = local.server_dir
    command     = "docker build -t ${data.aws_ssm_parameter.ecr_url.insecure_value} ."
  }
}

resource "terraform_data" "lambda_push" {
  depends_on = [terraform_data.lambda_build]
  lifecycle {
    replace_triggered_by = [terraform_data.lambda_build]
  }
  provisioner "local-exec" {
    working_dir = local.server_dir
    command     = "aws ecr get-login-password --region ${local.region} | docker login --username AWS --password-stdin ${local.aws_account_id}.dkr.ecr.${local.region}.amazonaws.com"
  }
  provisioner "local-exec" {
    working_dir = local.server_dir
    command     = "docker push ${data.aws_ssm_parameter.ecr_url.insecure_value}:latest"
  }
}

data "aws_ecr_image" "lambda" {
  depends_on = [
    terraform_data.lambda_push
  ]
  repository_name = data.aws_ssm_parameter.ecr_name.insecure_value
  image_tag       = "latest"
}
