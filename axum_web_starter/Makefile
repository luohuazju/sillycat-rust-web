IMAGE=sillycat.axum_web_starter
TAG=latest
NAME=sillycat.axum_web_starter
REPOSITORY=docker.io
STAGE=stage


docker-context:

build: docker-context
	docker build --tag $(REPOSITORY)/$(IMAGE):$(TAG) --build-arg STAGE=${STAGE} .

build-release:
	SQLX_OFFLINE=true cargo build --release
	mkdir axum_web_starter-1.0.0
	cp target/release/axum_web_starter ./axum_web_starter-1.0.0/axum_web_starter
	cp .env.template ./axum_web_starter-1.0.0/.env.template
	tar -czvf target/axum_web_starter-1.0.0.tar.gz ./axum_web_starter-1.0.0
	rm -fr ./axum_web_starter-1.0.0

run:
	docker run -d \
	-p 3000:3000 \
	--link cockroach:cockroach \
	-e "DATABASE_URL=$(shell echo $$DATABASE_URL)" \
	--name $(NAME) $(REPOSITORY)/$(IMAGE):$(TAG)

run-db:
	docker run --name cockroach -d \
  		-p 26257:26257 -p 8080:8080 \
  		-v cockroach-data:/cockroach/cockroach-data \
  		cockroachdb/cockroach:latest start-single-node --insecure \
  		--cache=0.25 --max-sql-memory=0.25

run-redis:
	docker run --name redis -d \
		-p 6379:6379 \
		-v redis-data:/data \
		redis:latest redis-server --maxmemory 256mb --maxmemory-policy allkeys-lru


enter-db:
	docker exec -it cockroach cockroach sql --insecure

enter-redis:
	docker exec -it redis redis-cli

clean-db:
	docker stop cockroach
	docker rm cockroach

clean-redis:
	docker stop redis
	docker rm redis

clean:
	docker stop $(NAME)
	docker rm $(NAME)