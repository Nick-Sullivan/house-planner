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

data "aws_caller_identity" "identity" {}

locals {
  region                    = "eu-west-2"
  prefix                    = "HousePlanner-${title(var.environment)}"
  prefix_lower              = "house-planner-${lower(var.environment)}"
  prefix_parameter          = "/HousePlanner/${title(var.environment)}"
  aws_account_id            = data.aws_caller_identity.identity.account_id
  root_dir                  = "${path.root}/.."
  browser_dir                = "${local.root_dir}/browser"
  server_dir                = "${local.root_dir}/server"
  tags = {
    Project     = "House Planner"
    Environment = var.environment
  }
}