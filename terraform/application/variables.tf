variable "namespace" {
  description = "Kubernetes namespace to deploy to (must exist)"
  type        = string
  default     = "veloxfi"
}

variable "kubeconfig_path" {
  description = "Path to kubeconfig file"
  type        = string
  default     = "~/.kube/config-minikube"
}

variable "kubeconfig_context" {
  description = "Kubernetes context to use"
  type        = string
  default     = "minikube"
}

variable "container_image" {
  description = "Docker image for veloxfi-core"
  type        = string
  default     = "veloxfi-core:latest"
}

variable "image_pull_policy" {
  description = "Image pull policy"
  type        = string
  default     = "IfNotPresent"
}

variable "docker_secret_name" {
  description = "Name of the existing Docker registry secret (optional)"
  type        = string
  default     = ""
}

variable "replicas" {
  description = "Number of pod replicas"
  type        = number
  default     = 1
}

variable "cpu_request" {
  description = "CPU request for container"
  type        = string
  default     = "100m"
}

variable "cpu_limit" {
  description = "CPU limit for container"
  type        = string
  default     = "500m"
}

variable "memory_request" {
  description = "Memory request for container"
  type        = string
  default     = "128Mi"
}

variable "memory_limit" {
  description = "Memory limit for container"
  type        = string
  default     = "512Mi"
}

variable "data_storage_size" {
  description = "Size of data storage PVC"
  type        = string
  default     = "1Gi"
}

variable "log_level" {
  description = "Log level (debug, info, warn, error)"
  type        = string
  default     = "info"
}

variable "domain_name" {
  description = "Domain name for Ingress"
  type        = string
  default     = "veloxfi.local"
}

variable "enable_https" {
  description = "Enable HTTPS with Let's Encrypt"
  type        = bool
  default     = false
}
