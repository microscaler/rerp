# One runtime image definition for every RERP service.
# The build command stages exactly one service into a narrow temporary context.

ARG BASE_IMAGE=rerp-base:latest
FROM ${BASE_IMAGE}

ARG TARGETARCH
ARG RERP_SUITE
ARG RERP_SERVICE
ARG RERP_ARTIFACT_SHA256

LABEL org.opencontainers.image.title="RERP ${RERP_SUITE}/${RERP_SERVICE}"
LABEL io.rerp.suite="${RERP_SUITE}"
LABEL io.rerp.service="${RERP_SERVICE}"
LABEL io.rerp.artifact.sha256="${RERP_ARTIFACT_SHA256}"

COPY ${TARGETARCH}/service /app/service
COPY config/ /app/config/
COPY doc/ /app/doc/
COPY static_site/ /app/static_site/
RUN chmod 0755 /app/service

EXPOSE 8080

ENV RUST_BACKTRACE=1
ENV RUST_LOG=debug

ENTRYPOINT ["/dev-entrypoint.sh", "/app/service", \
    "--spec", "/app/doc/openapi.yaml", \
    "--doc-dir", "/app/doc", \
    "--static-dir", "/app/static_site", \
    "--config", "/app/config/config.yaml"]
