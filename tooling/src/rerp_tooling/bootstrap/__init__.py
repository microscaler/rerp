"""Bootstrap: microservice from OpenAPI (brrtrouter-gen, Dockerfile, Cargo, Tiltfile)."""

from .microservice import run_bootstrap_microservice

__all__ = ["run_bootstrap_microservice"]
