resource "helm_release" "tempo" {
  name = "tempo"

  # https://github.com/grafana/helm-charts/releases
  repository       = "https://grafana.github.io/helm-charts"
  chart            = "tempo"
  namespace        = "monitoring"
  version          = "1.7.2"
  create_namespace = true

  values = [file("values/tempo.yaml")]
}
