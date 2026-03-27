#!/usr/bin/env bash
set -euo pipefail

if [[ ! -f Cargo.toml ]]; then
  echo "error: Cargo.toml was not found in $(pwd)" >&2
  exit 1
fi

# Keep cargo-pgrx exactly aligned with the pgrx crate declared in [dependencies].
want_req="$({
  awk '
    /^\[dependencies\]$/ { in_deps = 1; next }
    /^\[/ { in_deps = 0 }
    in_deps && /^[[:space:]]*pgrx[[:space:]]*=/ { print; exit }
  ' Cargo.toml
} | sed -E 's/.*version[[:space:]]*=[[:space:]]*"([^"]+)".*/\1/; s/^[^"]*"([^"]+)".*/\1/')"

want_version="${want_req#=}"

if [[ -z "${want_version}" ]]; then
  echo "error: failed to parse pgrx version from Cargo.toml" >&2
  exit 1
fi

tests_req="$({
  awk '
    /^\[dev-dependencies\]$/ { in_dev_deps = 1; next }
    /^\[/ { in_dev_deps = 0 }
    in_dev_deps && /^[[:space:]]*pgrx-tests[[:space:]]*=/ { print; exit }
  ' Cargo.toml
} | sed -E 's/.*version[[:space:]]*=[[:space:]]*"([^"]+)".*/\1/; s/^[^"]*"([^"]+)".*/\1/')"

tests_version="${tests_req#=}"

if [[ -n "${tests_version}" && "${tests_version}" != "${want_version}" ]]; then
  echo "error: pgrx-tests (${tests_req}) must match pgrx (${want_req})" >&2
  exit 1
fi

have_version=""
if cargo pgrx --version >/dev/null 2>&1; then
  have_version="$(cargo pgrx --version | awk '{ print $2 }')"
fi

if [[ "${have_version}" != "${want_version}" ]]; then
  echo "sync: installing cargo-pgrx ${want_version} (current: ${have_version:-none})" >&2
  cargo install --locked cargo-pgrx --version "${want_version}"
fi

exec cargo pgrx "$@"
