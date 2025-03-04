terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "5.56.1"
    }
  }
  backend "s3" {
    bucket = "nicks-terraform-states"
    region = "ap-southeast-2"
  }
}

provider "aws" {
  region = local.region
  default_tags {
    tags = local.tags
  }
}

module "shared_locals" {
  source = "../../modules/shared_locals"
}

locals {
  environment      = module.shared_locals.environment
  region           = module.shared_locals.region
  prefix           = module.shared_locals.prefix
  prefix_lower     = module.shared_locals.prefix_lower
  prefix_parameter = module.shared_locals.prefix_parameter
  aws_account_id   = module.shared_locals.aws_account_id
  root_dir         = module.shared_locals.root_dir
  browser_dir      = module.shared_locals.browser_dir
  server_dir       = module.shared_locals.server_dir
  tags             = module.shared_locals.tags
}
