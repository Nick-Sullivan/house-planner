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

module "shared_locals" {
  source = "../../modules/shared_locals"
}

locals {
  region              = module.shared_locals.region
  environment         = module.shared_locals.environment
  prefix_lower        = module.shared_locals.prefix_lower
  prefix_parameter    = module.shared_locals.prefix_parameter
  browser_dir         = module.shared_locals.browser_dir
  tags                = module.shared_locals.tags
  build_folder        = "${local.browser_dir}/build"
  build_folder_client = "${local.build_folder}/client"
}

provider "aws" {
  region = "ap-southeast-2"
  default_tags {
    tags = local.tags
  }
}
