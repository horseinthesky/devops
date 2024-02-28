ip=$(kubectl -n monitoring get svc | grep grafana | awk '{print $4}')
user=$(grep User terraform/values/grafana.yaml | awk '{print $NF}')
pass=$(grep Pass terraform/values/grafana.yaml | awk '{print $NF}')

curl -X POST -H "Content-Type: application/json" -u "$user:$pass" -d @dashboard.json http://"$ip"/api/dashboards/db
