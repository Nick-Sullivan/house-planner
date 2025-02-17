
Terraform folders are seperated into 
- browser (frontend infrastructure) 
- server (backend infrastructure)
- modules (re-useable components)

The `browser` and `server` folders are split into multiple steps, to support different component lifecycles.
- foundation 
    Created once, costly to destroy. For infrastructure such as auth, databases, object stores.

- frame
    Infrastructure that is ephemeral apart from in-flight data, such as temporary object storage and queues.

- upload
    Deployment of code builds.


```bash
# Initialise & deploy backend (requires server to have been built).
# This will generate an output file which is used by the browser so it knows the API URL.
cd terraform/server
terraform init -backend-config "key=house_planner/dev/server/terraform.tfstate"
terraform apply

# Initialise & deploy (requires browser to have been built)
cd ../browser
cd foundation
terraform init -backend-config "key=house_planner/dev/browser/foundation/terraform.tfstate"
terraform apply
cd ../infrastructure
terraform init -backend-config "key=house_planner/dev/browser/infrastructure/terraform.tfstate"
terraform apply
cd ../upload
terraform init -backend-config "key=house_planner/dev/browser/upload/terraform.tfstate"
terraform apply

```