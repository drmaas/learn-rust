# Rust Course — Anki Deck

A flashcard deck built from the course's spaced-repetition concepts and quiz bank: **~115 atomic cards** across all 12 weeks, each tagged (`week01` … `week12`).

Cards follow good Anki practice: one fact per card, question on the front, concise answer on the back, with code styled in monospace.

---

## How to get the deck into Anki

### Option A — import the ready-made `.apkg` (easiest)

If `rust_course.apkg` exists in this folder:

1. Open Anki.
2. **File → Import** → select `rust_course.apkg`.
3. It lands in a deck named **"Rust Course (12 weeks)"** with all tags.

If the `.apkg` isn't there yet, generate it (see "Regenerating" below) or use Option B.

### Option B — import `cards.tsv` directly (no tools needed)

`cards.tsv` is a plain tab-separated file Anki imports natively:

1. Open Anki.
2. **File → Import** → select `cards.tsv`.
3. In the import dialog, set: **Type** = `Basic`, **Deck** = your Rust deck, **Fields separated by** = Tab.
4. Import. (The third column — the `weekNN` tag — appears as a third field; you can map it to a "Tags" field if you add one to the note type, or just ignore it.)

---

## The spaced-repetition workflow (ties into the course)

- **When:** after each week's weekend checkpoint, review that week's tagged cards (search `tag:week01` in Anki's browser).
- **How:** do the cards normally — Anki's scheduler *is* spaced repetition, so it handles the +1/+3/+7/+14-day intervals for you.
- **Missed quiz questions:** if you miss a card, open the corresponding [week quiz](../quizzes/README.md) for the deeper explanation.

---

## Regenerating the deck

If you edit `cards.tsv` (add your own cards, tweak wording) and want a fresh `.apkg`:

```bash
cd learn-rust/anki
python3 -m venv .venv
.venv/bin/pip install genanki
.venv/bin/python build_deck.py
```

The script reads `cards.tsv` and writes `rust_course.apkg`. It's idempotent — safe to re-run any time.

---

## File layout

```
anki/
├── README.md          ← you are here
├── cards.tsv          ← the source of truth (front, back, weekNN tag) — editable
├── build_deck.py      ← genanki script that packs cards.tsv into the .apkg
└── rust_course.apkg   ← the ready-to-import deck (generated)
```

**Adding your own cards:** append lines to `cards.tsv` in the same `front\tback\tweekNN` format, then regenerate. Cards are one per line; keep each front/back on a single line (no tabs or newlines inside a field).
