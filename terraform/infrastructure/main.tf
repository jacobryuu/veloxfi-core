# 1. Namespace Definition
resource "kubernetes_namespace" "veloxfi" {
  metadata {
    name = var.namespace
    labels = {
      name       = var.namespace
      managed-by = "terraform-infrastructure"
    }
  }
}

# 2. Docker Registry Secret (Optional)
resource "kubernetes_secret" "docker_registry" {
  count = var.create_docker_secret ? 1 : 0

  metadata {
    name      = "docker-registry-secret"
    namespace = kubernetes_namespace.veloxfi.metadata.name
  }

  type = "kubernetes.io/dockerconfigjson"

  data = {
    ".dockerconfigjson" = jsonencode({
      auths = {
        (var.docker_registry) = {
          username = var.docker_username
          password = var.docker_password
          email    = var.docker_email
          auth     = base64encode("${var.docker_username}:${var.docker_password}")
        }
      }
    })
  }
}

# 3. NGINX Ingress Controller (Helm)
# Clean up existing NGINX Ingress if it conflicts
resource "null_resource" "cleanup_nginx" {
  provisioner "local-exec" {
    command = "kubectl delete ingressclass nginx --ignore-not-found=true 2>/dev/null || true"
  }
}

resource "helm_release" "nginx_ingress" {
  name             = "nginx-ingress"
  repository       = "https://kubernetes.github.io/ingress-nginx"
  chart            = "ingress-nginx"
  namespace        = "ingress-nginx"
  create_namespace = true
  version          = var.nginx_ingress_version
  force_update     = true
  recreate_pods    = true

  values = [
    yamlencode({
      controller = {
        service = {
          type = var.ingress_service_type
        }
        hostNetwork = var.host_network
        dnsPolicy   = var.dns_policy
      }
    })
  ]

  depends_on = [null_resource.cleanup_nginx]
}

# 4. CertManager Issuer (Optional)
resource "kubernetes_manifest" "letsencrypt_issuer" {
  count = var.enable_https ? 1 : 0

  manifest = {
    apiVersion = "cert-manager.io/v1"
    kind       = "ClusterIssuer"
    metadata = {
      name = "letsencrypt-prod"
    }
    spec = {
      acme = {
        server = "https://acme-v02.api.letsencrypt.org/directory"
        email  = var.cert_email
        privateKeySecretRef = {
          name = "letsencrypt-prod"
        }
        solvers = [
          {
            http01 = {
              ingress = {
                class = "nginx"
              }
            }
          }
        ]
      }
    }
  }
}
