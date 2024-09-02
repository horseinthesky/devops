# 📈 Python (FastAPI) vs Go (Fiber) vs Rust (Axum) simple benchmark

## k8s

Working k8s cluster. Something like [this](https://github.com/horseinthesky/ycloud) setup.

In my case I can do
```
make create
```

In a couple of minutes my Yandex Cloud managed m8s cluster is ready to serve.
```
> k get nodes
NAME                        STATUS   ROLES    AGE   VERSION
cl1jj6hvt1r34cvj50eq-amum   Ready    <none>   67s   v1.28.9
cl1jj6hvt1r34cvj50eq-anem   Ready    <none>   65s   v1.28.9
cl1jj6hvt1r34cvj50eq-ymoz   Ready    <none>   75s   v1.28.9
```

For this benchmark lab to work you need at least 3 nodes with a minimum amount of resources. I use nodes with 4 CPUs and 8 Gigs of RAM each.

## Observability

Prometheus, Grafana, Tempo.

To setup all the monitoring stuff, `cd` to the `terraform` subdir and
```
terraform apply
```

Everything monitoring related is deployed to `monitoring` namespace:

- per-node cadvisor
- grafana
- kube-state-metrics
- prometheus
- prometheus-operator
- tempo

```
> k -n monitoring get pods
NAME                                   READY   STATUS    RESTARTS   AGE
cadvisor-nh2cn                         1/1     Running   0          88s
cadvisor-ts2zw                         1/1     Running   0          88s
cadvisor-vwj95                         1/1     Running   0          88s
grafana-55f8b9848f-mbzr7               1/1     Running   0          103s
kube-state-metrics-db5b89d6d-k9nnh     1/1     Running   0          88s
prometheus-main-0                      2/2     Running   0          72s
prometheus-operator-7888b5785c-n42vf   1/1     Running   0          88s
tempo-0                                1/1     Running   0          105s
```

### Prometheus

To manually connect to Prometheus use the following command:
```
kubectl port-forward svc/prometheus-operated 9090 -n monitoring
```

Then open Prometheus UI in your browser
```
http://localhost:9090
```

### Grafana

Next `cd` back to the project root and upload the Grafana dashboard
```
./dash.sh
```

Grafana deployment has an external loadbalancer. You can find it's IP address with the following command:
```
> k -n monitoring get svc grafana
NAME      TYPE           CLUSTER-IP       EXTERNAL-IP     PORT(S)        AGE
grafana   LoadBalancer   10.255.255.214   <ip_address>    80:31658/TCP   19m
```

Open `<ip_address>` in you browser. Creds are `admin/devops123`. You can open "Performance" dashboard you have just uploaded.

## Apps

### Build

Now you need to build images for the apps and upload them to the container registry (CR):
- `python-app` ([FastAPI](https://fastapi.tiangolo.com/))
- `go-app` ([Fiber](https://gofiber.io/))
- `rust-app` ([Axum](https://github.com/tokio-rs/axum))
- `client` (go-based)

Use the following commands to do so:
```
make
```

I use a hardcoded CR id in my Yandex Cloud environment. Change it to your own in the `Makefile`.

### Deploy

After all the images are uploaded we can finally deploy the pods:
```
k apply -Rf deployment
```

Everything should be up and running:
```
> k get pods
NAME                          READY   STATUS    RESTARTS   AGE
go-app-5fdcfc546d-qswxp       1/1     Running   0          84s
python-app-5465b89b56-5htv7   1/1     Running   0          84s
rust-app-6dc5dd444d-s99g2     1/1     Running   0          83s
```

All the pods have equal amount of CPU and RAM.

## Tests

### Test #1

Now we can run the first test:
```
k apply -Rf 1-test
```

For the sake of truthfulness of the results each client is running on the same node as it's server counterpart:
```
k get pods -o wide
NAME                          READY   STATUS    RESTARTS   AGE     IP              NODE                        NOMINATED NODE   READINESS GATES
go-app-5fdcfc546d-qswxp       1/1     Running   0          3m35s   192.168.129.6   cl1jj6hvt1r34cvj50eq-amum   <none>           <none>
go-app-client-c9nv8           1/1     Running   0          27s     192.168.129.7   cl1jj6hvt1r34cvj50eq-amum   <none>           <none>
python-app-5465b89b56-5htv7   1/1     Running   0          3m35s   192.168.128.8   cl1jj6hvt1r34cvj50eq-ymoz   <none>           <none>
python-app-client-z96rn       1/1     Running   0          27s     192.168.128.9   cl1jj6hvt1r34cvj50eq-ymoz   <none>           <none>
rust-app-6dc5dd444d-s99g2     1/1     Running   0          3m34s   192.168.130.8   cl1jj6hvt1r34cvj50eq-anem   <none>           <none>
rust-app-client-mv59w         1/1     Running   0          27s     192.168.130.9   cl1jj6hvt1r34cvj50eq-anem   <none>           <none>
```

This test measures:
- client side:
    - client request latency (p99)
    - request per second (RPS)
- server side:
    - CPU usage
    - Memory usage

Get back to your Grafana "Performance" dashboard and you see the graphs:

![test1](https://github.com/horseinthesky/devops/blob/main/benchmark/images/test1.png)

#### Cleanup

Stop the test:

```
k delete -Rf 1-test
```

Redeploy observability stack to clean all the data:
```
cd terraform
terraform destroy
terraform apply
```

### Test #2

2nd test:
```
k apply -Rf 2-test
```

This test measures:
- client side:
    - client request latency (p99)
    - request per second (RPS)
- server side:
    - CPU usage
    - Memory usage
    - S3 latency (Average)
    - DB latency (Average)

S3 and DB metrics are just mocks. They are needed to make the Tempo traces in look realistic.

Graphs:

![test2](https://github.com/horseinthesky/devops/blob/main/benchmark/images/test2.png)

#### Cleanup

Stop the test:

```
k delete -Rf 2-test
```

## Telemetry

All requests are measured and reported to `tempo`.

![test2](https://github.com/horseinthesky/devops/blob/main/benchmark/images/tempo.png)
