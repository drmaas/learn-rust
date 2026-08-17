#!/usr/bin/env python3
"""Build the Rust course Anki deck (.apkg) from cards.tsv.

Requires genanki (a project-local venv is recommended):
    python3 -m venv .venv
    .venv/bin/pip install genanki
    .venv/bin/python build_deck.py

Output: rust_course.apkg — import into Anki via File > Import.
"""

import html
import os
import random

import genanki

HERE = os.path.dirname(os.path.abspath(__file__))
CARDS_TSV = os.path.join(HERE, "cards.tsv")
OUTPUT = os.path.join(HERE, "rust_course.apkg")

MODEL_ID = random.randrange(1 << 30, 1 << 31)
DECK_ID = random.randrange(1 << 30, 1 << 31)

CSS = """
.card {
  font-family: -apple-system, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  font-size: 18px;
  text-align: left;
  color: #222;
  background-color: #fff;
  padding: 12px;
}
code {
  font-family: "SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace;
  font-size: 0.88em;
  background-color: #f2f2f2;
  padding: 1px 5px;
  border-radius: 4px;
  color: #b3185c;
}
hr { border: 0; border-top: 1px solid #ddd; margin: 14px 0; }
"""

model = genanki.Model(
    MODEL_ID,
    "Rust Course (Basic)",
    fields=[
        {"name": "Front"},
        {"name": "Back"},
    ],
    templates=[
        {
            "name": "Card 1",
            "qfmt": "{{Front}}",
            "afmt": "{{Front}}<hr id=answer>{{Back}}",
        },
    ],
    css=CSS,
)

deck = genanki.Deck(DECK_ID, "Rust Course (12 weeks)")


def render(text: str) -> str:
    """Escape HTML so <T>, &, etc. render literally; turn `...` into <code>."""
    escaped = html.escape(text)
    parts = escaped.split("`")
    out = []
    for i, part in enumerate(parts):
        if i % 2 == 1:
            out.append(f"<code>{part}</code>")
        else:
            out.append(part)
    return "".join(out)


with open(CARDS_TSV, encoding="utf-8") as f:
    for line in f:
        line = line.rstrip("\n")
        if not line.strip():
            continue
        front, back, tag = line.split("\t")
        if front.count("`") % 2 != 0 or back.count("`") % 2 != 0:
            raise SystemExit(f"unbalanced backticks: {front[:60]!r}")
        note = genanki.Note(
            model=model,
            fields=[render(front), render(back)],
            tags=[tag],
        )
        deck.add_note(note)

package = genanki.Package(deck)
package.write_to_file(OUTPUT)

print(f"Wrote {OUTPUT}")
print(f"Cards: {len(deck.notes)}")
