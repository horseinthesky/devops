resource "helm_release" "grafana" {
  name = "grafana"

  # https://github.com/grafana/helm-charts/releases
  repository       = "https://grafana.github.io/helm-charts"
  chart            = "grafana"
  namespace        = "monitoring"
  version          = "7.3.1"
  create_namespace = true

  values = [file("values/grafana.yaml")]
}
