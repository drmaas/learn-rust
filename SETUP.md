# Setup (one time, ~30–60 min)

Do this *before* Week 1. You only do it once.

---

## 1. Install the Rust toolchain (rustup)

`rustup` is the official installer and version manager (like `nvm` for Node). It gives you `rustc` (compiler), `cargo` (build system + package manager), and `rustup` (updater).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Choose the default options. Then add it to your shell:

```bash
source "$HOME/.cargo/env"
```

Verify:

```bash
rustc --version   # e.g. rustc 1.8x.0 (2024 edition)
cargo --version
```

Add the two tools you'll use constantly:

```bash
rustup component add rustfmt clippy
```

> **What's what:** `rustc` compiles. `cargo run` compiles *and* runs. `cargo fmt` formats your code. `cargo clippy` is a linter — a free expert code review. `cargo test` runs tests. `cargo doc` builds docs.

---

## 2. Editor: VS Code + rust-analyzer

Any editor works, but VS Code with the **rust-analyzer** extension is the smoothest path on Linux.

1. Install VS Code.
2. Install the **rust-analyzer** extension (the one by rust-lang).
3. Optional but recommended: **Error Lens** (shows compiler errors inline) and **Even Better TOML** (for `Cargo.toml`).

rust-analyzer gives you go-to-definition, find-references, type hints, and hover docs — essential for reading big codebases in Weeks 8–11.

> **⚠️ Learning rule:** while *learning*, type code manually and don't rely on autocomplete. rust-analyzer is for *reading* and *navigation*, and for catching errors — not for writing your code for you.

---

## 3. Sanity check: your first project

```bash
cargo new hello
cd hello
cargo run
# prints: Hello, world!
```

Then try:

```bash
cargo fmt        # formats
cargo clippy     # lints (should be clean for hello world)
cargo test       # runs the default test
```

If all four work, the toolchain is ready. If `cargo new` complains about git, ignore it — it's cosmetic.

---

## 4. Tauri Linux dependencies (needed for Week 12, install now so it's done)

Tauri uses the system webview on Linux, which needs WebKitGTK. On Debian/Ubuntu:

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

> **Note:** use `libwebkit2gtk-4.1-dev` (not `-4.0` — that's the old Tauri v1 package and isn't available on newer Ubuntu anyway). If you're on a non-Debian distro (Arch, Fedora), see the [Tauri prerequisites page](https://v2.tauri.app/start/prerequisites/).

You won't touch Tauri until Week 12, but installing this now means no surprises later.

---

## 5. Optional but useful

- **`cargo-edit`** (lets you `cargo add` dependencies): `cargo install cargo-edit`. *(Modern cargo has `cargo add` built in — check with `cargo add --help` first.)*
- **`cargo-watch`** (auto-recompile on save): `cargo install cargo-watch`.
- **Rust Playground** (https://play.rust-lang.org) — great for throwing away tiny snippets without creating files (this is the "write tiny programs, throw them away" rule from the README).

---

## Common problems

| Symptom | Fix |
|---------|-----|
| `cargo: command not found` | You didn't `source "$HOME/.cargo/env"` (or restart your terminal) |
| `rustup` not recognized | Re-run the install script; add `~/.cargo/bin` to PATH |
| `clippy` not found | `rustup component add clippy` |
| Compile is slow first time | Normal — it's building dependencies. Subsequent builds are fast |
| `error: linker 'cc' not found` | Install build tools: `sudo apt install build-essential` |

---

**Done?** Read [HOW-YOU-LEARN.md](HOW-YOU-LEARN.md), then start [weeks/week-01.md](weeks/week-01.md).
