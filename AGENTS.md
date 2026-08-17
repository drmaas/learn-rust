# AGENTS.md

## Project purpose

`learn-rust/` is a 12-week, hands-on Rust course for an experienced programmer who is new to Rust. The repository contains:

- Course lessons and exercises in `weeks/`.
- Learning-method research in `HOW-YOU-LEARN.md`.
- Setup instructions in `SETUP.md`.
- Progress and spaced-repetition tracking in `progress.md`.
- Weekly quizzes and answer keys in `quizzes/`.
- A Tauri-oriented ClipNote capstone specification in `capstone/`.
- A compilable, tested Rust examples crate in `examples/`.
- An Anki deck source and generated package in `anki/`.

## Repository conventions

- Prefer editing existing files over adding new files unless a new module, lesson, test, or resource is clearly needed.
- Preserve the learner-facing tone: practical, encouraging, concise, and explicit about the mental model behind Rust syntax.
- Keep the learner's existing Java, Python, and TypeScript knowledge as a bridge, but call out where the analogy breaks.
- Explain both syntax and ownership/resource-management consequences.
- Use standard Rust terminology and current idioms.
- Keep examples small enough to understand in a 30-minute session.
- Do not silently remove exercises, quizzes, answer explanations, or learning-science guidance when revising lessons.
- Use relative Markdown links that work from the file containing the link.

## Verified Rust examples

The authoritative executable examples live in `examples/`.

When adding or changing a Rust example:

1. Put the implementation in the appropriate module (`src/w1.rs` through `src/w12.rs`, `src/learn.rs`, or `src/capstone.rs`).
2. Add or update a `#[test]` that asserts the documented behavior.
3. Add an integration test under `examples/tests/` when the public API or external-consumer behavior is relevant.
4. Keep deliberately-invalid examples documented as comments with their expected compiler error; test the corrected version instead.
5. Use `#[should_panic(expected = "...")]` for intentional runtime-panic demonstrations.
6. Do not execute examples that intentionally crash the process, such as null-pointer dereferences.
7. Keep `examples/src/capstone.rs` clean under module-scoped `clippy::pedantic`; do not disable pedantic lints there without a specific, documented reason.
8. Tauri command examples may test command logic as plain Rust functions; avoid adding the full Tauri dependency to the examples crate unless system requirements and build impact are intentional.

## Required verification

Run these commands from `learn-rust/examples/` after Rust changes:

```bash
cargo fmt --check
cargo clippy --all-targets
cargo test
```

`cargo clippy --all-targets` must produce no warnings. The capstone module additionally enforces `clippy::pedantic`.

For documentation-only changes, check links and code fences manually. If a code block is intended to be executable, ensure its corresponding tested implementation remains synchronized.

## Lessons, quizzes, and spaced repetition

When changing a lesson:

- Update the corresponding quiz in `quizzes/week-NN.md` if the concept, expected output, or terminology changes.
- Keep the answer key explanatory rather than merely providing a final answer.
- Preserve the retrieval-practice pattern: questions should be answerable closed-book and should include a mix of recall, prediction, compilation reasoning, and code writing.
- Update `progress.md` only when the learning workflow or tracked concepts change.
- Keep the weekly rhythm compatible with approximately 30 minutes per day.

When adding a new core concept, consider adding:

- A lesson explanation and hands-on exercise.
- A weekend quiz question and answer.
- A spaced-repetition card in `anki/cards.tsv`.
- A tested example in `examples/`.

## Anki deck

`anki/cards.tsv` is the editable source of truth. Keep each card on one line with exactly three tab-separated fields:

```text
front<TAB>back<TAB>weekNN
```

After changing cards, regenerate the package with the project-local virtual environment:

```bash
cd anki
.venv/bin/python build_deck.py
```

The generated `anki/rust_course.apkg` should contain the same card count as `cards.tsv`. Keep Rust syntax such as `Option<T>` escaped correctly in the generated deck; `build_deck.py` handles HTML escaping and code styling.

## Capstone guidance

The capstone's intended architecture is:

- A pure Rust core library with no Tauri imports.
- A thin Tauri application layer for state, commands, IPC, and UI.
- A separate plugin crate for stats functionality.
- Tests for validation, storage round trips, search, frequency analysis, command behavior, and plugin integration.

Do not add `unsafe` to the capstone unless it is an explicitly documented stretch goal with a narrow safe wrapper and a `// SAFETY:` explanation.

## What not to do

- Do not use AI-generated code as a substitute for understanding in learner-facing exercises; the course explicitly teaches typing, predicting, and reading compiler errors.
- Do not introduce a new dependency when the standard library or an existing dependency is sufficient.
- Do not change toolchains, install global packages, deploy, push, commit, or reset repository history unless explicitly requested.
- Do not replace a tested course example with an untested Markdown-only snippet.
