
# module "template_files" {
#   # Calculates the content_type of each file.
#   # https://registry.terraform.io/modules/hashicorp/dir/template/latest
#   depends_on = [terraform_data.build]
#   source     = "hashicorp/dir/template"
#   base_dir   = local.build_folder_client
# }

# resource "aws_s3_object" "static_files" {
#   # Loads all files to the s3 bucket
#   for_each     = module.template_files.files
#   bucket       = data.aws_ssm_parameter.s3_bucket_id.value
#   key          = each.key
#   content_type = each.value.content_type
#   source       = each.value.source_path
#   content      = each.value.content
#   etag         = each.value.digests.md5
# }

# resource "aws_s3_object" "upload_files" {
#   depends_on = [terraform_data.build]
#   bucket     = data.aws_ssm_parameter.s3_bucket_name.insecure_value
#   for_each   = fileset(local.build_folder_client, "**/*.*")
#   key        = each.value
#   source     = "${local.build_folder_client}/${each.value}"
#   etag       = filemd5("${local.build_folder_client}/${each.value}")
# }

resource "terraform_data" "upload" {
  depends_on       = [terraform_data.build]
  triggers_replace = [timestamp()]
  provisioner "local-exec" {
    working_dir = local.browser_dir
    command     = "aws s3 cp ${local.build_folder_client} s3://${data.aws_ssm_parameter.s3_bucket_name.insecure_value}/ --recursive"
  }
}
