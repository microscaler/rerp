# One parameterised database-initializer image for every RERP suite.
# The suite migrations and bootstrap script are immutable image content.
ARG POSTGRES_IMAGE=docker.io/bitnamilegacy/postgresql:17.6.0-debian-12-r2
FROM ${POSTGRES_IMAGE}

ARG RERP_SUITE

USER 0
COPY --chmod=0755 microservices/${RERP_SUITE}/scripts/db-init-job.sh /usr/local/bin/rerp-db-init
COPY microservices/${RERP_SUITE}/migrations/ /opt/rerp/suite/migrations/
COPY microservices/${RERP_SUITE}/sql/ /opt/rerp/suite/sql/
USER 1001

ENV RERP_SUITE_DIR=/opt/rerp/suite
ENTRYPOINT ["/usr/local/bin/rerp-db-init"]
