#!/usr/bin/env bash
# Fixture tests for scripts/check-vocabulary.mjs.
#
# The prose scanner is the part of the vocabulary gate that is easy to get
# subtly wrong, because a listing line legitimately carries narration: a YAML
# key before the values, a comment after them, a markdown table cell beside
# them. Two bugs got through ad-hoc probing during v4.0.0 — a table row read as
# a stray value, then (fixing that) markdown's escaped pipes leaving a trailing
# backslash so every value in a table read as prose and nothing was checked at
# all. Both directions are pinned here.

set -uo pipefail

REPO="$(cd "$(dirname "$0")/.." && pwd)"
CHECK="$REPO/scripts/check-vocabulary.mjs"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

pass=0
fail=0

# expect <name> <expected-exit> <file-body>
expect() {
  local name="$1" want="$2" body="$3"
  rm -rf "$WORK/skills"; mkdir -p "$WORK/skills/fixture/references"
  printf '%s\n' "$body" > "$WORK/skills/fixture/references/listing.md"
  node "$CHECK" --skills "$WORK/skills" >/dev/null 2>&1
  local got=$?
  if [ "$got" = "$want" ]; then
    echo "PASS: $name (exit $got)"; pass=$((pass + 1))
  else
    echo "FAIL: $name — expected exit $want, got $got"; fail=$((fail + 1))
  fi
}

ALL='not_started | running | scripted_passed | passed | failed | deferred | waived'
ALL_TABLE='not_started \| running \| scripted_passed \| passed \| failed \| deferred \| waived'

expect "inline listing, complete" 0 \
  "status: not_started # $ALL   (aep-vocab: layer_gate_status)"

expect "inline listing, missing a value" 1 \
  "status: not_started # not_started | running | passed | failed | deferred | waived   (aep-vocab: layer_gate_status)"

expect "inline listing, invented value" 1 \
  "status: not_started # not_started | running | scripted_passed | passed | staging | failed | deferred | waived   (aep-vocab: layer_gate_status)"

expect "table row, complete" 0 \
  "| State | Meaning |
| ----- | ------- |
| $ALL_TABLE | all seven (aep-vocab: layer_gate_status) |"

expect "table row, missing a value" 1 \
  "| State | Meaning |
| ----- | ------- |
| not_started \\| running \\| passed \\| failed \\| deferred \\| waived | six of them (aep-vocab: layer_gate_status) |"

expect "table row, invented value" 1 \
  "| State | Meaning |
| ----- | ------- |
| not_started \\| running \\| scripted_passed \\| passed \\| staging \\| failed \\| deferred \\| waived | all seven (aep-vocab: layer_gate_status) |"

expect "narrative mentioning three states, unmarked" 0 \
  "A gate walks not_started → scripted_passed → passed as its tiers go green."

expect "marker naming a vocabulary that does not exist" 1 \
  "status: a | b   (aep-vocab: no_such_vocabulary)"

expect "substring is not a token: scripted_passed does not satisfy passed" 1 \
  "status: not_started # not_started | running | scripted_passed | failed | deferred | waived   (aep-vocab: layer_gate_status)"

expect "listing with a pattern member (dogfood_target)" 0 \
  "dogfood_target: none | cli | local | deployed:<url>   # surface (aep-vocab: dogfood_target)"

echo ""
echo "check-vocabulary fixtures: $pass passed, $fail failed"
[ "$fail" -eq 0 ]
