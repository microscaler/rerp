"""Shim CLI package: re-export ``main`` for the ``rerp`` console script."""

from rerp_tooling.cli.main import main

__all__ = ["main"]
