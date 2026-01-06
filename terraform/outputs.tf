output "namespace" {
  description = "The namespace created for the application"
  value       = module.infrastructure.namespace
}

output "service_url" {
  description = "URL to access veloxfi-core"
  value       = module.application.service_url
}

output "service_endpoint" {
  description = "Kubernetes service endpoint"
  value       = module.application.service_endpoint
}
