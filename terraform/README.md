Terraform folders are seperated into

- browser (frontend infrastructure)
- server (backend infrastructure)
- modules (re-useable components)

The `browser` and `server` folders are split into multiple steps, to support different component lifecycles.

- foundation
  Created once, costly to destroy. For infrastructure such as auth, databases, object stores.

- frame
  Infrastructure that is mostly ephemeral. In-flight data will be impacted. This is for infrastructure such as temporary object storage and queues.

- upload
  Deployment of code builds.

## Running

First, go to `modules/shared_locals/environment.tf` and edit the value to set what environment you'll be deploying to.

```bash
# Initialise & deploy backend.
# This will generate an output file which is used by the browser so it knows the API URL.
cd terraform/server/foundation
terraform init -backend-config "key=house_planner/dev/server/foundation/terraform.tfstate"
terraform apply
cd ../frame
terraform init -backend-config "key=house_planner/dev/server/frame/terraform.tfstate"
terraform apply
cd ../upload
terraform init -backend-config "key=house_planner/dev/server/upload/terraform.tfstate"
terraform apply

# Initialise & deploy frontend.
cd ../browser
cd foundation
terraform init -backend-config "key=house_planner/dev/browser/foundation/terraform.tfstate"
terraform apply
cd ../frame
terraform init -backend-config "key=house_planner/dev/browser/frame/terraform.tfstate"
terraform apply
cd ../upload
terraform init -backend-config "key=house_planner/dev/browser/upload/terraform.tfstate"
terraform apply

```
