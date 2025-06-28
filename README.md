# Rust dioxus hot dog example 

Simple web app build following dioxus quick start


dx serve --platform web 

az group create --name test-rust-webapp --location australiaeast

az acr create --resource-group test-rust-webapp --name myregistry2345234 --sku Basic

az acr login --name myregistry2345234

az acr update -n <acrName> --admin-enabled true

docker build -t myregistry2345234.azurecr.io/dioxus-app:latest . 

docker build -t myregistry2345234.azurecr.io/dioxus-app:latest -platform=linux/amd64 .

docker push myregistry2345234.azurecr.io/dioxus-app:latest

az group delete --name myResourceGroup


az container create --resource-group test-rust-webapp --name test-rust --image myregistry2345234.azurecr.io/dioxus-app:latest --cpu 1 --moemory 1 :

az container create --resource-group myResourceGroup --name aci-tutorial-app --image <acrLoginServer>/aci-tutorial-app:v1 --cpu 1 --memory 1 --registry-login-server <acrLoginServer> --registry-username <service-principal-ID> --registry-password <service-principal-password> --ip-address Public --dns-name-label <aciDnsLabel> --ports 80


