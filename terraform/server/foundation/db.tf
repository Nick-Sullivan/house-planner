
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
  # attribute {
  #   name = "Distances"
  #   type = "S"
  # }
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
  # attribute {
  #   name = "CreationDate"
  #   type = "S"
  # }
  # attribute {
  #   name = "RequestParams"
  #   type = "S"
  # }
  # attribute {
  #   name = "SpatialResult"
  #   type = "S"
  # }
  # attribute {
  #   name = "TimeToLive"
  #   type = "N"
  # }
  ttl {
    attribute_name = "TimeToLive"
    enabled        = true
  }
}
