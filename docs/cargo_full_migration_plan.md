# Full Cargo Migration Plan: `cargo run` / `cargo install` for the Browser

**Document Version**: 1.0
**Created**: 2026-07-28
**Status**: Approved plan, ready for execution
**Scope**: Linux x86_64 first. Windows and macOS are explicitly deferred (see §4.4).

[TOC]

---

## 1. Purpose and end state

This is the master execution plan for making Cargo the front door — and eventually
the entire build system — of the Crustonium browser.

**End state (user-visible):**

```sh
git clone https://github.com/rexlunae/crustonium
cd crustonium
cargo run                 # builds the browser and launches it
cargo install --path .    # installs a `crustonium` binary on PATH that launches the browser
```

This document supersedes the *timeline* in
[docs/cargo_adoption_plan.md](cargo_adoption_plan.md) but keeps its phase
vocabulary. The critical strategic change: **none of the existing phases ever
delivers a working `cargo run`**. This plan delivers it early (Stage A) by making
Cargo *orchestrate* the existing GN/Ninja build, then progressively shrinks GN's
role (Stages B–D) until it is gone. Working front door first, purity later.

Mapping to existing phase numbering (do not renumber the old docs):

| This plan | Existing docs equivalent | Status |
|---|---|---|
| Stage A: Cargo Front Door | New "Phase 2.5" (runs in parallel with Phase 2 Tier 2/3) | This plan §6 |
| Stage B: Hermetic deps + leaf migration | Phase 2 Tier 2/Tier 3 + Phase 3.1 | This plan §7 |
| Stage C: Translator + mass C++ migration | Phase 3 (Comprehensive Migration) | This plan §8 |
| Stage D: Retire GN | Phase 4 (Completion) | This plan §9 |

---

## 2. How to execute this plan (agent contract)

This plan is written to be executed **one task at a time by a smaller LLM agent**.
Rules for every executing agent:

1. **One task per PR.** Pick the lowest-numbered unclaimed task whose `Depends`
   are complete (see Task Index, §11). Do not combine tasks.
2. **Never break the two green builds.** After your change, both of these must
   still pass:
   - `cargo build --workspace --exclude ble-scan-parser --exclude mojo-rust-system-api`
   - The GN build (where it was previously green — do not edit `BUILD.gn` files
     unless the task says to).
3. **Run the task's Verify command(s) and paste the output in the PR.** "Done
   when" must follow mechanically from the Verify output.
4. **Environment tiers.** Each task is tagged:
   - `Env: small` — runnable on any machine with stable Rust + this repo cloned
     (no submodules, no GN). CI can verify it.
   - `Env: full` — needs a bootstrapped checkout (~100 GB disk, all submodules,
     GN toolchain) and hours of build time.
   If a task is `Env: full` and your environment is `small`: implement the
   change, run every Verify that *is* possible, and mark the PR title with
   `[needs-full-env-verify]` instead of pretending the full verify passed.
   Never fabricate Verify output.
5. **Dual-build rule (Stages A–C).** A component migrated to Cargo keeps its
   `BUILD.gn` working until Stage D explicitly deletes it. Both builds compile
   the same source files.
6. **Dependency discipline.** New Rust dependencies go in `[workspace.dependencies]`
   in the root `Cargo.toml` and are referenced with `workspace = true`. Prefer
   zero new dependencies; the launcher uses only `std`.
7. **Upstream mergeability.** This fork regularly merges upstream Chromium.
   Put all new files under `src/` (launcher), `build/cargo/`, `tools/cargo/`,
   or `docs/` — paths upstream does not own. Minimize edits to upstream files.
8. **Bookkeeping.** In the same PR, tick your task's checkbox in §11 and, when
   a milestone completes, update `docs/rust/MIGRATION_STATUS.md`.
9. **Vague-word ban.** If while executing you find this plan ambiguous, do the
   smallest reasonable thing, and record the decision in the PR description.

---

## 3. Current state (verified 2026-07-28, on this checkout)

Facts below were verified empirically; trust them over older docs.

**Cargo workspace works.** `cargo metadata` parses; `cargo check -p
workspace-structure-test` compiles in ~10 s with crates.io access. The 12
workspace packages and their exact names:

| Package name | Path | Kind |
|---|---|---|
| `cargo-cpp-integration-prototype` | `prototypes/cargo_cpp_integration` | staticlib+rlib, has build.rs (cc + cxx-build) |
| `workspace-structure-test` | `prototypes/workspace_structure_test` | lib |
| `qr-code-generator-ffi` | `components/qr_code_generator` | staticlib+rlib |
| `facilitated-payments-pix-validator` | `components/facilitated_payments/core/validation` | rlib+staticlib |
| `user-data-importer-parsing-ffi` | `components/user_data_importer/utility/parsing_ffi` | staticlib+rlib |
| `media-filters` | `media/filters` | staticlib+rlib |
| `rust-gtest-interop` | `testing/rust_gtest_interop` | staticlib+rlib |
| `chromium_macro_shim` | `testing/rust_gtest_interop/chromium_macro_shim` | lib |
| `gtest_attribute` | `testing/rust_gtest_interop/gtest_attribute` | proc-macro |
| `gnrt` | `tools/crates/gnrt` | lib+bin |
| `ble-scan-parser` | `device/bluetooth/bluez/ble_scan_parser` | staticlib+rlib — **Tier 2, needs GN-generated headers** |
| `mojo-rust-system-api` | `mojo/public/rust/system` | lib — **Tier 2, needs GN-generated bindings** |

**A GN build is NOT currently possible from a plain `git clone`:**

- `.gitmodules` declares **270 submodules**; in a plain clone **all 270 are
  uninitialized** (`git submodule status` shows `-` for every one). Third-party
  *source* comes from submodules.
- `buildtools/linux64/` (the `gn` binary) **does not exist** in a plain clone.
  Binary tools come from `DEPS` hooks/CIPD, normally driven by `gclient`:
  `gn` (pin: `gn_version` at `DEPS:557`), ninja (`third_party/ninja`,
  `DEPS:2681`), clang (hook `clang_tot` → `tools/clang/scripts/update.py`),
  GN's Rust toolchain (hook `rust_tot` → `tools/rust/update_rust.py`),
  Linux sysroot (`build/linux/sysroot_scripts/install-sysroot.py`).
- Official flow (docs/linux/build_instructions.md): `install-build-deps.sh` →
  `gclient sync` → `gn gen out/Default` → `autoninja -C out/Default chrome`.

**Browser targets** (GN label → ninja target → binary):

| Target | Ninja command | Binary |
|---|---|---|
| Minimal browser | `autoninja -C out/cargo content_shell` | `out/cargo/content_shell` |
| Headless | `autoninja -C out/cargo headless_shell` | `out/cargo/headless_shell` |
| Full browser | `autoninja -C out/cargo chrome` | `out/cargo/chrome` (target `//chrome:chrome` is a group wrapping `chrome_initial`, `chrome/BUILD.gn:134`) |

**Runtime data files** (a Chromium binary cannot run bare): `icudtl.dat`,
`v8_context_snapshot.bin`, `resources.pak`, `chrome_100_percent.pak`,
`chrome_200_percent.pak`, `locales/*.pak`, `libEGL.so`, `libGLESv2.so`,
`libvk_swiftshader.so`, `vk_swiftshader_icd.json`, `chrome_crashpad_handler`,
optionally `chrome-sandbox` (SUID). content_shell needs its own
`content_shell.pak` + icu + v8 snapshot. The authoritative per-target list is
mechanical: `gn desc out/cargo <label> runtime_deps` (see Task A6). Do not
hand-maintain these lists.

**Toolchain:** no `rust-toolchain.toml` exists; the workspace builds with
whatever stable rustc is installed (verified with 1.94.1). GN builds Rust with
its own pinned toolchain via `tools/rust/update_rust.py` — these are, and will
remain, two different compilers until Stage D.

**Vendored crates:** `third_party/rust/chromium_crates_io/vendor/` holds 302
crates managed by `gnrt` for the GN build, but they have **no
`.cargo-checksum.json`** files, so Cargo's vendored-source feature cannot
consume that directory as-is (the commented-out `[source]` block in
`.cargo/config.toml` also points at the wrong path). Fixed in Task B2.

**Existing migration tooling to reuse, not reinvent:**
`tools/cargo_migration/gn_to_cargo.py`, `tools/cargo_migration/hybrid_build.sh`,
`tools/crates/gnrt` (crates.io→GN direction), templates in
`docs/rust/phase2/migration_templates.md`.

---

## 4. Design decisions (locked — do not relitigate per-task)

### 4.1 Root package, not a subdirectory crate

The repo root `Cargo.toml` gains a `[package] name = "crustonium"` section on
top of the existing `[workspace]` section. Rationale: bare `cargo run` and
`cargo install --path .` at the repo root then do exactly what the user asked
— no `-p` flag, no cd. The launcher lives in a new root `src/` directory
(upstream Chromium has no root `src/`; merge-safe) plus a root `build.rs`.

Consequences to be aware of: bare `cargo build` now means "build the browser";
`cargo build --workspace` still builds all member crates; CI uses the latter.

### 4.2 Orchestrate first, absorb later

`build.rs` drives the *existing* GN/Ninja build in `out/cargo/` (a dedicated
out-dir so it never fights a developer's `out/Default`). Ninja remains the
incremental engine — build.rs always invokes it; a no-op ninja run is cheap.
Cargo's role in Stage A is bootstrap-checking, invoking, and packaging — not
compiling C++. Stages B–D move real compilation under Cargo.

### 4.3 Never hard-fail the Cargo build for missing bootstrap

`build.rs` must **always succeed** on a plain clone: if the GN toolchain is
missing it records that fact (into `$OUT_DIR/bootstrap_status.json`) and skips
orchestration; the *launcher* then explains at runtime what to do (`crustonium
doctor`). This keeps `cargo build --workspace`, CI, and `cargo run -- doctor`
working on any machine, while `cargo run` on a bootstrapped machine builds and
launches the browser. Env contract:

| Variable | Effect |
|---|---|
| `CRUSTONIUM_LAUNCHER_ONLY=1` | build.rs skips bootstrap checks and ninja entirely (CI speed) |
| `CRUSTONIUM_TARGET` | `chrome` (default after M5) \| `content_shell` \| `headless_shell` |
| `CRUSTONIUM_OUT` | runtime override of the artifact directory (default: baked `<repo>/out/cargo`) |

### 4.4 Supported install invocations

- **Supported**: `cargo run` / `cargo install --path .` from a clone;
  `cargo install --git https://github.com/rexlunae/crustonium` (best-effort:
  the cached checkout in `~/.cargo/git/checkouts/` persists, so the baked
  artifact path stays valid, but the clone is enormous).
- **Not supported, ever**: publishing to crates.io. The tree is tens of GB;
  crates.io's package size limit is ~10 MB. State this in docs rather than
  letting anyone attempt it.
- `cargo install` only copies the binary into `~/.cargo/bin/`. Runtime data
  files therefore live in `~/.local/share/crustonium/<rev>/` staged by the
  launcher's `setup` subcommand (Task A8) — auto-run on first launch.

### 4.5 Linux x86_64 first

Every task in this plan targets Linux x86_64. Windows/macOS work is out of
scope until Stage B is complete; the launcher must compile everywhere but may
print "unsupported platform" at runtime off-Linux.

### 4.6 Two rustcs during transition

The Cargo workspace pins stable Rust via `rust-toolchain.toml` (Task B1). GN
keeps its own nightly. Do not try to unify them before Stage C; unification is
Task D2's problem.

---

## 5. Milestones

| ID | Definition (one line) | Verify command | Env |
|---|---|---|---|
| M0 | Baseline green: workspace builds + tests pass, plan merged | `cargo build --workspace --exclude ble-scan-parser --exclude mojo-rust-system-api && cargo test --workspace --exclude ble-scan-parser --exclude mojo-rust-system-api` | small |
| M1 | Root `crustonium` package + launcher stub exists | `cargo run -- --launcher-version` prints version; `cargo run -- doctor` exits 0 with a report | small |
| M2 | Bootstrap tooling produces a working GN toolchain | `python3 tools/cargo/bootstrap.py && out/cargo-toolchain/gn --version` | full |
| M3 | `cargo run` builds & launches content_shell | `CRUSTONIUM_TARGET=content_shell cargo run -- --run-web-tests about:blank` exits 0 | full |
| M4 | headless_shell supported | `CRUSTONIUM_TARGET=headless_shell cargo run -- --dump-dom about:blank` prints HTML | full |
| M5 | `cargo run` launches full chrome by default | `cargo run -- --version` prints Chromium version string | full |
| M6 | `cargo install --path .` works end-to-end | `cargo install --path . && crustonium --version` | full |
| M7 | Hermetic offline build of pure-Rust members | `cargo build --offline -p qr-code-generator-ffi` (after `rm -rf ~/.cargo/registry` equivalent isolation) | small |
| M8 | Tier 2 crates build under pure Cargo | `cargo build -p ble-scan-parser -p mojo-rust-system-api` (no GN) | small |
| M9 | First C++ leaf component compiled by Cargo (cc crate), tests pass | `cargo test -p <component>` | small |
| M10 | `gn2cargo` translator emits buildable crates for `//url` + a `//base` subset | `cargo build -p url-cargo` (generated) | full |
| M11 | content_shell closure builds under Cargo without Ninja | `CRUSTONIUM_PURE_CARGO=1 cargo build` produces runnable content_shell | full |
| M12 | GN retired: docs, CI, and tree have no GN on the critical path | CI green with cargo-only pipeline | full |

---

## 6. Stage A — Cargo Front Door

Goal: M1 through M6. Everything here is orchestration; no C++ moves.

### Task A0 — Baseline audit and status sync

`Env: small` · `Depends: —`

- **Goal**: Confirm M0 and make the docs tell the truth before new work starts.
- **Files**: `docs/rust/MIGRATION_STATUS.md`, `README.md`.
- **Steps**:
  1. Run both M0 verify commands; capture output.
  2. Add a "Phase 2.5: Cargo Front Door — started" section to
     `docs/rust/MIGRATION_STATUS.md` linking to this plan.
  3. Add this plan to README.md's Rust Development links.
- **Verify**: the two M0 commands.
- **Done when**: both pass and the two docs link here.

### Task A1 — Root `crustonium` package and launcher stub

`Env: small` · `Depends: A0`

- **Goal**: M1. `cargo run -- --launcher-version` and `cargo run -- doctor`
  work on a plain clone with no GN anywhere.
- **Files**: `Cargo.toml` (root), `src/main.rs` (new), `build.rs` (new),
  `.gitignore` (verify only — `/out*/` and `/target` are already ignored,
  lines 252/354).
- **Steps**:
  1. In root `Cargo.toml`, add above `[workspace]`:
     ```toml
     [package]
     name = "crustonium"
     version.workspace = true
     edition.workspace = true
     license.workspace = true
     repository.workspace = true
     description = "Cargo front door for the Crustonium browser"
     build = "build.rs"
     include = ["src/**", "build.rs", "build/cargo/**"]

     [[bin]]
     name = "crustonium"
     path = "src/main.rs"
     ```
     (The root package is implicitly a workspace member; do not add it to
     `members`.)
  2. `build.rs` v1: read `CRUSTONIUM_LAUNCHER_ONLY`; write
     `$OUT_DIR/bootstrap_status.json` with `{"gn": false, "checked": false}`;
     emit `cargo:rustc-env=CRUSTONIUM_REPO_DIR=` + `CARGO_MANIFEST_DIR`;
     always exit 0. Force rerun every build by emitting
     `cargo:rerun-if-changed=` on a path that never exists
     (`build/cargo/.force-rerun-stamp`) — this is deliberate; ninja is the
     real incrementality engine (§4.2) and a stale-skip here means a stale
     browser.
  3. `src/main.rs` v1, `std` only, subcommand dispatch on `args[1]`:
     - `--launcher-version` → print `crustonium-launcher <CARGO_PKG_VERSION> (<CRUSTONIUM_REPO_DIR>)`, exit 0.
     - `doctor` → run the checks from Task A3 that don't need the bootstrap
       script (git present, submodules initialized?, `out/cargo-toolchain/gn`
       exists?, `out/cargo/` exists?), print a ✅/❌ table with the fix
       command for each ❌, exit 0.
     - anything else → for now print "browser not yet wired; run `crustonium doctor`", exit 1.
- **Verify**:
  ```sh
  cargo run -- --launcher-version
  cargo run -- doctor
  cargo build --workspace --exclude ble-scan-parser --exclude mojo-rust-system-api
  ```
- **Done when**: all three succeed on a plain clone; M1 ticked.

### Task A2 — Checked-in GN args profiles

`Env: small` · `Depends: A1`

- **Goal**: Deterministic, reviewable GN configuration for the Cargo out-dir.
- **Files**: `build/cargo/args_dev.gn`, `build/cargo/args_release.gn` (new).
- **Steps**: create the two files:
  ```gn
  # build/cargo/args_dev.gn — used when cargo PROFILE=debug
  is_debug = false            # "debug browser" is not the dev default; too slow
  symbol_level = 1
  is_component_build = true   # fast links
  ```
  ```gn
  # build/cargo/args_release.gn — used when cargo PROFILE=release (incl. cargo install)
  is_debug = false
  symbol_level = 0
  is_component_build = false
  ```
- **Verify**: `gn format --dry-run build/cargo/args_dev.gn build/cargo/args_release.gn`
  if `gn` available; otherwise files exist and match above.
- **Done when**: files merged.

### Task A3 — Bootstrap script (`tools/cargo/bootstrap.py`)

`Env: full` to fully verify · `Depends: A1`

- **Goal**: M2. One idempotent script takes a plain clone to "GN build
  possible", replicating what `gclient sync` provides, without depot_tools.
- **Files**: `tools/cargo/bootstrap.py` (new), `docs/cargo_front_door.md` (new, brief usage doc).
- **Steps** — the script performs, in order, each step skippable when already
  satisfied (idempotent), all output prefixed `[bootstrap]`:
  1. Preflight: `python3 >= 3.8`, `git`, `curl` present; warn (not fail) that
     `build/install-build-deps.sh` needs sudo and must be run once by a human.
  2. Submodules: `git submodule update --init --recursive --jobs 8` (270
     submodules, tens of GB — print a size warning first).
  3. `gn`: download the CIPD package pinned by `gn_version` in `DEPS`
     (`DEPS:557`; fetch via
     `https://chrome-infra-packages.appspot.com/dl/gn/gn/linux-amd64/+/<version>`,
     unzip) into `out/cargo-toolchain/gn`. Read the pin from DEPS at runtime —
     never hardcode.
  4. `ninja`: same approach from the `src/third_party/ninja` entry (`DEPS:2681`)
     into `out/cargo-toolchain/ninja`.
  5. clang: `python3 tools/clang/scripts/update.py`.
  6. GN's Rust toolchain: `python3 tools/rust/update_rust.py`.
  7. sysroot: `python3 build/linux/sysroot_scripts/install-sysroot.py --arch=amd64`.
  8. `--check` mode: perform all existence checks, change nothing, exit 0/1.
- **Verify** (full env):
  ```sh
  python3 tools/cargo/bootstrap.py
  out/cargo-toolchain/gn --version && out/cargo-toolchain/ninja --version
  python3 tools/cargo/bootstrap.py --check   # second run: all satisfied, fast
  ```
  Small-env verify: `python3 tools/cargo/bootstrap.py --check` reports missing
  pieces and exits 1 without side effects.
- **Done when**: full-env verify passes (or PR is `[needs-full-env-verify]`
  with small-env verify shown); M2 ticked when confirmed on a full env.

### Task A4 — build.rs orchestration: `gn gen` + ninja

`Env: full` · `Depends: A2, A3`

- **Goal**: `cargo build` on a bootstrapped machine produces
  `out/cargo/content_shell`.
- **Files**: `build.rs`.
- **Steps** (extend build.rs; keep the never-hard-fail rule of §4.3):
  1. If `CRUSTONIUM_LAUNCHER_ONLY=1` → skip to writing status JSON.
  2. Run bootstrap check (equivalent of `bootstrap.py --check` logic, in Rust
     or by invoking the script). If unsatisfied: write status JSON with the
     missing list, emit ONE `cargo:warning=crustonium: GN toolchain not
     bootstrapped; `cargo run` will print instructions`, and exit 0.
  3. If `out/cargo/args.gn` missing or differs from the profile file: copy
     `build/cargo/args_{dev|release}.gn` → `out/cargo/args.gn` (choose by
     `PROFILE` env), then run `out/cargo-toolchain/gn gen out/cargo`.
  4. Run `out/cargo-toolchain/ninja -C out/cargo <target>` where `<target>` =
     `CRUSTONIUM_TARGET` or default `content_shell` (becomes `chrome` in A7).
     Stream ninja output to `out/cargo/cargo-ninja.log`; emit a
     `cargo:warning=` line only at start ("building <target>; tail
     out/cargo/cargo-ninja.log for progress") and on completion. Nonzero ninja
     exit **is** a build failure — `panic!` with the last 40 log lines.
  5. Write `$OUT_DIR/bootstrap_status.json` `{gn: true, target: "...", out: "out/cargo"}`.
- **Verify** (full env): `cargo build` twice — first run builds (hours), second
  run's ninja step is a no-op (seconds). Then `test -x out/cargo/content_shell`.
- **Done when**: verify passes on a full env.

### Task A5 — Launcher v1: run content_shell

`Env: full` · `Depends: A4`

- **Goal**: M3 — `cargo run` launches a browser window.
- **Files**: `src/main.rs`.
- **Steps**:
  1. Default subcommand (no recognized subcommand): resolve out dir
     (`CRUSTONIUM_OUT` → else `<CRUSTONIUM_REPO_DIR>/out/cargo`), resolve
     target binary (`CRUSTONIUM_TARGET` default per current stage), verify the
     binary exists — if not, print the doctor hint and exit 1.
  2. `exec()` the binary (`std::os::unix::process::CommandExt::exec`),
     forwarding **all** remaining argv unchanged and preserving env. Note in a
     comment: `cargo run -- <flags>` → flags land after the binary name.
  3. Sandbox handling: if `chrome-sandbox` exists but is not SUID-root, set
     `env: CHROME_DEVEL_SANDBOX` only if user provided it; otherwise rely on
     user-namespace sandboxing (modern kernels) and do NOT inject
     `--no-sandbox`. Print a one-line warning if
     `/proc/sys/kernel/unprivileged_userns_clone` exists and is `0`.
- **Verify** (full env):
  ```sh
  CRUSTONIUM_TARGET=content_shell cargo run -- --run-web-tests about:blank; echo exit=$?
  ```
- **Done when**: exit=0; M3 ticked.

### Task A6 — Mechanical runtime-deps manifest

`Env: full` · `Depends: A4`

- **Goal**: The definitive list of files the browser needs at runtime,
  generated, never hand-written — consumed by Task A8's staging.
- **Files**: `build.rs` (extend), no checked-in lists.
- **Steps**: after a successful ninja run, execute
  `out/cargo-toolchain/gn desc out/cargo //content/shell:content_shell runtime_deps`
  (label per target: `//chrome:chrome`, `//headless:headless_shell`), filter to
  paths that exist under `out/cargo/`, write `out/cargo/runtime_deps_<target>.txt`.
- **Verify** (full env): file exists, contains `icudtl.dat` and a `.pak` line.
- **Done when**: verify passes for content_shell.

### Task A7 — Full chrome as the default target

`Env: full` · `Depends: A5, A6`

- **Goal**: M4 + M5. All three targets work; default flips to `chrome`.
- **Files**: `build.rs`, `src/main.rs`, `docs/cargo_front_door.md`.
- **Steps**: allow `CRUSTONIUM_TARGET` ∈ {`chrome`, `content_shell`,
  `headless_shell`}; default `chrome`; ninja target and binary name follow §3's
  table; runtime_deps manifest per target (A6).
- **Verify** (full env):
  ```sh
  CRUSTONIUM_TARGET=headless_shell cargo run -- --dump-dom about:blank | head -3
  cargo run -- --version
  ```
- **Done when**: headless prints HTML; chrome prints its version string; M4+M5 ticked.

### Task A8 — `cargo install` support: data staging

`Env: full` · `Depends: A7`

- **Goal**: M6. An installed `crustonium` binary works from anywhere.
- **Files**: `src/main.rs`, `build.rs` (bake `CRUSTONIUM_BUILD_REV` via
  `git rev-parse --short HEAD`), `docs/cargo_front_door.md`.
- **Steps**:
  1. `crustonium setup`: read `out/cargo/runtime_deps_<target>.txt` from the
     baked repo dir, copy the binary + every listed file (preserving relative
     layout) into `${XDG_DATA_HOME:-~/.local/share}/crustonium/<rev>/`, write
     `install.json` (`{rev, target, staged_at}`) next to it.
  2. Launch resolution order: `CRUSTONIUM_OUT` env → staged dir (if
     `install.json` exists for baked rev) → baked `<repo>/out/cargo`. If
     running from an install (`argv[0]` under `.cargo/bin`) and no staged dir:
     auto-run `setup` first, then exec.
  3. Document: `cargo install --path .` is the supported path;
     `--git` is best-effort; crates.io is never supported (§4.4).
- **Verify** (full env):
  ```sh
  cargo install --path .
  cd /tmp && crustonium --version
  ```
- **Done when**: installed binary launches chrome from staged data; M6 ticked.

### Task A9 — CI: front-door smoke job

`Env: small` · `Depends: A1`

- **Goal**: CI proves the launcher + build.rs never break on plain clones.
- **Files**: `.github/workflows/cargo-ci.yml`.
- **Steps**: add a `front-door-smoke` job (ubuntu-latest):
  ```sh
  CRUSTONIUM_LAUNCHER_ONLY=1 cargo build
  CRUSTONIUM_LAUNCHER_ONLY=1 cargo run -- --launcher-version
  CRUSTONIUM_LAUNCHER_ONLY=1 cargo run -- doctor
  python3 tools/cargo/bootstrap.py --check || true   # must not crash
  ```
  Also update the existing build/test jobs if the root package changed any
  invocation (they keep `--workspace --exclude ...` and are unaffected by the
  root package only if `CRUSTONIUM_LAUNCHER_ONLY=1` is exported job-wide — do
  that).
- **Verify**: CI green on the PR.
- **Done when**: job merged and green.

### Task A10 — Front-door documentation

`Env: small` · `Depends: A8`

- **Goal**: A newcomer can go clone → running browser with one page of docs.
- **Files**: `docs/cargo_front_door.md` (finalize), `README.md` (Quick Start
  section: the four commands from §1).
- **Verify**: docs build none; review that every command in the doc appears in
  a Verify block somewhere in this plan (i.e., is known to work).
- **Done when**: merged; MIGRATION_STATUS updated: "Phase 2.5 complete".

---

## 7. Stage B — Hermetic deps and leaf migration

Goal: M7–M9. Shrink what GN uniquely provides.

### Task B1 — Pin the workspace toolchain

`Env: small` · `Depends: A0`

- **Files**: `rust-toolchain.toml` (new, repo root): `[toolchain] channel =
  "1.94.1"` plus `components = ["rustfmt", "clippy"]`; update
  `.github/workflows/cargo-ci.yml` to drop `dtolnay/rust-toolchain@stable`
  auto-float (the action respects rust-toolchain.toml if told no explicit
  toolchain; simplest: keep action but set `toolchain: 1.94.1`).
- **Verify**: `cargo --version` in repo prints 1.94.1; CI green.
- **Done when**: merged.

### Task B2 — Hermetic vendored Cargo sources

`Env: small` · `Depends: B1`

- **Goal**: M7 — workspace builds offline; no crates.io at build time.
- **Files**: `.cargo/config.toml`, `third_party/rust/cargo_vendor/` (new),
  `docs/cargo_front_door.md`.
- **Steps**:
  1. Do **not** point Cargo at `third_party/rust/chromium_crates_io/vendor/` —
     verified: it lacks `.cargo-checksum.json` files (gnrt-managed, not
     cargo-vendor-managed). Instead run `cargo vendor third_party/rust/cargo_vendor`
     and commit the result.
  2. Replace the commented `[source]` block in `.cargo/config.toml` with:
     ```toml
     [source.crates-io]
     replace-with = "vendored-sources"
     [source.vendored-sources]
     directory = "third_party/rust/cargo_vendor"
     ```
  3. File a follow-up note in `docs/rust/MIGRATION_STATUS.md`: long-term the
     two vendor dirs should merge (gnrt could learn to emit checksums); do not
     attempt that in this task.
- **Verify**: `cargo build --offline -p qr-code-generator-ffi` on a machine
  with an empty `CARGO_HOME` registry (CI: set `CARGO_HOME` to a temp dir).
- **Done when**: offline build green in CI; M7 ticked.

### Task B3 — Tier 2 unlock: `ble-scan-parser` standalone

`Env: small` · `Depends: B1`

- **Goal**: Remove the "needs GN-generated headers" constraint by generating
  what it needs in its own `build.rs` (pattern:
  `prototypes/cargo_cpp_integration/build.rs` — cc + cxx-build).
- **Files**: `device/bluetooth/bluez/ble_scan_parser/build.rs`,
  `Cargo.toml` (that crate), root `Cargo.toml` + CI (drop the `--exclude
  ble-scan-parser`).
- **Steps**: identify the exact GN-generated inputs (see
  `docs/rust/phase2/TIER_2_FIRST_COMPONENT.md`), replicate each via cxx-build
  in build.rs; keep the GN path working (dual-build rule).
- **Verify**: `cargo build -p ble-scan-parser && cargo test -p ble-scan-parser`
  on a plain clone.
- **Done when**: CI builds it without the exclude flag.

### Task B4 — Tier 2 unlock: `mojo-rust-system-api`

`Env: small` · `Depends: B3`

- Same shape as B3 for the Mojo bindings: generate or check in the needed
  bindings via build.rs (bindgen is available: `build/rust/rust_bindgen.gni`
  shows what GN feeds it; replicate the same headers/flags). Drop its CI
  exclude. Verify: `cargo build -p mojo-rust-system-api` plain-clone. This
  completes **M8** together with B3.

### Task B5 — Inventory and convert remaining pure-Rust GN targets

`Env: small` · repeatable · `Depends: B1`

- **Goal**: Every `rust_static_library` / `cargo_crate` GN target that has no
  C++ deps gets a workspace crate (Tier 1 pattern, template in
  `docs/rust/phase2/migration_templates.md`).
- **Steps**: (a) one PR: check in the inventory
  (`grep -rl "rust_static_library\|cargo_crate" --include=BUILD.gn` output,
  triaged into a table in `docs/rust/phase2/TIER_1_REMAINING.md`); (b) then one
  PR per crate: add Cargo.toml, add to `members`, dual-build.
- **Verify per crate**: `cargo build -p <crate> && cargo test -p <crate>`.
- **Done when**: inventory table shows 100% converted.

### Task B6 — First C++ leaf component under Cargo (×3)

`Env: small` · `Depends: B2`

- **Goal**: M9 — prove the `cc`-crate pattern on real Chromium C++.
- **Steps**: pick three small leaf C++ targets with no generated-code inputs
  (candidate hunting: `gn desc` deps that are only `//base`-free leaves; the
  C++ side of already-migrated crates, e.g. the QR generator's C++ wrapper, is
  a good first pick). For each: a crate wrapping the C++ via
  `cc::Build` in build.rs + cxx bridge if it has an API, tests ported to
  `cargo test`. Dual-build stays.
- **Verify**: `cargo test -p <each>` on plain clone.
- **Done when**: three merged; M9 ticked; pattern written up in
  `docs/rust/phase2/migration_templates.md` as "Tier 2.5: C++ leaf via cc".

### Task B7 — Dual-build drift guard

`Env: small` · `Depends: B5`

- **Goal**: CI fails if a migrated crate's source list drifts between
  BUILD.gn and Cargo.toml.
- **Files**: `tools/cargo/check_dual_build.py` (new), CI job.
- **Steps**: script parses each migrated crate's BUILD.gn `sources` list and
  compares with files under the crate's `src/` (+ explicit map for
  exceptions); exits 1 on drift.
- **Verify**: `python3 tools/cargo/check_dual_build.py` exits 0; seeded drift
  (local test) exits 1.
- **Done when**: CI job merged and green.

---

## 8. Stage C — Translator and mass C++ migration

Goal: M10–M11. This is the long haul (thousands of GN targets — automation
only; the rule is **build the translator, then run the translator**; no
hand-porting of BUILD.gn files beyond Stage B's leaves).

### Task C1 — `gn2cargo` skeleton

`Env: full` · `Depends: A4, B6`

- **Files**: `tools/cargo/gn2cargo/` (new workspace member, Rust).
- **Steps**: consume `gn desc out/cargo <label> --format=json` (and/or `gn gen
  --ide=json` project.json); model targets (`static_library`, `source_set`,
  `component`, `action`, `executable`); emit, for a given label, a generated
  crate under `out/cargo-gen/<sanitized-label>/` with Cargo.toml + build.rs
  invoking `cc::Build` with the *same* sources, include_dirs, defines, cflags
  GN used. Evaluate `tools/cargo_migration/gn_to_cargo.py` first; supersede it
  (note in its README) rather than maintaining two translators.
- **Verify**: golden test — translate one Stage B leaf target and diff against
  its hand-written crate's effective build (both produce linkable staticlibs;
  `cargo build -p <generated>` passes).

### Task C2 — `action()` target translation table

`Env: full` · `Depends: C1`

- **Goal**: Generated code without GN: each `action()` class gets a build.rs
  recipe that shells out to the **same** in-tree Python generators.
- **Steps**: build the table incrementally, starting with the three that
  dominate: mojom bindings (`mojo/public/tools/`), grit resources (`tools/grit/`),
  protobuf. gn2cargo emits build.rs steps that run the identical script with
  identical args (extracted from `gn desc ... --format=json` `args`), with
  inputs/outputs declared via `cargo:rerun-if-changed`.
- **Verify**: translate one mojom-consuming target; generated crate builds and
  its outputs byte-match the ninja-produced ones (`diff -r`).

### Task C3 — Subsystem march: `//url`, then `//base` subset

`Env: full` · `Depends: C2`

- **Goal**: M10.
- **Steps**: topologically order deps from project.json; translate the `//url`
  closure (small, few actions) into generated crates; wire its existing GN unit
  tests to `cargo test` via `rust_gtest_interop` where applicable or a C++ test
  runner crate. Then a `//base` subset (`base/strings`, `base/numerics` are
  header-heavy and low-dep). Track progress in a generated table
  (`docs/rust/phase3/CARGO_COVERAGE.md`) with target counts:
  translated / building / tests-green.
- **Verify**: `cargo build -p url-cargo && cargo test -p url-cargo-tests`.
- **Done when**: //url closure green; M10 ticked.

### Task C4 — Widen to the content_shell closure

`Env: full` · repeatable · `Depends: C3`

- **Steps**: iterate C1–C3 outward through the dependency graph toward
  `//content/shell:content_shell` (the smallest full-browser closure).
  Milestone gate is mechanical: `CRUSTONIUM_PURE_CARGO=1` makes build.rs skip
  ninja and link the browser from Cargo-built artifacts only; the flag flips
  per-target as closures complete. Every iteration updates CARGO_COVERAGE.md.
- **Done when**: content_shell runs from a pure-Cargo build — **M11**. (chrome
  follows the same loop; it's "more of the same", tracked in the same table.)

---

## 9. Stage D — Retire GN

### Task D1 — Cargo-primary CI

`Env: full` · `Depends: M11`

Flip CI: cargo builds are the gating jobs (including a full-env pipeline on a
beefy runner or scheduled job); GN jobs demoted to non-blocking legacy lane.

### Task D2 — Toolchain unification

`Env: full` · `Depends: D1`

One rustc: move the workspace onto the same pinned toolchain the C++ interop
needs (or vice versa); delete the dual-toolchain caveat from §4.6; remove
`bootstrap.py` steps that only served GN.

### Task D3 — Delete the GN path

`Env: full` · `Depends: D2`

Remove dual-build guard (B7), delete migrated BUILD.gn files, `build/rust/`
GN templates, and finally `gn`/`ninja` from bootstrap. Update every doc that
mentions GN as current. **M12.**

---

## 10. Risk register

| # | Risk | Mitigation |
|---|---|---|
| 1 | Bootstrap downloads (submodules, clang, sysroot) are huge and network-dependent | bootstrap.py is idempotent + resumable; `--check` mode; document sizes up front |
| 2 | DEPS hook parity: gclient does things bootstrap.py misses | Scope to the 7 named steps (§6/A3); on gn gen failure, diff against a gclient-managed checkout and add the missing hook to A3's list |
| 3 | `cargo install --git` builds in `~/.cargo/git/checkouts` — path assumptions break | Launcher resolves via baked path *and* staged-data dir (A8); baked path in the cached checkout persists |
| 4 | Long ninja runs inside build.rs look frozen (cargo hides stdout) | Log file + `cargo:warning` breadcrumbs (A4); document `tail -f out/cargo/cargo-ninja.log` |
| 5 | Root package changes semantics of bare `cargo build` for existing devs | `CRUSTONIUM_LAUNCHER_ONLY=1` escape hatch; CI exports it; release notes in MIGRATION_STATUS |
| 6 | Sandbox: no SUID `chrome-sandbox` on cargo-installed layout | Rely on user-namespace sandbox; detect and warn (A5); never silently `--no-sandbox` |
| 7 | Upstream merges conflict with root `src/`, `Cargo.toml` edits | All new code in paths upstream doesn't own (§2.7); root Cargo.toml edit is additive |
| 8 | Two vendor dirs (gnrt's vs cargo's) drift | Accepted during B2; unification tracked as follow-up, blocked on gnrt checksum support |
| 9 | Two rustc toolchains miscompile shared staticlibs | Staticlibs cross C ABI only until D2; no Rust-to-Rust linkage across toolchains |
| 10 | Translator (C1) chases a moving GN target set as upstream merges land | Translator is re-runnable from `gn desc` output each merge; generated crates are build artifacts, not hand-edited |
| 11 | `action()` generators assume ninja env (depfiles, response files) | C2 golden tests byte-compare outputs against ninja's before a target class is declared supported |
| 12 | Smaller-model executors fabricate full-env verification | Hard rule §2.4: `[needs-full-env-verify]` tag; humans (or a full-env agent) re-run before merge |

---

## 11. Task index

Stage A — Front Door (Phase 2.5):
- [ ] A0 Baseline audit and status sync
- [ ] A1 Root `crustonium` package + launcher stub (**M1**)
- [ ] A2 Checked-in GN args profiles
- [ ] A3 Bootstrap script (**M2**)
- [ ] A4 build.rs orchestration: gn gen + ninja
- [ ] A5 Launcher v1: content_shell (**M3**)
- [ ] A6 Mechanical runtime-deps manifest
- [ ] A7 chrome default target (**M4, M5**)
- [ ] A8 cargo install + data staging (**M6**)
- [ ] A9 CI front-door smoke job
- [ ] A10 Front-door docs

Stage B — Hermetic + leaves:
- [ ] B1 rust-toolchain.toml pin
- [ ] B2 Vendored Cargo sources (**M7**)
- [ ] B3 Tier 2 unlock: ble-scan-parser
- [ ] B4 Tier 2 unlock: mojo-rust-system-api (**M8**)
- [ ] B5 Pure-Rust GN target inventory + conversions (repeatable)
- [ ] B6 C++ leaf components via cc ×3 (**M9**)
- [ ] B7 Dual-build drift guard

Stage C — Translator:
- [ ] C1 gn2cargo skeleton
- [ ] C2 action() translation table
- [ ] C3 //url + //base subset (**M10**)
- [ ] C4 content_shell closure (**M11**)

Stage D — Retire GN:
- [ ] D1 Cargo-primary CI
- [ ] D2 Toolchain unification
- [ ] D3 Delete the GN path (**M12**)

---

## 12. Relationship to existing documents

- [cargo_adoption_plan.md](cargo_adoption_plan.md) — strategic vision; its
  Phase 1/2 content is done or in flight; its Phase 3/4 are realized here as
  Stages C/D. Timeline there is superseded by milestone gating here.
- [rust_adoption_plan.md](rust_adoption_plan.md) — language-adoption policy;
  unchanged by this plan.
- [rust/MIGRATION_STATUS.md](rust/MIGRATION_STATUS.md) — living status; every
  milestone completion updates it (§2.8).
- [rust/phase2/](rust/phase2/) — Tier 1/2 templates and playbooks; Stage B
  reuses them directly.
- [rust/phase3/](rust/phase3/) — component-category frameworks; Stage C's
  coverage table (C3) becomes the concrete tracker those docs anticipated.
