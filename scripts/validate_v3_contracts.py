#!/usr/bin/env python3
"""Validate DemoSwarm v3 manifests, shared contracts, schemas, and fixtures."""

from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator, FormatChecker, ValidationError
from referencing import Registry, Resource

ROOT = Path(__file__).resolve().parents[1]
FLOW_IDS = ["signal", "plan", "build", "review", "gate", "deploy", "wisdom"]
MATURITY = {"native", "preview", "experimental", "unsupported", "unknown"}
INSTALL_MODES = {"native", "managed-files", "local", "bundle"}


def fail(message: str) -> None:
    print(f"ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        fail(f"{path.relative_to(ROOT)}: {error}")


def load_toml(path: Path) -> dict[str, Any]:
    try:
        with path.open("rb") as handle:
            return tomllib.load(handle)
    except (OSError, tomllib.TOMLDecodeError) as error:
        fail(f"{path.relative_to(ROOT)}: {error}")


def validate_manifest() -> dict[str, Any]:
    manifest = load_toml(ROOT / "demoswarm-pack.toml")
    if manifest.get("schema_version") != 1:
        fail("demoswarm-pack.toml: schema_version must be 1")
    if manifest.get("id") != "demoswarm":
        fail("demoswarm-pack.toml: id must be demoswarm")

    flows = manifest.get("flows", [])
    ids = [flow.get("id") for flow in flows]
    ordinals = [flow.get("ordinal") for flow in flows]
    if ids != FLOW_IDS or ordinals != list(range(1, 8)):
        fail(f"demoswarm-pack.toml: expected ordered flows {FLOW_IDS}, got {ids}")

    for flow in flows:
        for key in ("contract", "receipt_schema"):
            path = ROOT / flow[key]
            if not path.is_file():
                fail(f"demoswarm-pack.toml: declared {key} does not exist: {flow[key]}")
        contract = load_toml(ROOT / flow["contract"])
        if contract.get("id") != flow["id"] or contract.get("ordinal") != flow["ordinal"]:
            fail(f"{flow['contract']}: identity does not match pack manifest")

    capability_contract = load_toml(ROOT / "contracts/capabilities.toml")
    declared_capabilities = {entry["id"] for entry in capability_contract["capabilities"]}
    if set(capability_contract.get("maturity", [])) != MATURITY:
        fail("contracts/capabilities.toml: maturity vocabulary drift")

    adapters = manifest.get("adapters", [])
    adapter_ids = [adapter.get("id") for adapter in adapters]
    if len(adapter_ids) != len(set(adapter_ids)):
        fail("demoswarm-pack.toml: adapter IDs must be unique")
    for adapter in adapters:
        if adapter.get("support") not in MATURITY:
            fail(f"adapter {adapter.get('id')}: invalid support maturity")
        if not set(adapter.get("install_modes", [])).issubset(INSTALL_MODES):
            fail(f"adapter {adapter.get('id')}: invalid install mode")
        adapter_path = ROOT / adapter["path"]
        if not adapter_path.is_dir():
            fail(f"adapter {adapter.get('id')}: declared path does not exist")
        capabilities = adapter.get("capabilities", {})
        if set(capabilities) != declared_capabilities:
            missing = sorted(declared_capabilities - set(capabilities))
            extra = sorted(set(capabilities) - declared_capabilities)
            fail(f"adapter {adapter.get('id')}: capability mismatch; missing={missing} extra={extra}")
        invalid = {key: value for key, value in capabilities.items() if value not in MATURITY}
        if invalid:
            fail(f"adapter {adapter.get('id')}: invalid capability maturity {invalid}")

    return manifest


def schema_registry() -> tuple[dict[str, Any], Registry]:
    schemas: dict[str, Any] = {}
    registry = Registry()
    for path in sorted((ROOT / "schemas").glob("*.schema.json")):
        schema = load_json(path)
        Draft202012Validator.check_schema(schema)
        schema_id = schema.get("$id")
        if not schema_id:
            fail(f"{path.relative_to(ROOT)}: schema has no $id")
        if schema_id in schemas:
            fail(f"duplicate schema $id: {schema_id}")
        schemas[schema_id] = schema
        registry = registry.with_resource(schema_id, Resource.from_contents(schema))
    return schemas, registry


def validate_instance(
    schemas: dict[str, Any], registry: Registry, schema_id: str, instance: Any, label: str
) -> None:
    schema = schemas[schema_id]
    validator = Draft202012Validator(
        schema,
        registry=registry,
        format_checker=FormatChecker(),
    )
    errors = sorted(validator.iter_errors(instance), key=lambda error: list(error.path))
    if errors:
        rendered = "; ".join(error.message for error in errors)
        fail(f"{label}: {rendered}")


def validate_fixtures(schemas: dict[str, Any], registry: Registry) -> tuple[int, int]:
    valid_count = 0
    invalid_count = 0

    config = load_toml(ROOT / "fixtures/contracts/config.valid.toml")
    validate_instance(schemas, registry, "urn:demoswarm:schema:config:1", config, "config.valid.toml")
    valid_count += 1

    run = load_json(ROOT / "fixtures/contracts/run.valid.json")
    validate_instance(schemas, registry, "urn:demoswarm:schema:run:1", run, "run.valid.json")
    valid_count += 1

    for path in sorted((ROOT / "fixtures/contracts").glob("receipt*.valid.json")):
        instance = load_json(path)
        schema_id = f"urn:demoswarm:schema:receipt:{instance['flow']}:2"
        validate_instance(schemas, registry, schema_id, instance, path.name)
        valid_count += 1

    for path in sorted((ROOT / "fixtures/contracts").glob("receipt*.invalid.json")):
        instance = load_json(path)
        schema_id = f"urn:demoswarm:schema:receipt:{instance['flow']}:2"
        validator = Draft202012Validator(
            schemas[schema_id],
            registry=registry,
            format_checker=FormatChecker(),
        )
        try:
            validator.validate(instance)
        except ValidationError:
            invalid_count += 1
        else:
            fail(f"{path.name}: invalid fixture unexpectedly passed")

    return valid_count, invalid_count


def main() -> int:
    manifest = validate_manifest()
    schemas, registry = schema_registry()
    valid_count, invalid_count = validate_fixtures(schemas, registry)
    print(
        "DemoSwarm v3 contracts valid: "
        f"{len(manifest['flows'])} flows, {len(manifest['adapters'])} adapters, "
        f"{len(schemas)} schemas, {valid_count} valid fixtures, "
        f"{invalid_count} rejected fixtures"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
