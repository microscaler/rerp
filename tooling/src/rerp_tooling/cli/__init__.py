"""Shim CLI package: expose ``main`` for the ``rerp`` console script."""


def main() -> None:
    """Lazy entry point so ``python -m rerp_tooling.cli.main`` stays warning-free."""
    from rerp_tooling.cli.main import main as run_main

    run_main()

__all__ = ["main"]
