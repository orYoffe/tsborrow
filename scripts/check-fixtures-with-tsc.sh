#!/usr/bin/env bash
set -euo pipefail

readonly TYPESCRIPT_VERSION="${TYPESCRIPT_VERSION:-7.0.2}"
readonly DECLARATIONS="tests/types/fixture-globals.d.ts"
readonly INVALID_CONTROL="tests/types/invalid-control.ts"

mapfile -t fixtures < <(
  find tests/fixtures -type f \( -name '*.js' -o -name '*.ts' \) -print | sort
)

if [ "${#fixtures[@]}" -eq 0 ]; then
  echo "no JavaScript or TypeScript fixtures found" >&2
  exit 1
fi

negative_count=0
for fixture in "${fixtures[@]}"; do
  expected="${fixture}.expected"
  if [ ! -f "$expected" ]; then
    echo "missing expected output for $fixture" >&2
    exit 1
  fi

  expected_output="$(tr -d '\r\n' < "$expected")"
  if [ "$expected_output" != "OK" ]; then
    negative_count="$((negative_count + 1))"
  fi
done

tsc() {
  npx --yes --package "typescript@${TYPESCRIPT_VERSION}" tsc \
    --allowJs \
    --checkJs \
    --lib ES2022 \
    --module ESNext \
    --moduleDetection force \
    --moduleResolution Bundler \
    --noEmit \
    --noErrorTruncation \
    --pretty false \
    --strict \
    --target ES2022 \
    "$DECLARATIONS" \
    "$@"
}

set +e
control_output="$(tsc "$INVALID_CONTROL" 2>&1)"
control_status="$?"
set -e

if [ "$control_status" -eq 0 ]; then
  echo "TypeScript unexpectedly accepted the invalid control" >&2
  exit 1
fi

if ! grep -Fq 'error TS2322:' <<< "$control_output"; then
  printf '%s\n' "$control_output" >&2
  echo "TypeScript rejected the control for an unexpected reason" >&2
  exit 1
fi

echo "ok: TypeScript rejected the invalid control with TS2322"

set +e
fixture_output="$(tsc "${fixtures[@]}" 2>&1)"
fixture_status="$?"
set -e

if [ "$fixture_status" -ne 0 ]; then
  printf '%s\n' "$fixture_output" >&2
  echo "fixture corpus must compile cleanly before tsborrow expectations are evaluated" >&2
  exit 1
fi

printf 'ok: TypeScript %s accepted %d fixture(s); %d are independent tsborrow-negative cases\n' \
  "$TYPESCRIPT_VERSION" \
  "${#fixtures[@]}" \
  "$negative_count"
