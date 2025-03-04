
resource "aws_dynamodb_table" "spatial_distances" {
  name         = "${local.prefix}-SpatialDistances"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "SourceIndex"
  range_key    = "DestinationIndex"
  attribute {
    name = "SourceIndex"
    type = "S"
  }
  attribute {
    name = "DestinationIndex"
    type = "S"
  }
  attribute {
    name = "CityCode"
    type = "S"
  }
  global_secondary_index {
    name            = "CityCodeIndex"
    hash_key        = "CityCode"
    range_key       = "SourceIndex"
    projection_type = "ALL"
  }
}

resource "aws_dynamodb_table" "requirements" {
  name         = "${local.prefix}-Requirements"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "RequirementId"
  attribute {
    name = "RequirementId"
    type = "S"
  }
  attribute {
    name = "CityCode"
    type = "S"
  }
  global_secondary_index {
    name            = "CityCodeIndex"
    hash_key        = "CityCode"
    projection_type = "ALL"
  }
  ttl {
    attribute_name = "TimeToLive"
    enabled        = true
  }
}

resource "aws_dynamodb_table" "houses" {
  name         = "${local.prefix}-Houses"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "H3Index"
  range_key    = "Address"
  attribute {
    name = "H3Index"
    type = "S"
  }
  attribute {
    name = "Address"
    type = "S"
  }
  attribute {
    name = "CityCode"
    type = "S"
  }
  global_secondary_index {
    name            = "CityCodeIndex"
    hash_key        = "CityCode"
    range_key       = "H3Index"
    projection_type = "ALL"
  }
  ttl {
    attribute_name = "TimeToLive"
    enabled        = true
  }
}
