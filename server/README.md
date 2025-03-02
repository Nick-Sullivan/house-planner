Create a .env with secrets

```bash
AWS_REGION="eu-west-2"
REQUIREMENTS_TABLE_NAME="HousePlanner-Dev-Requirements"
SPATIAL_DISTANCES_TABLE_NAME="HousePlanner-Dev-SpatialDistances"
```

Build and run the API server:

```bash
cargo build
cargo run --bin api
```

Script to openapi.json for automated frontend client:

```bash
cargo build
cargo run --bin openapi
```

View the docs at
http://localhost:3000/swagger/
