#!/usr/bin/env bash
# Provision RERP's private accounting-document bucket in shared-k8s MinIO.
# The application receives a namespace-local, least-privilege credential; MinIO
# root credentials remain in the platform-owned `data` namespace.
set -euo pipefail

DATA_NAMESPACE="${RERP_OBJECT_STORE_DATA_NAMESPACE:-data}"
APP_NAMESPACE="${RERP_OBJECT_STORE_APP_NAMESPACE:-rerp}"
MINIO_DEPLOYMENT="${RERP_OBJECT_STORE_MINIO_DEPLOYMENT:-minio}"
BUCKET="${RERP_OBJECT_STORE_BUCKET:-rerp-accounting-documents}"
ACCESS_KEY="${RERP_OBJECT_STORE_ACCESS_KEY:-rerp-accounting}"
POLICY_NAME="${RERP_OBJECT_STORE_POLICY:-rerp-accounting-documents}"
SECRET_NAME="${RERP_OBJECT_STORE_SECRET_NAME:-rerp-object-store}"

if [[ ! "${BUCKET}" =~ ^[a-z0-9][a-z0-9.-]{1,61}[a-z0-9]$ ]]; then
  echo "Invalid S3 bucket name: ${BUCKET}" >&2
  exit 1
fi
if [[ ! "${POLICY_NAME}" =~ ^[A-Za-z0-9][A-Za-z0-9._-]{1,127}$ ]]; then
  echo "Invalid MinIO policy name: ${POLICY_NAME}" >&2
  exit 1
fi

if ! kubectl get namespace "${APP_NAMESPACE}" >/dev/null 2>&1; then
  echo "RERP namespace ${APP_NAMESPACE} does not exist; run the cluster bootstrap first." >&2
  exit 1
fi

echo "Waiting for MinIO in namespace ${DATA_NAMESPACE}..."
kubectl rollout status \
  "deployment/${MINIO_DEPLOYMENT}" \
  --namespace "${DATA_NAMESPACE}" \
  --timeout "${RERP_OBJECT_STORE_TIMEOUT:-180s}"

if kubectl get secret "${SECRET_NAME}" --namespace "${APP_NAMESPACE}" >/dev/null 2>&1; then
  ACCESS_KEY="$(kubectl get secret "${SECRET_NAME}" --namespace "${APP_NAMESPACE}" --output jsonpath='{.data.access-key}' | base64 --decode)"
  SECRET_KEY="$(kubectl get secret "${SECRET_NAME}" --namespace "${APP_NAMESPACE}" --output jsonpath='{.data.secret-key}' | base64 --decode)"
else
  SECRET_KEY="$(openssl rand -hex 24)"
fi

# Pass application credentials over stdin, rather than exposing them in the
# kubectl command line. The MinIO container already receives its own root
# credentials from the platform Secret.
printf '%s\n%s\n%s\n%s\n' \
  "${ACCESS_KEY}" "${SECRET_KEY}" "${BUCKET}" "${POLICY_NAME}" | \
kubectl exec --stdin --namespace "${DATA_NAMESPACE}" \
  "deployment/${MINIO_DEPLOYMENT}" -- sh -c '
    set -eu
    IFS= read -r app_access_key
    IFS= read -r app_secret_key
    IFS= read -r bucket
    IFS= read -r policy_name
    alias_name=rerp-bootstrap
    policy_file=/tmp/rerp-accounting-documents-policy.json

    mc alias set "${alias_name}" http://127.0.0.1:9000 \
      "${MINIO_ROOT_USER}" "${MINIO_ROOT_PASSWORD}" >/dev/null
    mc mb --ignore-existing "${alias_name}/${bucket}" >/dev/null

    cat >"${policy_file}" <<POLICY
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": ["s3:GetBucketLocation", "s3:ListBucket"],
      "Resource": ["arn:aws:s3:::${bucket}"]
    },
    {
      "Effect": "Allow",
      "Action": ["s3:GetObject", "s3:PutObject"],
      "Resource": ["arn:aws:s3:::${bucket}/*"]
    }
  ]
}
POLICY
    mc admin policy create "${alias_name}" "${policy_name}" \
      "${policy_file}" >/dev/null
    mc admin user add "${alias_name}" "${app_access_key}" \
      "${app_secret_key}" >/dev/null
    mc admin policy attach "${alias_name}" "${policy_name}" \
      --user "${app_access_key}" >/dev/null
    rm -f "${policy_file}"
  '

kubectl create secret generic "${SECRET_NAME}" \
  --namespace "${APP_NAMESPACE}" \
  --from-literal=access-key="${ACCESS_KEY}" \
  --from-literal=secret-key="${SECRET_KEY}" \
  --dry-run=client \
  --output yaml | kubectl apply --filename - >/dev/null

echo "RERP object store ready: ${BUCKET} (private, application-scoped credentials)."
