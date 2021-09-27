install:
	go mod tidy

dev:
	air 

start:
	go run ./main.go

test:
	go test ./...

test-docker:
	docker build -t go-service:test --target test .

build-docker:
	docker build -t go-service --target production .
	@echo "build complete docker image name: go-service"