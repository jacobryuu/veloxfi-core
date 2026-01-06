output "namespace" {
  description = "The namespace created for the application"
  value       = kubernetes_namespace.veloxfi.metadata[0].name
}

output "docker_secret_name" {
  description = "The name of the docker registry secret"
  value       = var.create_docker_secret ? kubernetes_secret.docker_registry[0].metadata[0].name : ""
}
