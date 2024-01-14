FROM harbor.leryn.top/infra/rust:1.77.0-nightly AS build

WORKDIR /opt/project

COPY .  /opt/project

RUN --mount=type=cache,target=~/.cargo/registry \
    make install

RUN make build

#FROM ubuntu:20.04 AS runtime
FROM debian:buster-slim AS runtime

WORKDIR /

COPY etc/cmdb /etc/cmdb

COPY ci/docker/docker-entrypoint.sh /usr/local/bin

COPY --from=build /opt/project/target/release/cmdb-agent /usr/local/bin

RUN ln -s /usr/local/bin/docker-entrypoint.sh /docker-entrypoint.sh

ENTRYPOINT [ "docker-entrypoint.sh" ]

CMD [ "/usr/local/bin/cmdb-agent" ]
