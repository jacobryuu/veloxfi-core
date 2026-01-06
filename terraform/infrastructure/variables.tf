variable "namespace" {
  description = "Kubernetes namespace for veloxfi"
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

variable "create_docker_secret" {
  description = "Create Docker registry secret"
  type        = bool
  default     = false
}

variable "docker_registry" {
  description = "Docker registry URL"
  type        = string
  default     = "docker.io"
  sensitive   = true
}

variable "docker_username" {
  description = "Docker registry username"
  type        = string
  default     = ""
  sensitive   = true
}

variable "docker_password" {
  description = "Docker registry password"
  type        = string
  default     = ""
  sensitive   = true
}

variable "docker_email" {
  description = "Docker registry email"
  type        = string
  default     = "user@example.com"
  sensitive   = true
}

variable "ingress_service_type" {
  description = "Type of ingress controller service (LoadBalancer, NodePort, ClusterIP)"
  type        = string
  default     = "NodePort"
}

variable "nginx_ingress_version" {
  description = "NGINX Ingress Controller Helm chart version"
  type        = string
  default     = "4.8.3"
}

variable "host_network" {
  description = "Enable host network for ingress controller"
  type        = bool
  default     = false
}

variable "dns_policy" {
  description = "DNS policy for ingress controller pods"
  type        = string
  default     = "ClusterFirst"
}

variable "enable_https" {
  description = "Enable HTTPS with Let's Encrypt"
  type        = bool
  default     = false
}

variable "cert_email" {
  description = "Email for Let's Encrypt certificate"
  type        = string
  default     = "admin@veloxfi.local"
}
