# resource "terraform_data" "clear_cloudfront_cache" {
#   depends_on       = [aws_s3_object.static_files]
#   triggers_replace = [timestamp()] # Triggers every time

#   provisioner "local-exec" {
#     command = "aws cloudfront create-invalidation --distribution-id ${data.aws_ssm_parameter.cloudfront_distribution_id.value} --paths '/*'"
#   }
# }
