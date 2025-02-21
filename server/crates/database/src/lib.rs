#[cfg(not(any(feature = "cloud", feature = "local")))]
compile_error!("You must enable either the `cloud` or `local` feature.");
pub mod attribute_value_parser;
pub mod dynamodb_client_cloud;
pub mod dynamodb_client_local;
pub mod dynamodb_client_trait;
pub mod requirement_item;
pub mod spatial_distance_item;
#[cfg(feature = "cloud")]
pub use dynamodb_client_cloud::DynamoDbClient;
#[cfg(feature = "local")]
pub use dynamodb_client_local::DynamoDbClient;
pub use dynamodb_client_trait::IDynamoDbClient;
