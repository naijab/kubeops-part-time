FROM golang:1.17.1 AS base
WORKDIR /app
COPY . .
RUN go mod tidy
RUN CGO_ENABLED=0 GOOS=linux go build -o main ./...

FROM base as test
RUN chmod +x ./run-unit-test.sh
RUN ./run-unit-test.sh

FROM alpine:latest AS production
COPY --from=base /app .
CMD ["./main"]
