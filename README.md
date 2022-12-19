![Arrow Banner](https://github.com/Arrow-air/.github/raw/main/profile/assets/arrow_v2_twitter-banner_neu.png)

# `svc-pricing`
![GitHub release (latest by date including pre-releases)](https://img.shields.io/github/v/release/Arrow-air/svc-pricing?include_prereleases)
![Sanity Checks](https://github.com/arrow-air/svc-pricing/actions/workflows/sanity_checks.yml/badge.svg?branch=main)
![Rust Checks](https://github.com/arrow-air/svc-pricing/actions/workflows/rust_ci.yml/badge.svg?branch=main)
![Python Flake8](https://github.com/arrow-air/svc-pricing/actions/workflows/python_ci.yml/badge.svg?branch=main)
![Arrow DAO Discord](https://img.shields.io/discord/853833144037277726?style=plastic)

## :telescope: Overview

`svc-pricing` provides the ticket price (in USD) for a given trip. The pricing model currently supports three use cases: cargo, rideshare, and charter.

The price supplied and presented to the customers through GUI clients should be the definitive charge to be billed.

Directory:
- `src/`: Source Code and Unit Tests
- `tests/`: Integration Tests
- `docs/`: Module Documentation

## Installation

Install Rust with [Rustup](https://www.rust-lang.org/tools/install).

```bash
# Adds custom pre-commit hooks to .git through cargo-husky dependency
# !! Required for developers !!
cargo test
```

## Make

### Build and test

To ensure consistent build and test outputs, Arrow provides a Docker image with all required software installed to build and test Rust projects.
Using the Makefile, you can easily test and build your code.

```bash
# Build Locally
make rust-build

# Create Deployment Container
make build

# Run Deployment Container
make docker-run

# Stopping Deployment Container
make docker-stop

# Running examples (uses docker compose file)
make rust-example-grpc
```

### Formatting

The Arrow docker image has some formatting tools installed which can fix your code formatting for you.
Using the Makefile, you can easily run the formatters on your code.
Make sure to commit your code before running these commands, as they might not always result in a desired outcome.

```bash
# Format TOML files
make toml-tidy

# Format Rust files
make rust-tidy

# Format Python files
make python-tidy

# Format all at once
make tidy
```

### Spell check

Before being able to commit, cspell will be used as a spelling checker for all files, making sure no unintended spelling errors are found.
You can run cspell yourself by using the following make target:
```bash
make cspell-test
```

If all spelling errors are fixed, but cspell still finds words that are unknown, you can add these words to the local project words list by running the following command:
```bash
make cspell-add-words
```

### Other make targets

There are additional make targets available. You can find all possible targets by running make without a target or use `make help`

## :scroll: Documentation
The following documents are relevant to this service:
- [Concept of Operations](./docs/conops.md)
- [Software Design Document (SDD)](./docs/sdd.md)
- [Interface Control Document (ICD)](./docs/icd.md)

## :busts_in_silhouette: Arrow DAO
Learn more about us:
- [Website](https://www.arrowair.com/)
- [Arrow Docs](https://www.arrowair.com/docs/intro)
- [Discord](https://discord.com/invite/arrow)

## LICENSE Notice

Please note that svc-pricing is under BUSL license until the Change Date, currently the earlier of two years from the release date. Exceptions to the license may be specified by Arrow Governance via Additional Use Grants, which can, for example, allow svc-pricing to be deployed for certain production uses. Please reach out to Arrow DAO to request a DAO vote for exceptions to the license, or to move up the Change Date.

## :exclamation: Treatment of `Cargo.lock`
If you are building a non-end product like a library, include `Cargo.lock` in `.gitignore`.

If you are building an end product like a command line tool, check `Cargo.lock` to the git. 

Read more about it [here](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html).
