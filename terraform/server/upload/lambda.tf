resource "aws_cloudwatch_log_group" "api" {
  name              = "/aws/lambda/${local.prefix}-API"
  retention_in_days = 90
}

resource "aws_lambda_function" "api" {
  package_type  = "Image"
  image_uri     = "${data.aws_ssm_parameter.ecr_url.insecure_value}@${data.aws_ecr_image.lambda.id}"
  function_name = "${local.prefix}-API"
  role          = aws_iam_role.lambda_api.arn
  timeout       = 5
  depends_on = [
    aws_cloudwatch_log_group.api,
    terraform_data.lambda_push,
  ]
  environment {
    variables = {
      REQUIREMENTS_TABLE_NAME=data.aws_ssm_parameter.requirements_table_name.insecure_value,
      SPATIAL_DISTANCES_TABLE_NAME=data.aws_ssm_parameter.spatial_distances_table_name.insecure_value,
    }
  }
}

resource "aws_iam_role" "lambda_api" {
  name               = "${local.prefix}-API"
  description        = "Allows Lambda run"
  assume_role_policy = data.aws_iam_policy_document.lambda_assume_role.json
}

resource "aws_iam_role_policy_attachment" "execute_api_lambda" {
  role       = aws_iam_role.lambda_api.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

data "aws_iam_policy_document" "lambda_assume_role" {
  statement {
    actions = ["sts:AssumeRole"]
    effect  = "Allow"
    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
  }
}

