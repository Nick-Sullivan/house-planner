# Order of creation is important
# method -> integration -> method response -> integration response

locals {
  # this determines when to redeploy API gateway
  all_integrations = [
    aws_api_gateway_method.houses,
    aws_api_gateway_integration.houses,
    aws_api_gateway_method_response.houses_200,

    # aws_api_gateway_method.swagger,
    # aws_api_gateway_integration.swagger,
    # aws_api_gateway_method_response.swagger_200,
  ]
}

# houses

resource "aws_api_gateway_resource" "houses" {
  path_part   = "houses"
  parent_id   = aws_api_gateway_rest_api.gateway.root_resource_id
  rest_api_id = aws_api_gateway_rest_api.gateway.id
}

resource "aws_api_gateway_method" "houses" {
  rest_api_id   = aws_api_gateway_rest_api.gateway.id
  resource_id   = aws_api_gateway_resource.houses.id
  http_method   = "GET"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "houses" {
  rest_api_id             = aws_api_gateway_rest_api.gateway.id
  resource_id             = aws_api_gateway_resource.houses.id
  http_method             = aws_api_gateway_method.houses.http_method
  uri                     = aws_lambda_function.api.invoke_arn
  content_handling        = "CONVERT_TO_TEXT"
  integration_http_method = "POST"
  type                    = "AWS_PROXY"
}

resource "aws_api_gateway_method_response" "houses_200" {
  rest_api_id = aws_api_gateway_rest_api.gateway.id
  resource_id = aws_api_gateway_resource.houses.id
  http_method = aws_api_gateway_integration.houses.http_method
  status_code = "200"
  response_parameters = {
    "method.response.header.Access-Control-Allow-Headers" = true
    "method.response.header.Access-Control-Allow-Methods" = true
    "method.response.header.Access-Control-Allow-Origin"  = true
  }
}

# # swagger

# resource "aws_api_gateway_resource" "swagger" {
#   path_part   = "swagger"
#   parent_id   = aws_api_gateway_rest_api.gateway.root_resource_id
#   rest_api_id = aws_api_gateway_rest_api.gateway.id
# }

# resource "aws_api_gateway_method" "swagger" {
#   rest_api_id   = aws_api_gateway_rest_api.gateway.id
#   resource_id   = aws_api_gateway_resource.swagger.id
#   http_method   = "GET"
#   authorization = "NONE"
# }

# resource "aws_api_gateway_integration" "swagger" {
#   rest_api_id             = aws_api_gateway_rest_api.gateway.id
#   resource_id             = aws_api_gateway_resource.swagger.id
#   http_method             = aws_api_gateway_method.swagger.http_method
#   uri                     = aws_lambda_function.api.invoke_arn
#   content_handling        = "CONVERT_TO_TEXT"
#   integration_http_method = "POST"
#   type                    = "AWS_PROXY"
# }

# resource "aws_api_gateway_method_response" "swagger_200" {
#   rest_api_id = aws_api_gateway_rest_api.gateway.id
#   resource_id = aws_api_gateway_resource.swagger.id
#   http_method = aws_api_gateway_integration.swagger.http_method
#   status_code = "200"
#   response_parameters = {
#     "method.response.header.Access-Control-Allow-Headers" = true
#     "method.response.header.Access-Control-Allow-Methods" = true
#     "method.response.header.Access-Control-Allow-Origin"  = true
#   }
# }

