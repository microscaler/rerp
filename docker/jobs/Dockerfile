# One parameterised role/database bootstrap image for every RERP suite.
# Application migrations are intentionally excluded and remain in the Tilt
# development cycle.
ARG POSTGRES_IMAGE=docker.io/bitnamilegacy/postgresql:17.6.0-debian-12-r2
FROM ${POSTGRES_IMAGE}

ARG RERP_SUITE

USER 0
COPY --chmod=0755 microservices/${RERP_SUITE}/scripts/db-init-job.sh /usr/local/bin/rerp-db-init
USER 1001

ENTRYPOINT ["/usr/local/bin/rerp-db-init"]
