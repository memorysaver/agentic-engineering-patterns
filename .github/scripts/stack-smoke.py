#!/usr/bin/env python3
"""Smoke test of AEP's bundled default-stack procedure (`aep --skill stack`).

Follows the procedure mechanically, reading every command and file from the guidance
embedded in the given `aep` binary: scaffold with the pinned command into a temporary
directory, merge into an empty repository, `aep init`, apply the stack files, install and
confirm that every installed Effect package has the overridden release,
configure the starter checks, run every configured check in order before anything is
committed, run the task cache a second time, check that the landing test passes once the
page uses a <Link>, start the account-free local run and probe it, confirm that it wrote
nothing under the HOME's .alchemy and that the env files carry NODE_ENV=development,
confirm that deploy refuses (including a profile that only starts with the project's
name), check the root version, the product README then commit through the Git hook and
confirm nothing is left uncommitted. It never deploys and needs no Cloudflare account.
A failure names the step that failed.
"""

import argparse
import json
import os
import re
import shutil
import signal
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.request

GITHUB = os.environ.get("GITHUB_ACTIONS") == "true"
PROJECT = "stack-smoke"
STEP = "setup"


def step(name):
    global STEP
    if GITHUB and STEP != "setup":
        print("::endgroup::", flush=True)
    STEP = name
    print(f"::group::{name}" if GITHUB else f"\n== {name}", flush=True)


def fail(message):
    if GITHUB:
        print("::endgroup::", flush=True)
        print(f"::error title=Default stack smoke test::{STEP}: {message}", flush=True)
    print(f"FAILED step: {STEP}: {message}", file=sys.stderr, flush=True)
    sys.exit(1)


def run(cmd, cwd, env=None, timeout=900, capture=False):
    print("$ " + " ".join(cmd), flush=True)
    try:
        result = subprocess.run(
            cmd,
            cwd=cwd,
            env=env,
            timeout=timeout,
            text=True,
            stdout=subprocess.PIPE if capture else None,
            stderr=subprocess.STDOUT if capture else None,
        )
    except subprocess.TimeoutExpired:
        fail(f"{' '.join(cmd)} timed out after {timeout}s")
    if capture:
        print(result.stdout, flush=True)
    if result.returncode != 0:
        fail(f"{' '.join(cmd)} exited {result.returncode}")
    return result.stdout if capture else ""


def fenced_blocks(text):
    """Yield (heading, fence body) pairs: the nearest `###` heading before each fence."""
    heading, lines, i = None, text.split("\n"), 0
    while i < len(lines):
        line = lines[i]
        if line.startswith("### "):
            heading = line[4:].strip()
        match = re.match(r"^(`{3,})[\w-]*$", line)
        if match:
            fence, body, i = match.group(1), [], i + 1
            while i < len(lines) and lines[i] != fence:
                body.append(lines[i])
                i += 1
            yield heading, "\n".join(body) + "\n"
            heading = None
        i += 1


def merge(base, extra):
    for key, value in extra.items():
        if isinstance(value, dict) and isinstance(base.get(key), dict):
            merge(base[key], value)
        else:
            base[key] = value
    return base


def tree(root):
    """Every path under root with its size and modification time; empty if root is absent."""
    entries = {}
    for directory, dirs, files in os.walk(root):
        for name in dirs + files:
            path = os.path.join(directory, name)
            info = os.lstat(path)
            entries[os.path.relpath(path, root)] = (info.st_size, info.st_mtime_ns)
    return entries


def http(url, data=None, headers=None, cookie=None):
    request = urllib.request.Request(url, data=data, headers=dict(headers or {}))
    if cookie:
        request.add_header("Cookie", cookie)
    try:
        with urllib.request.urlopen(request, timeout=10) as response:
            return response.status, response.read().decode(), response.headers
    except urllib.error.HTTPError as error:
        return error.code, error.read().decode(), error.headers


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("--aep", required=True, help="aep binary whose guidance is tested")
    parser.add_argument("--workdir", help="empty directory to work in (default: a new temp dir)")
    parser.add_argument("--keep", action="store_true", help="keep node_modules afterwards")
    args = parser.parse_args()

    aep = os.path.abspath(args.aep)
    work = os.path.abspath(args.workdir or tempfile.mkdtemp(prefix="stack-smoke-"))
    os.makedirs(work, exist_ok=True)
    bin_dir = os.path.join(work, "bin")
    os.makedirs(bin_dir, exist_ok=True)
    link = os.path.join(bin_dir, "aep")
    if not os.path.exists(link):
        os.symlink(aep, link)
    env = dict(os.environ, PATH=bin_dir + os.pathsep + os.environ["PATH"])
    for name in ("CLOUDFLARE_API_TOKEN", "CLOUDFLARE_ACCOUNT_ID", "ALCHEMY_PROFILE"):
        env.pop(name, None)
    repo = os.path.join(work, PROJECT)
    dev = None
    try:
        step("read the procedure")
        procedure = run(["aep", "--skill", "stack"], work, env, capture=True)
        files = run(["aep", "--skill", "stack", "--ref", "files"], work, env, capture=True)
        checks = run(["aep", "--skill", "stack", "--ref", "checks"], work, env, capture=True)
        scaffold = [
            line.strip()
            for line in procedure.split("\n")
            if line.strip().startswith("bunx create-better-t-stack@")
        ]
        if len(scaffold) != 1 or "@latest" in scaffold[0]:
            fail("expected exactly one pinned `bunx create-better-t-stack@<version>` command")

        step("scaffold into a temporary directory")
        scratch = os.path.join(work, "scaffold")
        os.makedirs(scratch, exist_ok=True)
        run(scaffold[0].replace("<name>", PROJECT).split(), scratch, env, timeout=600)

        step("merge into an empty repository and run aep init")
        os.makedirs(repo)
        run(["git", "init", "-q", "-b", "main"], repo, env)
        run(["git", "config", "user.email", "stack-smoke@example.invalid"], repo, env)
        run(["git", "config", "user.name", "stack smoke"], repo, env)
        shutil.copytree(os.path.join(scratch, PROJECT), repo, dirs_exist_ok=True)
        shutil.rmtree(scratch)
        run(["aep", "init"], repo, env)

        step("apply the stack files")
        applied = 0
        for heading, body in fenced_blocks(files):
            match = re.match(r"^(Write|Merge into|Append to) `([^`]+)`$", heading or "")
            if not match:
                continue
            action, path = match.group(1), match.group(2).replace("__PROJECT__", PROJECT)
            # The agent replaces the README's purpose line with the product's purpose.
            body = body.replace("__PROJECT__", PROJECT).replace(
                "__PURPOSE__", "A smoke test of AEP's default stack."
            )
            target = os.path.join(repo, path)
            os.makedirs(os.path.dirname(target), exist_ok=True)
            if action == "Write":
                with open(target, "w") as handle:
                    handle.write(body)
            elif action == "Merge into":
                with open(target) as handle:
                    data = json.load(handle)
                with open(target, "w") as handle:
                    json.dump(merge(data, json.loads(body)), handle, indent=2)
                    handle.write("\n")
            else:
                existing = open(target).read() if os.path.exists(target) else ""
                lines = existing.split("\n")
                missing = [line for line in body.strip().split("\n") if line not in lines]
                if missing:
                    with open(target, "a") as handle:
                        if existing and not existing.endswith("\n"):
                            handle.write("\n")
                        handle.write("\n".join(missing) + "\n")
            print(f"{action} {path}")
            applied += 1
        if applied < 10:
            fail(f"only {applied} file blocks found in `aep --skill stack --ref files`")

        step("install, format once and create the first migration")
        run(["bun", "install"], repo, env, timeout=900)
        hooks = run(["git", "config", "core.hooksPath"], repo, env, capture=True).strip()
        if not hooks.startswith(".vite-hooks"):
            fail(f"prepare did not install the Vite+ hooks (core.hooksPath={hooks!r})")
        # Every installed `effect` and `@effect/*` package has the overridden release.
        overrides = json.load(open(os.path.join(repo, "package.json"))).get("overrides", {})
        lock = open(os.path.join(repo, "bun.lock")).read()
        installed = sorted(set(re.findall(r'"((?:effect|@effect/[\w.-]+)@[^"]+)"', lock)))
        if not installed:
            fail("bun.lock lists no effect or @effect/* package")
        mixed = [
            entry
            for entry in installed
            if overrides.get(entry.rsplit("@", 1)[0]) != entry.rsplit("@", 1)[1]
        ]
        if mixed:
            fail(f"installed Effect packages without a matching override: {mixed} (overrides: {overrides})")
        print("Effect packages: " + ", ".join(installed))
        run(["bun", "run", "format"], repo, env)
        run(["bun", "run", "db:generate"], repo, env)

        step("configure the starter checks")
        blocks = [body for _, body in fenced_blocks(checks)]
        check_block = next((b for b in blocks if "[[checks]]" in b), None)
        required = next((b for b in blocks if b.startswith("required_checks")), None)
        if not check_block or not required:
            fail("`aep --skill stack --ref checks` lacks the checks or required_checks block")
        shown = json.loads(run(["aep", "--json", "config", "show"], repo, env, capture=True))
        config_path = os.path.join(repo, ".aep", "config.toml")
        config = open(config_path).read()
        config = "\n".join(line for line in config.split("\n") if line.strip() != "checks = []")
        config = re.sub(
            r"required_checks = \[[^\]]*\]", required.strip().replace("\\", "\\\\"), config
        )
        candidate = os.path.join(work, "config-candidate.toml")
        with open(candidate, "w") as handle:
            handle.write(config.rstrip("\n") + "\n\n" + check_block)
        run(
            ["aep", "config", "update", "--file", candidate, "--expect", shown["data"]["revision"]],
            repo,
            env,
        )

        configured = json.loads(run(["aep", "--json", "config", "show"], repo, env, capture=True))
        for check in configured["data"]["config"]["checks"]:
            step(f"check {check['id']}")
            run(
                check["command"],
                os.path.join(repo, check.get("cwd") or "."),
                env,
                timeout=check.get("timeout_seconds") or 900,
            )

        step("task cache hits on a second run")
        for script in ("check", "build"):
            output = run(["bun", "run", script], repo, env, capture=True)
            hits = re.findall(r"vp run: (\d+)/(\d+) cache hit", output)
            if not hits or int(hits[-1][0]) == 0:
                fail(f"`bun run {script}` reported no cache hit on its second run")

        step("the landing test passes when the page uses a <Link>")
        landing = os.path.join(repo, "apps", "web", "src", "routes", "index.tsx")
        original = open(landing).read()
        router_import = 'import { createFileRoute } from "@tanstack/react-router";'
        heading = '<h2 className="mb-2 font-medium">API Status</h2>'
        if router_import not in original or heading not in original:
            fail("the scaffold's landing route changed; update this step")
        with open(landing, "w") as handle:
            handle.write(
                original.replace(
                    router_import, 'import { Link, createFileRoute } from "@tanstack/react-router";'
                ).replace(heading, heading + '\n<Link to="/login">Sign in</Link>')
            )
        try:
            run(["bun", "run", "test"], repo, env, timeout=600)
        finally:
            with open(landing, "w") as handle:
                handle.write(original)

        step("account-free local run")
        alchemy_home = os.path.join(os.path.expanduser("~"), ".alchemy")
        before = tree(alchemy_home)
        dev = subprocess.Popen(["bun", "run", "dev"], cwd=repo, env=env, start_new_session=True)
        deadline = time.time() + 240
        while True:
            try:
                web = http("http://localhost:3001/")
                api = http("http://localhost:3000/")
                break
            except (urllib.error.URLError, ConnectionError, TimeoutError):
                if dev.poll() is not None:
                    fail(f"`bun run dev` exited {dev.returncode} before serving")
                if time.time() > deadline:
                    fail("the local run did not serve ports 3000 and 3001 within 240s")
                time.sleep(3)
        if web[0] != 200 or "<html" not in web[1]:
            fail(f"landing page answered {web[0]}")
        if api[0] != 200 or api[1].strip() != "OK":
            fail(f"API root answered {api[0]} {api[1][:80]!r}")
        origin = {"Content-Type": "application/json", "Origin": "http://localhost:3001"}
        status, body, headers = http(
            "http://localhost:3000/api/auth/sign-up/email",
            json.dumps({"name": "Smoke", "email": "smoke@example.invalid", "password": "smoke-password-123"}).encode(),
            origin,
        )
        if status != 200 or '"token"' not in body:
            fail(f"sign-up answered {status} {body[:120]!r}")
        cookie = "; ".join(c.split(";")[0] for c in headers.get_all("Set-Cookie") or [])
        if http("http://localhost:3000/rpc/privateData", b"{}", origin, cookie)[0] != 200:
            fail("protected data refused a signed-in caller")
        if http("http://localhost:3000/rpc/privateData", b"{}", origin)[0] != 401:
            fail("protected data did not refuse an anonymous caller")
        os.killpg(dev.pid, signal.SIGTERM)
        dev.wait(timeout=30)
        dev = None
        after = tree(alchemy_home)
        added = sorted(set(after) - set(before))
        changed = sorted(p for p in before if p in after and before[p] != after[p])
        if added or changed:
            fail(f"the local run wrote under {alchemy_home}: added {added}, changed {changed}")
        for path in ("apps/server/.env", "apps/web/.env"):
            if "NODE_ENV=development" not in open(os.path.join(repo, path)).read().split("\n"):
                fail(f"{path} lacks NODE_ENV=development after the local run")

        step("deploy and destroy refuse without an explicit stage and this project's profile")
        local = dict(env)
        local.pop("CI", None)
        ci = dict(env, CI="true")

        def guard(cmd, environment):
            result = subprocess.run(
                cmd, cwd=repo, env=environment, text=True, capture_output=True, timeout=120
            )
            print(f"$ {' '.join(cmd)} -> {result.returncode}: {(result.stderr or result.stdout).strip()[:200]}")
            return result

        for cmd, environment, reason in (
            (["bun", "run", "deploy"], local, "--stage"),
            (["bun", "run", "destroy"], local, "--stage"),
            (["bun", "run", "deploy", "--", "--stage", "prod", "--profile", "default"], local, "profile"),
            (["bun", "run", "deploy", "--", "--stage", "pr-1", "--profile", f"{PROJECT}-x"], local, "profile"),
            (["bun", "run", "destroy", "--", "--stage", "pr-1", "--profile", f"{PROJECT}-x"], local, "profile"),
            (["bun", "run", "deploy", "--", "--stage", "pr-1", "--profile", PROJECT], ci, "profile"),
        ):
            result = guard(cmd, environment)
            if result.returncode == 0 or "refusing" not in result.stderr or reason not in result.stderr:
                fail(f"`{' '.join(cmd)}` did not refuse for its {reason}")
        allowed = ["bun", "run", "deploy", "--", "--stage", "pr-1", "--profile", PROJECT, "--guard-only"]
        result = guard(allowed, local)
        if result.returncode != 0 or "would run" not in result.stdout:
            fail(f"`{' '.join(allowed)}` did not pass the guard's checks")

        step("root version, product README and the standard rules")
        package = json.load(open(os.path.join(repo, "package.json")))
        if package.get("version") != "0.1.0":
            fail(f"root package.json version is {package.get('version')!r}, not 0.1.0")
        readme = open(os.path.join(repo, "README.md")).read()
        if "Better-T-Stack" in readme or "bunx alchemy deploy" in readme or "__PURPOSE__" in readme:
            fail("README.md is still the scaffold's, or its purpose line was not replaced")
        step("commit through the pre-commit hook")
        run(["git", "add", "-A"], repo, env)
        run(["git", "commit", "-q", "-m", "Scaffold the default stack"], repo, env, timeout=300)
        tracked = run(["git", "ls-files"], repo, env, capture=True).split("\n")
        leaked = [p for p in tracked if re.search(r"(^|/)(\.env|\.alchemy|\.wrangler|node_modules)(/|$)", p)]
        if leaked:
            fail(f"local secrets or state were committed: {leaked}")

        status_lines = run(["git", "status", "--porcelain"], repo, env, capture=True).strip()
        if status_lines:
            fail(f"files remain uncommitted after the commit (the hook changed them or Git does not ignore them):\n{status_lines}")

        step("done")
        print(f"The default stack scaffolds and its starter checks pass ({repo}).")
    finally:
        if dev is not None and dev.poll() is None:
            os.killpg(dev.pid, signal.SIGKILL)
        if not args.keep:
            shutil.rmtree(os.path.join(repo, "node_modules"), ignore_errors=True)
        if GITHUB:
            print("::endgroup::", flush=True)


if __name__ == "__main__":
    main()
