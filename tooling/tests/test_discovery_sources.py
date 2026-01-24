"""Tests for discovery.sources: helm, kind, tiltfile, bff-suite, openapi localhost."""

from rerp_tooling.discovery.sources import (
    discover_bff_suite_config,
    discover_helm,
    discover_kind_host_ports,
    discover_openapi_bff_localhost,
    discover_openapi_suite_microservice_localhost,
    discover_tiltfile,
)


def test_discover_helm_missing_dir(tmp_path):
    got = discover_helm(tmp_path)
    assert got == {}


def test_discover_helm_one_values(tmp_path):
    d = tmp_path / "helm" / "rerp-microservice" / "values"
    d.mkdir(parents=True)
    (d / "a.yaml").write_text("service:\n  name: a\n  port: 8001\n")
    got = discover_helm(tmp_path)
    assert got == {"a": 8001}


def test_discover_helm_name_from_stem(tmp_path):
    d = tmp_path / "helm" / "rerp-microservice" / "values"
    d.mkdir(parents=True)
    (d / "svc.yaml").write_text("service:\n  port: 8002\n")
    got = discover_helm(tmp_path)
    assert got == {"svc": 8002}


def test_discover_kind_host_ports_missing(tmp_path):
    got = discover_kind_host_ports(tmp_path)
    assert got == []


def test_discover_kind_host_ports(tmp_path):
    (tmp_path / "kind-config.yaml").write_text("""
nodes:
  - role: control-plane
    extraPortMappings:
      - containerPort: 80
        hostPort: 9000
      - containerPort: 443
        hostPort: 9443
""")
    got = discover_kind_host_ports(tmp_path)
    assert (9000, "80") in got
    assert (9443, "443") in got


def test_discover_tiltfile_missing(tmp_path):
    got = discover_tiltfile(tmp_path)
    assert got == {}


def test_discover_tiltfile_ports_block(tmp_path):
    (tmp_path / "Tiltfile").write_text("""
ports = {
  'svc1': '8001',
  'svc2': '8002',
}
""")
    got = discover_tiltfile(tmp_path)
    assert got == {"svc1": 8001, "svc2": 8002}


def test_discover_bff_suite_config_empty(tmp_path):
    got = discover_bff_suite_config(tmp_path)
    assert got == {}


def test_discover_bff_suite_config_one_suite(tmp_path):
    # suites_with_bff looks for openapi/{suite}/bff-suite-config.yaml
    (tmp_path / "openapi" / "acc").mkdir(parents=True)
    (tmp_path / "openapi" / "acc" / "bff-suite-config.yaml").write_text("""
services:
  bff:
    port: 8010
  gl:
    port: 8001
""")
    got = discover_bff_suite_config(tmp_path)
    assert "bff" in got and got["bff"] == 8010
    assert "gl" in got and got["gl"] == 8001


def test_discover_openapi_bff_localhost_empty(tmp_path):
    got = discover_openapi_bff_localhost(tmp_path)
    assert got == {}


def test_discover_openapi_bff_localhost(tmp_path):
    # iter_bffs needs bff_service_name in bff-suite-config; openapi_bff_path = openapi/{suite}/openapi_bff.yaml
    (tmp_path / "openapi" / "acc").mkdir(parents=True)
    (tmp_path / "openapi" / "acc" / "bff-suite-config.yaml").write_text("""
bff_service_name: bff
services:
  bff:
    port: 8010
""")
    (tmp_path / "openapi" / "acc" / "openapi_bff.yaml").write_text("""
servers:
  - url: http://localhost:8010
    description: local
""")
    got = discover_openapi_bff_localhost(tmp_path)
    assert "bff" in got
    assert got["bff"] == (8010, "acc")


def test_discover_openapi_suite_microservice_localhost_empty(tmp_path):
    got = discover_openapi_suite_microservice_localhost(tmp_path)
    assert got == {}


def test_discover_openapi_suite_microservice_localhost(tmp_path):
    (tmp_path / "openapi" / "s" / "svc").mkdir(parents=True)
    (tmp_path / "openapi" / "s" / "svc" / "openapi.yaml").write_text("""
servers:
  - url: http://localhost:8001/api/v1/s/svc
""")
    got = discover_openapi_suite_microservice_localhost(tmp_path)
    assert "svc" in got
    assert got["svc"] == ("s", 8001)
