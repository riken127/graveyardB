.PHONY: gen test lint

lint:
	golangci-lint run

gen:
	mkdir -p proto
	protoc --go_out=proto --go_opt=paths=source_relative \
	       --go-grpc_out=proto --go-grpc_opt=paths=source_relative \
	       -I ../../proto/api \
	       ../../proto/api/eventstore.proto

test:
	go test ./...
