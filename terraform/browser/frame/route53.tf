
# # Create a hosted zone for our domain and point it to CloudFront.

# resource "aws_route53_zone" "website" {
#   name              = local.url
#   delegation_set_id = data.aws_ssm_parameter.delegation_set_id.value
# }

# resource "aws_route53_record" "ipv4" {
#   zone_id = aws_route53_zone.website.zone_id
#   name    = local.url
#   type    = "A"
#   alias {
#     name                   = module.cloudfront.domain_name
#     zone_id                = module.cloudfront.hosted_zone_id
#     evaluate_target_health = false
#   }
# }

# resource "aws_route53_record" "ipv6" {
#   zone_id = aws_route53_zone.website.zone_id
#   name    = local.url
#   type    = "AAAA"
#   alias {
#     name                   = module.cloudfront.domain_name
#     zone_id                = module.cloudfront.hosted_zone_id
#     evaluate_target_health = false
#   }
# }

# resource "aws_route53_record" "ipv4_www" {
#   zone_id = aws_route53_zone.website.zone_id
#   name    = local.url_www
#   type    = "A"
#   alias {
#     name                   = module.cloudfront.domain_name
#     zone_id                = module.cloudfront.hosted_zone_id
#     evaluate_target_health = false
#   }
# }

# resource "aws_route53_record" "ipv6_www" {
#   zone_id = aws_route53_zone.website.zone_id
#   name    = local.url_www
#   type    = "AAAA"
#   alias {
#     name                   = module.cloudfront.domain_name
#     zone_id                = module.cloudfront.hosted_zone_id
#     evaluate_target_health = false
#   }
# }
