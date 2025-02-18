resource "terraform_data" "build" {
  # always trigger a rebuild. In bigger projects we can split this
  # out into a deployment script instead of bundling it with the infrastructure.
  triggers_replace = [timestamp()]
  provisioner "local-exec" {
    working_dir = local.browser_dir
    command     = "npm run build"
  }
}
