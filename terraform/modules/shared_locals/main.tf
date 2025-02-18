data "aws_caller_identity" "identity" {}

locals {
  region           = "eu-west-2"
  prefix           = "HousePlanner-${title(local.environment)}"
  prefix_lower     = "house-planner-${lower(local.environment)}"
  prefix_parameter = "/HousePlanner/${title(local.environment)}"
  aws_account_id   = data.aws_caller_identity.identity.account_id
  root_dir         = abspath("${path.root}/../../..")
  browser_dir      = "${local.root_dir}/browser"
  server_dir       = "${local.root_dir}/server"
  tags = {
    Project     = "House Planner"
    Environment = local.environment
  }
}
