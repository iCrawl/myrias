FROM alpine:latest AS build

RUN apk update && apk add g++
COPY bf.cpp .
RUN g++ bf.cpp -o bf

FROM alpine:latest
LABEL author="1Computer1"

RUN apk update && apk add libstdc++
COPY --from=build bf /usr/local/bin/
COPY run.sh /var/run/
