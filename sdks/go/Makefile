.PHONY: gen test

gen:
	protoc --go_out=. --go_opt=paths=source_relative \
	       --go-grpc_out=. --go-grpc_opt=paths=source_relative \
	       -I ../../proto/api \
	       ../../proto/api/eventstore.proto

test:
	go test ./...
