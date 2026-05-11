set shell := ["bash", "-euo", "pipefail", "-c"]

default:
    @just --list

# Rebuild a full dev environment from a fresh clone.
bootstrap:
    mise install
    cargo fetch --locked
    prek install
