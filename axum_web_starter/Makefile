run-db:
	docker run --name cockroach -d \
  		-p 26257:26257 -p 8080:8080 \
  		-v cockroach-data:/cockroach/cockroach-data \
  		cockroachdb/cockroach:latest start-single-node --insecure \
  		--cache=0.25 --max-sql-memory=0.25

enter-db:
	docker exec -it cockroach cockroach sql --insecure
