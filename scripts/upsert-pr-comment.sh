#!/usr/bin/env bash
set -euo pipefail

marker='<!-- tsborrow-action-comment -->'

warn() {
  printf '::warning title=Unable to update tsborrow PR comment::%s\n' "$1"
}

if ! command -v gh >/dev/null 2>&1; then
  warn 'GitHub CLI is not available on this runner'
  exit 0
fi

if [ ! -f "${TSBORROW_REPORT_PATH:-}" ]; then
  warn 'The checker report file is unavailable'
  exit 0
fi

case "${TSBORROW_EXIT_CODE:-2}" in
  0) result='Passed' ;;
  1) result='Ownership violations found' ;;
  *) result='Checker error' ;;
esac

details="$(awk '
  BEGIN { used = 0 }
  {
    rendered = "    " $0
    size = length(rendered) + 1
    if (used + size > 45000) {
      print "    ... report truncated"
      exit
    }
    print rendered
    used += size
  }
' "$TSBORROW_REPORT_PATH")"

short_sha="${TSBORROW_SHA:0:12}"
body="$(printf '%s\n\n## tsborrow results\n\n**%s**\n\n%s\n\n<details><summary>Full report</summary>\n\n%s\n\n</details>\n\n[View workflow run](%s) for commit `%s`.' \
  "$marker" "$result" "${TSBORROW_SUMMARY:-No summary was emitted.}" "$details" "$TSBORROW_RUN_URL" "$short_sha")"

list_comments() {
  gh api --paginate "repos/$TSBORROW_REPOSITORY/issues/$TSBORROW_PR_NUMBER/comments" \
    --jq ".[] | select(.body | contains(\"$marker\")) | .id"
}

if ! existing="$(list_comments)"; then
  warn 'Could not read pull request comments; grant pull-requests: write permission'
  exit 0
fi

keep="$(printf '%s\n' "$existing" | sed -n '1p')"
if [ -z "$keep" ]; then
  if ! keep="$(gh api --method POST \
    "repos/$TSBORROW_REPOSITORY/issues/$TSBORROW_PR_NUMBER/comments" \
    --raw-field body="$body" --jq '.id')"; then
    warn 'Could not create the pull request comment; grant pull-requests: write permission'
    exit 0
  fi
else
  if ! gh api --method PATCH "repos/$TSBORROW_REPOSITORY/issues/comments/$keep" \
    --raw-field body="$body" >/dev/null; then
    warn 'Could not update the pull request comment; grant pull-requests: write permission'
    exit 0
  fi
fi

# A second read also converges comments created by overlapping workflow runs.
if ! current="$(list_comments)"; then
  warn 'The comment was updated, but duplicate detection could not be completed'
  exit 0
fi
canonical="$(printf '%s\n' "$current" | sed -n '1p')"
canonical="${canonical:-$keep}"
if [ "$canonical" != "$keep" ]; then
  gh api --method PATCH "repos/$TSBORROW_REPOSITORY/issues/comments/$canonical" \
    --raw-field body="$body" >/dev/null || true
fi
while IFS= read -r comment; do
  if [ -n "$comment" ] && [ "$comment" != "$canonical" ]; then
    gh api --method DELETE "repos/$TSBORROW_REPOSITORY/issues/comments/$comment" >/dev/null || \
      warn "Could not remove duplicate comment $comment"
  fi
done <<< "$current"
