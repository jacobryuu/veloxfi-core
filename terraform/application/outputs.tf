output "service_url" {
  description = "URL to access veloxfi-core"
  value       = var.enable_https ? "https://${var.domain_name}" : "http://${var.domain_name}"
}

output "service_endpoint" {
  description = "Kubernetes service endpoint"
  value       = "${kubernetes_service.veloxfi_core.metadata[0].name}.${var.namespace}.svc.cluster.local"
}
