FROM node:latest
WORKDIR /app
COPY react-sucks/package.json /app
RUN yarn install
COPY react-sucks/tsconfig.json /app
COPY react-sucks/tsconfig.node.json /app
COPY react-sucks/src /app/src
COPY react-sucks/testies.ts /app
COPY react-sucks/server.ts /app
COPY react-sucks/Makefile /app
COPY react-sucks/index.html /app
RUN make server
RUN make client
RUN make node-server
ENTRYPOINT ["node"]
CMD ["server.js", "index.html"]
