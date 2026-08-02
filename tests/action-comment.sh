#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
temporary="$(mktemp -d)"
trap 'rm -rf "$temporary"' EXIT

printf 'ok: checked 1 source file(s); analyzed 1 ownership contract(s) and 0 borrow(s); no ownership violations found\n' > "$temporary/report"
chmod +x "$root/tests/fake-gh/gh"

export PATH="$root/tests/fake-gh:$PATH"
export FAKE_GH_LOG="$temporary/log"
export FAKE_GH_STATE="$temporary/state"
export GH_TOKEN='test-token'
export TSBORROW_EXIT_CODE=0
export TSBORROW_PR_NUMBER=17
export TSBORROW_REPORT_PATH="$temporary/report"
export TSBORROW_REPOSITORY='owner/repository'
export TSBORROW_RUN_URL='https://github.com/owner/repository/actions/runs/1'
export TSBORROW_SHA='0123456789abcdef'
export TSBORROW_SUMMARY='ok: checked 1 source file(s)'

bash "$root/scripts/upsert-pr-comment.sh"
test "$(wc -l < "$FAKE_GH_STATE")" -eq 1
test "$(grep -c '^POST$' "$FAKE_GH_LOG")" -eq 1

bash "$root/scripts/upsert-pr-comment.sh"
test "$(wc -l < "$FAKE_GH_STATE")" -eq 1
test "$(grep -c '^PATCH 1$' "$FAKE_GH_LOG")" -eq 1

printf '2\n' >> "$FAKE_GH_STATE"
bash "$root/scripts/upsert-pr-comment.sh"
test "$(wc -l < "$FAKE_GH_STATE")" -eq 1
test "$(grep -c '^DELETE 2$' "$FAKE_GH_LOG")" -eq 1
