resource "aws_ecr_repository" "lambda" {
  name                 = "${local.prefix_lower}-lambda"
  image_tag_mutability = "MUTABLE"
  force_delete         = true
  image_scanning_configuration {
    scan_on_push = false
  }
}

resource "aws_ecr_lifecycle_policy" "lambda" {
  repository = aws_ecr_repository.lambda.name

  # May take up to 24 hours to expire old images
  policy = <<EOF
    {
        "rules": [
        {
            "rulePriority": 1,
            "description": "Keep last image",
            "selection": {
            "tagStatus": "any",
            "countType": "imageCountMoreThan",
            "countNumber": 1
            },
            "action": {
            "type": "expire"
            }
        }
        ]
    }
    EOF
}
