# 1. ConfigMap
resource "kubernetes_config_map" "veloxfi_config" {
  metadata {
    name      = "veloxfi-config"
    namespace = var.namespace
  }

  data = {
    "LOG_LEVEL"   = var.log_level
    "SERVER_HOST" = "0.0.0.0"
    "SERVER_PORT" = "8080"
  }
}

# 2. PersistentVolumeClaim
resource "kubernetes_persistent_volume_claim" "veloxfi_data" {
  metadata {
    name      = "veloxfi-data-pvc"
    namespace = var.namespace
  }

  spec {
    access_modes       = ["ReadWriteOnce"]
    storage_class_name = "standard"

    resources {
      requests = {
        storage = var.data_storage_size
      }
    }
  }
}

# 3. Deployment
resource "kubernetes_deployment" "veloxfi_core" {
  metadata {
    name      = "veloxfi-core"
    namespace = var.namespace
    labels = {
      app = "veloxfi-core"
    }
  }

  wait_for_rollout = false

  spec {
    replicas = var.replicas

    selector {
      match_labels = {
        app = "veloxfi-core"
      }
    }

    template {
      metadata {
        labels = {
          app = "veloxfi-core"
        }
      }

      spec {
        # Image pull secret (if provided)
        dynamic "image_pull_secrets" {
          for_each = var.docker_secret_name != "" ? [1] : []
          content {
            name = var.docker_secret_name
          }
        }

        container {
          name              = "veloxfi-core"
          image             = var.container_image
          image_pull_policy = var.image_pull_policy

          port {
            name           = "http"
            container_port = 8080
            protocol       = "TCP"
          }

          env_from {
            config_map_ref {
              name = kubernetes_config_map.veloxfi_config.metadata.name
            }
          }

          # Environment variables
          env {
            name  = "RUST_LOG"
            value = var.log_level
          }

          env {
            name  = "DATA_DIR"
            value = "/app/data"
          }

          resources {
            requests = {
              cpu    = var.cpu_request
              memory = var.memory_request
            }
            limits = {
              cpu    = var.cpu_limit
              memory = var.memory_limit
            }
          }

          # Liveness probe
          liveness_probe {
            http_get {
              path   = "/api/health"
              port   = 8080
              scheme = "HTTP"
            }
            initial_delay_seconds = 60
            period_seconds        = 10
            timeout_seconds       = 5
            failure_threshold     = 3
          }

          # Readiness probe
          readiness_probe {
            http_get {
              path   = "/api/health"
              port   = 8080
              scheme = "HTTP"
            }
            initial_delay_seconds = 30
            period_seconds        = 5
            timeout_seconds       = 3
            failure_threshold     = 2
          }

          # Volume mount for data
          volume_mount {
            name       = "data"
            mount_path = "/app/data"
          }

          # Security context
          security_context {
            allow_privilege_escalation = false
            read_only_root_filesystem  = false
          }
        }

        volume {
          name = "data"
          persistent_volume_claim {
            claim_name = kubernetes_persistent_volume_claim.veloxfi_data.metadata.name
          }
        }

        restart_policy = "Always"
      }
    }
  }
}

# 4. Service
resource "kubernetes_service" "veloxfi_core" {
  metadata {
    name      = "veloxfi-core-service"
    namespace = var.namespace
    labels = {
      app = "veloxfi-core"
    }
  }

  spec {
    selector = {
      app = "veloxfi-core"
    }

    port {
      name        = "http"
      port        = 80
      target_port = 8080
      protocol    = "TCP"
    }

    type = "ClusterIP"
  }

  depends_on = [kubernetes_deployment.veloxfi_core]
}

# 5. Ingress Rule
resource "kubernetes_ingress_v1" "veloxfi_gateway" {
  metadata {
    name      = "veloxfi-gateway"
    namespace = var.namespace

    annotations = merge(
      {
        "nginx.ingress.kubernetes.io/rewrite-target" = "/"
      },
      var.enable_https ? {
        "cert-manager.io/cluster-issuer" = "letsencrypt-prod"
      } : {}
    )
  }

  spec {
    ingress_class_name = "nginx"

    dynamic "tls" {
      for_each = var.enable_https ? [1] : []
      content {
        hosts       = [var.domain_name]
        secret_name = "veloxfi-tls-secret"
      }
    }

    rule {
      host = var.domain_name
      http {
        path {
          path      = "/"
          path_type = "Prefix"
          backend {
            service {
              name = kubernetes_service.veloxfi_core.metadata.name
              port {
                number = 80
              }
            }
          }
        }
      }
    }
  }

  depends_on = [kubernetes_service.veloxfi_core]
}
