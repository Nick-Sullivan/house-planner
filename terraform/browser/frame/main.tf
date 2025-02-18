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
  region           = module.shared_locals.region
  environment      = module.shared_locals.environment
  prefix_lower     = module.shared_locals.prefix_lower
  prefix_parameter = module.shared_locals.prefix_parameter
  tags             = module.shared_locals.tags
  url              = "houseplanner.com"
  url_www          = "www.${local.url}"
}

provider "aws" {
  region = "ap-southeast-2"
  default_tags {
    tags = local.tags
  }
}

provider "aws" {
  region = "us-east-1"
  alias  = "us-east-1"
  default_tags {
    tags = local.tags
  }
}


# Create a CloudFront distribution that redirects to the S3 bucket, and allows SSL

# module "cloudfront" {
#   source            = "./../modules/website_cloudfront"
#   domain_name       = local.url
#   alternative_names = [local.url_www]
#   # We use the URL rather than the bucket_regional_domain_name so that subfolders can be loaded correctly.
#   # From https://stackoverflow.com/questions/31017105/how-do-you-set-a-default-root-object-for-subdirectories-for-a-statically-hosted/65146447#65146447
#   redirect_url = aws_s3_bucket.website.website_endpoint
#   zone_id      = aws_route53_zone.website.zone_id
#   providers = {
#     aws = aws.us-east-1
#   }
# }
