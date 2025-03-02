# Order of creation is important
# method -> integration -> method response -> integration response

locals {
  # this determines when to redeploy API gateway
  all_integrations = [
    aws_api_gateway_method.houses,
    aws_api_gateway_integration.houses,
    aws_api_gateway_method_response.houses_200,

    aws_api_gateway_method.maps,
    aws_api_gateway_integration.maps,
    aws_api_gateway_method_response.maps_200,

    aws_api_gateway_method.maps_options,
    aws_api_gateway_integration.maps_options,
    aws_api_gateway_method_response.maps_options_200,
    aws_api_gateway_integration_response.maps_options_200,

    aws_api_gateway_method.maps_requirements,
    aws_api_gateway_integration.maps_requirements,
    aws_api_gateway_method_response.maps_requirements_200,

    aws_api_gateway_method.maps_requirements_options,
    aws_api_gateway_integration.maps_requirements_options,
    aws_api_gateway_method_response.maps_requirements_options_200,
    aws_api_gateway_integration_response.maps_requirements_options_200,

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

# maps

resource "aws_api_gateway_resource" "maps" {
  path_part   = "maps"
  parent_id   = aws_api_gateway_rest_api.gateway.root_resource_id
  rest_api_id = aws_api_gateway_rest_api.gateway.id
}

resource "aws_api_gateway_method" "maps" {
  rest_api_id   = aws_api_gateway_rest_api.gateway.id
  resource_id   = aws_api_gateway_resource.maps.id
  http_method   = "POST"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "maps" {
  rest_api_id             = aws_api_gateway_rest_api.gateway.id
  resource_id             = aws_api_gateway_resource.maps.id
  http_method             = aws_api_gateway_method.maps.http_method
  uri                     = aws_lambda_function.api.invoke_arn
  content_handling        = "CONVERT_TO_TEXT"
  integration_http_method = "POST"
  type                    = "AWS_PROXY"
}

resource "aws_api_gateway_method_response" "maps_200" {
  rest_api_id = aws_api_gateway_rest_api.gateway.id
  resource_id = aws_api_gateway_resource.maps.id
  http_method = aws_api_gateway_integration.maps.http_method
  status_code = "200"
  response_parameters = {
    "method.response.header.Access-Control-Allow-Headers" = true
    "method.response.header.Access-Control-Allow-Methods" = true
    "method.response.header.Access-Control-Allow-Origin"  = true
  }
}

resource "aws_api_gateway_method" "maps_options" {
  rest_api_id   = aws_api_gateway_rest_api.gateway.id
  resource_id   = aws_api_gateway_resource.maps.id
  http_method   = "OPTIONS"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "maps_options" {
  rest_api_id = aws_api_gateway_rest_api.gateway.id
  resource_id = aws_api_gateway_resource.maps.id
  http_method = aws_api_gateway_method.maps_options.http_method
  type        = "MOCK"
  request_templates = {
    "application/json" = "{\"statusCode\": 200}"
  }
}

resource "aws_api_gateway_method_response" "maps_options_200" {
  rest_api_id = aws_api_gateway_rest_api.gateway.id
  resource_id = aws_api_gateway_resource.maps.id
  http_method = aws_api_gateway_method.maps_options.http_method
  status_code = "200"
  response_parameters = {
    "method.response.header.Access-Control-Allow-Headers" = true
    "method.response.header.Access-Control-Allow-Methods" = true
    "method.response.header.Access-Control-Allow-Origin"  = true
  }
}

resource "aws_api_gateway_integration_response" "maps_options_200" {
  rest_api_id = aws_api_gateway_rest_api.gateway.id
  resource_id = aws_api_gateway_resource.maps.id
  http_method = aws_api_gateway_method.maps_options.http_method
  status_code = "200"
  response_parameters = {
    "method.response.header.Access-Control-Allow-Headers" = "'*'"
    "method.response.header.Access-Control-Allow-Methods" = "'POST,OPTIONS'"
    "method.response.header.Access-Control-Allow-Origin"  = "'*'"
  }
  response_templates = {
    "application/json" = ""
  }
}

# /maps/requirements

resource "aws_api_gateway_resource" "maps_requirements" {
  path_part   = "requirements"
  parent_id   = aws_api_gateway_resource.maps.id
  rest_api_id = aws_api_gateway_rest_api.gateway.id
}

resource "aws_api_gateway_method" "maps_requirements" {
  rest_api_id   = aws_api_gateway_rest_api.gateway.id
  resource_id   = aws_api_gateway_resource.maps_requirements.id
  http_method   = "POST"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "maps_requirements" {
  rest_api_id             = aws_api_gateway_rest_api.gateway.id
  resource_id             = aws_api_gateway_resource.maps_requirements.id
  http_method             = aws_api_gateway_method.maps_requirements.http_method
  uri                     = aws_lambda_function.api.invoke_arn
  content_handling        = "CONVERT_TO_TEXT"
  integration_http_method = "POST"
  type                    = "AWS_PROXY"
}

resource "aws_api_gateway_method_response" "maps_requirements_200" {
  rest_api_id = aws_api_gateway_rest_api.gateway.id
  resource_id = aws_api_gateway_resource.maps_requirements.id
  http_method = aws_api_gateway_integration.maps_requirements.http_method
  status_code = "200"
  response_parameters = {
    "method.response.header.Access-Control-Allow-Headers" = true
    "method.response.header.Access-Control-Allow-Methods" = true
    "method.response.header.Access-Control-Allow-Origin"  = true
  }
}

resource "aws_api_gateway_method" "maps_requirements_options" {
  rest_api_id   = aws_api_gateway_rest_api.gateway.id
  resource_id   = aws_api_gateway_resource.maps_requirements.id
  http_method   = "OPTIONS"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "maps_requirements_options" {
  rest_api_id = aws_api_gateway_rest_api.gateway.id
  resource_id = aws_api_gateway_resource.maps_requirements.id
  http_method = aws_api_gateway_method.maps_requirements_options.http_method
  type        = "MOCK"
  request_templates = {
    "application/json" = "{\"statusCode\": 200}"
  }
}

resource "aws_api_gateway_method_response" "maps_requirements_options_200" {
  rest_api_id = aws_api_gateway_rest_api.gateway.id
  resource_id = aws_api_gateway_resource.maps_requirements.id
  http_method = aws_api_gateway_method.maps_requirements_options.http_method
  status_code = "200"
  response_parameters = {
    "method.response.header.Access-Control-Allow-Headers" = true
    "method.response.header.Access-Control-Allow-Methods" = true
    "method.response.header.Access-Control-Allow-Origin"  = true
  }
}

resource "aws_api_gateway_integration_response" "maps_requirements_options_200" {
  rest_api_id = aws_api_gateway_rest_api.gateway.id
  resource_id = aws_api_gateway_resource.maps_requirements.id
  http_method = aws_api_gateway_method.maps_requirements_options.http_method
  status_code = "200"
  response_parameters = {
    "method.response.header.Access-Control-Allow-Headers" = "'*'"
    "method.response.header.Access-Control-Allow-Methods" = "'POST,OPTIONS'"
    "method.response.header.Access-Control-Allow-Origin"  = "'*'"
  }
  response_templates = {
    "application/json" = ""
  }
}
