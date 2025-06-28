# dx serve --platform web 

# Create resource group to wrap everything in 
az group create --name test-rust-webapp --location australiaeast

# Create container registry to upload prd image 
az acr create --resource-group test-rust-webapp --name myregistry2345234 --sku Basic

# COnnecting to acr 
az acr login --name myregistry2345234

# Acr requires admin account set for creating ACI
az acr update -n <acrName> --admin-enabled true

# Build image from dockerfile, store in acr registry 
docker build -t myregistry2345234.azurecr.io/dioxus-app:latest -platform=linux/amd64 .

# Push to ACR 
docker push myregistry2345234.azurecr.io/dioxus-app:latest

# Create container instance - this didn't build public ip for me last time, not sure if it works... 
az container create \
	--resource-group test-rust-webapp \
	--name test-rust3 \
	--image myregistry2345234.azurecr.io/dioxus-app:latest \
	--cpu 0.1 \
	--memory 0.1 \
	--ports 80 8080 443 \
	--sku Standard \
	--registry-username myregistry2345234 \
	--registry-password <pass> \
	--os-type Linux
	--ip-address Public 

# Clean up 
az group delete --name test-rust-webapp


