IMAGE=sillycat.axum_web_starter
TAG=latest
NAME=sillycat.axum_web_starter
REPOSITORY=docker.io
STAGE=stage


docker-context:

build: docker-context
	docker build --tag $(REPOSITORY)/$(IMAGE):$(TAG) --build-arg STAGE=${STAGE} .

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

enter-db:
	docker exec -it cockroach cockroach sql --insecure

clean-db:
	docker stop cockroach
	docker rm cockroach

clean:
	docker stop $(NAME)
	docker rm $(NAME)