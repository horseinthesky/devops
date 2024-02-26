FROM golang:1.22.0-bookworm AS build
WORKDIR /app
COPY . .
RUN : \
  && go mod download \
  && CGO_ENABLED=0 go build -o app .

FROM gcr.io/distroless/static-debian12
COPY --from=build /app/app /
ENTRYPOINT ["/app"]
