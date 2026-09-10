#!/usr/bin/env python3
"""Drive an AEP candidate's public interface; retain proof outside scratch state."""

import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    summary = {"binary": str(binary), "binary_sha256": digest(binary), "surfaces": [], "passed": False}
    counter = 0

    def run(argv, cwd):
        nonlocal counter
        counter += 1
        command = {"argv": [str(x) for x in argv], "cwd": str(cwd)}
        try:
            result = subprocess.run(command["argv"], cwd=cwd, text=True, capture_output=True, timeout=60)
        except subprocess.TimeoutExpired as error:
            def decoded(value):
                return value.decode(errors="replace") if isinstance(value, bytes) else value or ""
            command.update(timed_out=True, exit_code=None, stdout=decoded(error.stdout), stderr=decoded(error.stderr))
            raise RuntimeError(f"Command {counter} timed out after 60 seconds") from error
        except OSError as error:
            command.update(exit_code=None, execution_error=str(error))
            raise
        else:
            command.update(exit_code=result.returncode, stdout=result.stdout, stderr=result.stderr)
        finally:
            (output / f"{counter:02d}-command.json").write_text(json.dumps(command, indent=2) + "\n")
        if result.returncode:
            raise RuntimeError(f"Command {counter} failed: {result.stderr or result.stdout}")
        return result.stdout

    def aep(root, *argv):
        response = json.loads(run([binary, "--root", root, "--json", *argv], root))
        assert response["ok"] and response["exit_code"] == 0, response
        return response["data"]

    fixture = None
    try:
        with tempfile.TemporaryDirectory(prefix="aep-preview-fixture-") as scratch:
            fixture = Path(scratch)
            summary["version"] = run([binary, "--version"], fixture).strip()
            assert "preview" in summary["version"], "Select the preview binary"
            run(["git", "init", "-q", "-b", "main"], fixture)
            run(["git", "-C", fixture, "config", "user.name", "AEP preview fixture"], fixture)
            run(["git", "-C", fixture, "config", "user.email", "fixture@example.invalid"], fixture)
            sources = {
                "AGENTS.md": "# Project\n\nAEP v4.1.0. Use installed aep-* skills.\nRead project-convention/README.md.\n",
                "product-context.yaml": json.dumps({"stories": [{"id": "OLD-1", "title": "Existing work", "status": "pending"}]}),
                "project-convention/README.md": "# Rules\n\nPreserve UTF-8 output.\n",
                "lessons-learned/runtime.md": "Check observable command output.\n",
                ".agents/skills/aep-build/SKILL.md": "---\nname: aep-build\ndescription: Legacy fixture.\n---\nLegacy skill bytes.\n",
            }
            for relative, content in sources.items():
                dest = fixture / relative
                dest.parent.mkdir(parents=True, exist_ok=True)
                dest.write_text(content)
            # AGENTS is the selected route; all other legacy files stay byte-identical.
            legacy_hashes = {p: digest(fixture / p) for p in sources if p != "AGENTS.md"}
            summary["legacy_hashes"] = legacy_hashes
            run(["git", "-C", fixture, "add", "."], fixture)
            run(["git", "-C", fixture, "-c", "core.hooksPath=/dev/null", "-c", "commit.gpgsign=false", "commit", "-qm", "legacy fixture"], fixture)
            catalog = aep(fixture, "--skill")
            assert len(catalog["skills"]) == 8, catalog
            summary["guidance"] = catalog["metadata"]
            for argv in [("project",), ("design", "--ref", "prototype"), ("validate", "--ref", "self-verification")]:
                aep(fixture, "--skill", *argv)
            summary["surfaces"].append("embedded guidance outside source checkout")
            aep(fixture, "init")
            assert "AEP default: v4" in (fixture / "AGENTS.md").read_text()
            for relative, expected in legacy_hashes.items():
                assert digest(fixture / relative) == expected, relative
            summary["surfaces"].append("v4 initialization and source preservation")
            run(["git", "-C", fixture, "add", "."], fixture)
            run(["git", "-C", fixture, "-c", "core.hooksPath=/dev/null", "-c", "commit.gpgsign=false", "commit", "-qm", "preview setup"], fixture)
            plan = output / "migration-plan.json"
            aep(fixture, "migrate", "plan", "--output", plan)
            aep(fixture, "migrate", "apply", "--plan", plan)
            aep(fixture, "migrate", "verify")
            aep(fixture, "migrate", "apply", "--plan", plan)
            assert "AEP default: v5" in (fixture / "AGENTS.md").read_text()
            summary["surfaces"].append("migration, verification, and repeated apply")
            before_reads = {str(p.relative_to(fixture)): digest(p) for p in fixture.rglob("*") if p.is_file() and ".git" not in p.relative_to(fixture).parts}
            for argv in [("status",), ("query",), ("context", "OLD-1"), ("timeline", "OLD-1"), ("check",)]:
                aep(fixture, *argv)
            after_reads = {str(p.relative_to(fixture)): digest(p) for p in fixture.rglob("*") if p.is_file() and ".git" not in p.relative_to(fixture).parts}
            assert before_reads == after_reads, "Inspection mutated project files"
            story = output / "native-story.json"
            story.write_text(json.dumps({"kind": "story", "id": "NEW-1", "title": "Preview native work"}))
            aep(fixture, "story", "new", "--file", story)
            aep(fixture, "story", "show", "NEW-1")
            assert any("NEW-1" in p.read_text() for p in (fixture / "project-ledger").rglob("*.yaml"))
            for relative, expected in legacy_hashes.items():
                assert digest(fixture / relative) == expected, relative
            summary["surfaces"].append("read-only inspection and independent native writes")
            summary["passed"] = True
    except Exception as error:
        summary["error"] = str(error)
        raise
    finally:
        summary["fixture_removed"] = fixture is not None and not fixture.exists()
        (output / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
        print(output / "summary.json")


if __name__ == "__main__":
    main()
