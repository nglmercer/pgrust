#!/usr/bin/env python3
"""Inventory and enforce workspace ownership of external Cargo dependencies.

Internal path dependencies are intentionally out of scope: they cannot create
registry duplicates. External dependency versions belong in the root
[workspace.dependencies] table; member crates should inherit them with
`workspace = true` and only add `optional` or extra features locally.
"""

from __future__ import annotations

import argparse
import collections
import pathlib
import sys
import tomllib
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
CRATES = ROOT / "crates"
SECTIONS = ("dependencies", "dev-dependencies", "build-dependencies")

# Local exceptions need a reason in this table. Keep this list empty unless the
# shape genuinely cannot be represented by workspace inheritance (notably a
# no-default-features consumer of a dependency whose workspace baseline keeps
# default features for the rest of the tree).
EXCEPTIONS: dict[tuple[str, str], str] = {
    (
        "crates/_support/types/types_storage/Cargo.toml",
        "libc",
    ): "no-default-features consumer; the shared libc baseline keeps default features for the rest of the workspace",
}


def dep_tables(data: dict[str, Any]):
    for section in SECTIONS:
        yield section, data.get(section, {})
    for target, target_data in data.get("target", {}).items():
        for section in SECTIONS:
            yield f"target.{target}.{section}", target_data.get(section, {})


def scan() -> tuple[dict[str, list[dict[str, Any]]], list[dict[str, Any]]]:
    inventory: dict[str, list[dict[str, Any]]] = collections.defaultdict(list)
    violations: list[dict[str, Any]] = []
    for manifest in sorted(CRATES.rglob("Cargo.toml")):
        rel = manifest.relative_to(ROOT).as_posix()
        try:
            data = tomllib.loads(manifest.read_text())
        except (OSError, tomllib.TOMLDecodeError) as exc:
            print(f"{rel}: cannot parse: {exc}", file=sys.stderr)
            continue
        for section, table in dep_tables(data):
            for name, value in table.items():
                if isinstance(value, str):
                    spec: dict[str, Any] = {"version": value}
                elif isinstance(value, dict):
                    spec = dict(value)
                else:
                    continue
                if "path" in spec or ("version" not in spec and spec.get("workspace") is not True):
                    continue
                row = {
                    "manifest": rel,
                    "section": section,
                    "name": name,
                    "spec": spec,
                    "workspace": spec.get("workspace") is True,
                }
                inventory[name].append(row)
                if not row["workspace"] and (rel, name) not in EXCEPTIONS:
                    violations.append(row)
    return inventory, violations


def print_inventory(inventory: dict[str, list[dict[str, Any]]]) -> None:
    print(f"external dependency names: {len(inventory)}")
    print(f"external dependency entries: {sum(len(v) for v in inventory.values())}")
    for name, rows in sorted(inventory.items(), key=lambda kv: (-len(kv[1]), kv[0])):
        versions = sorted({str(row["spec"].get("version", "workspace")) for row in rows})
        inherited = sum(row["workspace"] for row in rows)
        print(f"{name}: entries={len(rows)} inherited={inherited} versions={', '.join(versions)}")


def print_violations(violations: list[dict[str, Any]]) -> None:
    for row in violations:
        spec = row["spec"]
        detail = spec if isinstance(spec, dict) else {"version": spec}
        print(f"{row['manifest']}: [{row['section']}] {row['name']} uses local spec {detail!r}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--inventory", action="store_true", help="print the external dependency inventory")
    parser.add_argument("--enforce", action="store_true", help="fail when a member uses a non-workspace external dependency")
    args = parser.parse_args()

    inventory, violations = scan()
    if args.inventory or not args.enforce:
        print_inventory(inventory)
    if violations and (args.inventory or not args.enforce):
        print(f"\nnon-workspace entries outside exceptions: {len(violations)}")
        print_violations(violations)
    if args.enforce and violations:
        print_violations(violations)
        print(f"\nerror: {len(violations)} external dependency entries do not use workspace inheritance", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
