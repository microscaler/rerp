# GitHub Actions workflows

| Workflow | Runs on | What it does |
| --- | --- | --- |
| `images.yml` | push to main, `v*` tags, dispatch | The octopilot pipeline over `skaffold.yaml`: one build and test leg per suite, side by side, only for the suites a change touches (`only_changed`; a tag compares the previous tag; dispatch with `build_all` for everything). On a tag, each changed suite's service images (`suites/<suite>.txt`) and the chart are published to ghcr.io. |
| `release.yml` | dispatch | Bumps the version, writes release notes, tags and creates the GitHub Release, then dispatches `images.yml` on the tag. |
| `checks.yml` | push to main, pull requests | Repo-wide checks that are not a suite build: conventional commits (PRs), the rerp CLI's tests and docs governance, OpenAPI and port validation, `cargo fmt`. |
| `deploy-website.yml` | push to main under `ui/website/**`, `ui/shared/**` | The brochure site at rerp.microscaler.io, on GitHub Pages. |

A new suite is a new artifact in `skaffold.yaml` (its `BP_RUST_PACKAGES`, `BP_SUITE_IMAGES=suites/<suite>.txt`,
`BP_TEST_COMMAND`, `BP_TEST_LABEL`, and `dependencies.paths` for its OpenAPI specs) plus its map in `suites/`; no
workflow changes. Images stay public: nothing here names a private registry, cloud project or secret.

The `rerp-base` image (`docker/base/`) is built locally by Tilt for development; it is not published.

The `rerp` CLI pins `brrtrouter_tooling` (tooling/pyproject.toml) to the BRRTRouter revision the release crates use;
`pip install -e ../BRRTRouter/tooling` in the same venv swaps in a sibling checkout.
