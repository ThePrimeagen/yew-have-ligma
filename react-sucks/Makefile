all: client server node-server testies
docker: server testies

client:
	npm run build:client

server:
	npm run build:server

node-server:
	tsc server.ts

testies:
	./node_modules/.bin/tsc testies.ts
