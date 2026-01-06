terraform {
  required_version = ">= 1.0"

  required_providers {
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "~> 2.23.0"
    }
    helm = {
      source  = "hashicorp/helm"
      version = "~> 2.11.0"
    }
    null = {
      source  = "hashicorp/null"
      version = "~> 3.2.0"
    }
  }
}

provider "kubernetes" {
  config_path    = var.kubeconfig_path
  config_context = var.kubeconfig_context
}

provider "helm" {
  kubernetes {
    config_path    = var.kubeconfig_path
    config_context = var.kubeconfig_context
  }
}

provider "null" {
}

module "infrastructure" {
  source = "./infrastructure"

  namespace             = var.namespace
  kubeconfig_path       = var.kubeconfig_path
  kubeconfig_context    = var.kubeconfig_context
  create_docker_secret  = var.create_docker_secret
  docker_registry       = var.docker_registry
  docker_username       = var.docker_username
  docker_password       = var.docker_password
  docker_email          = var.docker_email
  ingress_service_type  = var.ingress_service_type
  nginx_ingress_version = var.nginx_ingress_version
  host_network          = var.host_network
  dns_policy            = var.dns_policy
  enable_https          = var.enable_https
  cert_email            = var.cert_email
}

module "application" {
  source = "./application"

  namespace          = module.infrastructure.namespace
  kubeconfig_path    = var.kubeconfig_path
  kubeconfig_context = var.kubeconfig_context
  container_image    = var.container_image
  image_pull_policy  = var.image_pull_policy
  docker_secret_name = module.infrastructure.docker_secret_name
  replicas           = var.replicas
  cpu_request        = var.cpu_request
  cpu_limit          = var.cpu_limit
  memory_request     = var.memory_request
  memory_limit       = var.memory_limit
  data_storage_size  = var.data_storage_size
  log_level          = var.log_level
  domain_name        = var.domain_name
  enable_https       = var.enable_https

  depends_on = [module.infrastructure]
}
