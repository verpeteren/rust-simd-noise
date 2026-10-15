#!/usr/bin/env bash
#
# check-all.sh — compile-check this workspace for several target architectures
# from a single machine, and build + test the host target.
#
# What it does
#   * Host target:  `cargo build --workspace --all-targets`, then run the tests
#     and the doc-tests. This is the only target that is *linked* and *run*.
#   * Cross targets: `cargo check --workspace --all-targets` (lib, tests,
#     examples and benches). `cargo check` does not link, so a cross target
#     only needs its Rust std (added automatically with `rustup target add`).
#   * Warnings are treated as errors everywhere (`-D warnings`). Registry
#     dependencies are cap-lints'd by Cargo, so only this workspace's own
#     crates (simdnoise + the example) are held to that bar.
#
# C cross-compilers
#   The dev-dependency `criterion` (via `alloca`) and the example's `minifb`
#   compile a little C in their build scripts. Type-checking the tests, benches
#   or example for a *foreign* architecture therefore needs that architecture's
#   cross C compiler (a build-time C compile — still no Rust linking). When it
#   is missing, this script checks the *library only* for that target and prints
#   the apt package that unlocks the full check.
#
# Usage
#   ./check-all.sh                         # default target list
#   ./check-all.sh <triple> [<triple> ...] # check these targets instead
#   TARGETS="<triple> <triple>" ./check-all.sh
#
set -uo pipefail
cd "$(dirname "$0")"

HOST="$(rustc -vV | sed -n 's/^host: //p')"

# Cross targets to type-check. Same OS as the host keeps platform-specific
# dev/example dependencies checkable. Defaults cover the three code paths in
# this crate: x86 SIMD (host), aarch64 NEON, and the no-SIMD scalar fallback.
if [ "$#" -gt 0 ]; then
    read -r -a CROSS_TARGETS <<<"$*"
elif [ -n "${TARGETS:-}" ]; then
    read -r -a CROSS_TARGETS <<<"$TARGETS"
else
    CROSS_TARGETS=(
        aarch64-unknown-linux-gnu    # NEON
        riscv64gc-unknown-linux-gnu  # no SIMD intrinsics -> scalar fallback
    )
fi

export RUSTFLAGS="${RUSTFLAGS:-} -D warnings"
export RUSTDOCFLAGS="${RUSTDOCFLAGS:-} -D warnings"

# target triple -> cross C compiler binary / Debian package providing it.
cc_for() {
    case "$1" in
        aarch64-*-linux-gnu)      echo aarch64-linux-gnu-gcc ;;
        riscv64*-linux-gnu)       echo riscv64-linux-gnu-gcc ;;
        armv7-*-linux-gnueabihf)  echo arm-linux-gnueabihf-gcc ;;
        powerpc64le-*-linux-gnu)  echo powerpc64le-linux-gnu-gcc ;;
        s390x-*-linux-gnu)        echo s390x-linux-gnu-gcc ;;
        *)                        echo "" ;;
    esac
}
apt_pkg_for() {
    case "$1" in
        aarch64-*-linux-gnu)      echo gcc-aarch64-linux-gnu ;;
        riscv64*-linux-gnu)       echo gcc-riscv64-linux-gnu ;;
        armv7-*-linux-gnueabihf)  echo gcc-arm-linux-gnueabihf ;;
        powerpc64le-*-linux-gnu)  echo gcc-powerpc64le-linux-gnu ;;
        s390x-*-linux-gnu)        echo gcc-s390x-linux-gnu ;;
        *)                        echo "" ;;
    esac
}

FAILED=()
run() { echo "+ $*"; "$@"; }
record() { # name, then the command to run
    local name="$1"; shift
    if ! run "$@"; then
        FAILED+=("$name")
    fi
}

echo "============================================================"
echo "host target : $HOST  (build + test + doc-test)"
echo "cross checks: ${CROSS_TARGETS[*]}"
echo "============================================================"

# ---- host: the only target we link and run --------------------------------
echo
echo ">>> $HOST: build everything (lib, tests, examples, benches)"
record "build[$HOST]"    cargo build --workspace --all-targets
echo
echo ">>> $HOST: run tests"
record "test[$HOST]"     cargo test  --workspace
echo
echo ">>> $HOST: run doc-tests"
record "doctest[$HOST]"  cargo test  --workspace --doc

# ---- cross targets: type-check only (no linking) --------------------------
for t in "${CROSS_TARGETS[@]}"; do
    [ "$t" = "$HOST" ] && continue
    echo
    echo ">>> $t: cargo check (no linking)"
    if ! rustup target add "$t" >/dev/null 2>&1; then
        echo "!! could not install Rust std for '$t' (unsupported target?) — skipping"
        FAILED+=("std[$t]")
        continue
    fi

    cc="$(cc_for "$t")"
    if [ -n "$cc" ] && ! command -v "$cc" >/dev/null 2>&1; then
        pkg="$(apt_pkg_for "$t")"
        echo "!! cross C compiler '$cc' not found."
        echo "   criterion (dev-dep) and minifb (example) compile C for the target,"
        echo "   so checking tests/benches/examples for '$t' needs it. Install with:"
        echo "       sudo apt-get install ${pkg:-<your-distro-cross-gcc>}"
        echo "   Falling back to a library-only check for '$t':"
        record "check-lib[$t]" cargo check --lib --target "$t"
    else
        record "check[$t]" cargo check --workspace --all-targets --target "$t"
    fi
done

# ---- summary --------------------------------------------------------------
echo
echo "============================================================"
if [ "${#FAILED[@]}" -eq 0 ]; then
    echo "OK: all steps passed, no warnings."
    exit 0
else
    echo "FAILED steps: ${FAILED[*]}"
    exit 1
fi
