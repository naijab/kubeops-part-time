install:
	yarn

dev:
	yarn dev

start:
	yarn start

test:
	yarn test

test-docker:
	docker build -t node-service:test --target test .

build-docker:
	docker build -t node-service --target production .
	@echo "build complete docker image name: node-service"