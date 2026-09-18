#!/usr/bin/env python3
"""Isolated GitHub adapter fixture. It never contacts a provider."""
import json
import os
import pathlib
import re
import subprocess
import sys

args = sys.argv[1:]
state = pathlib.Path(os.environ["AEP_TEST_STATE"])
scenario = os.environ["AEP_TEST_SCENARIO"]
head = os.environ["AEP_TEST_HEAD"]
base = os.environ["AEP_TEST_BASE"]

if args[:2] == ["pr", "list"]:
    print("[]")
elif args[:2] == ["pr", "create"]:
    if scenario == "pr-edit":
        path = pathlib.Path(os.environ["AEP_TEST_STORY"])
        path.write_text(re.sub(r"^description:.*$", "description: Concurrent clarification", path.read_text(), flags=re.M))
    print("https://example.invalid/pull/1")
elif args[:2] == ["pr", "view"]:
    print(state.read_text() if state.exists() else json.dumps({
        "state": "OPEN", "headRefOid": head, "baseRefOid": base,
        "mergeStateStatus": "CLEAN", "url": "https://example.invalid/pull/1",
    }))
elif args[:2] == ["pr", "merge"]:
    root = pathlib.Path(os.environ["AEP_TEST_PROVIDER"])
    def git(*argv):
        return subprocess.check_output(["git", "-C", str(root), *argv], stderr=subprocess.PIPE).decode().strip()
    if scenario == "tree-drift":
        (root / "provider-change.txt").write_text("Concurrent provider base change\n")
        git("add", ".")
        git("commit", "-qm", "Advance provider base")
    git("merge", "--no-edit", "-m", "Fixture integration", head)
    state.write_text(json.dumps({"state": "MERGED", "headRefOid": head,
        "mergeCommit": {"oid": git("rev-parse", "HEAD")}, "url": "https://example.invalid/pull/1"}))
    if scenario == "lost-merge":
        print("Simulated lost provider response", file=sys.stderr)
        sys.exit(1)
    print("{}")
else:
    print("Unexpected mock arguments", args, file=sys.stderr)
    sys.exit(2)
