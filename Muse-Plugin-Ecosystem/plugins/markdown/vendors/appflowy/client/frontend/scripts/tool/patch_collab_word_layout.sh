#!/usr/bin/env bash
# Re-apply ViewLayout office blob variants onto the cargo git checkout of
# AppFlowy-Collab rev 4dfccef. Cargo rebuilds wipe this if the checkout is refreshed.
# Word=9, Excel=10, Slides=11, Pdf=12. Never reuse 0–8.
set -euo pipefail
CHECKOUT="${CARGO_HOME:-$HOME/.cargo}/git/checkouts/appflowy-collab-71d77b6db06446ab/4dfccef/collab-folder/src/view.rs"
if [[ ! -f "$CHECKOUT" ]]; then
  echo "collab-folder checkout not found: $CHECKOUT" >&2
  exit 1
fi
python3 - "$CHECKOUT" <<'PY'
from pathlib import Path
import sys
p = Path(sys.argv[1])
t = p.read_text()
if "Excel = 10" in t and "Slides = 11" in t and "Pdf = 12" in t and "Word = 9" in t:
    print("already patched:", p)
    raise SystemExit(0)
old = """pub enum ViewLayout {
  Document = 0,
  Grid = 1,
  Board = 2,
  Calendar = 3,
  Chat = 4,
}"""
if "Word = 9" in t:
    old = """pub enum ViewLayout {
  Document = 0,
  Grid = 1,
  Board = 2,
  Calendar = 3,
  Chat = 4,
  Word = 9,
}"""
new = """pub enum ViewLayout {
  Document = 0,
  Grid = 1,
  Board = 2,
  Calendar = 3,
  Chat = 4,
  Word = 9,
  Excel = 10,
  Slides = 11,
  Pdf = 12,
}"""
if old not in t:
    raise SystemExit(f"enum block not found in {p}")
t = t.replace(old, new, 1)
if "10 => Ok(ViewLayout::Excel)" not in t:
    t = t.replace(
"""      4 => Ok(ViewLayout::Chat),
      9 => Ok(ViewLayout::Word),
      _ => bail!("Unknown layout {}", value),""",
"""      4 => Ok(ViewLayout::Chat),
      9 => Ok(ViewLayout::Word),
      10 => Ok(ViewLayout::Excel),
      11 => Ok(ViewLayout::Slides),
      12 => Ok(ViewLayout::Pdf),
      _ => bail!("Unknown layout {}", value),""",
    1)
    t = t.replace(
"""      4 => Ok(ViewLayout::Chat),
      _ => bail!("Unknown layout {}", value),""",
"""      4 => Ok(ViewLayout::Chat),
      9 => Ok(ViewLayout::Word),
      10 => Ok(ViewLayout::Excel),
      11 => Ok(ViewLayout::Slides),
      12 => Ok(ViewLayout::Pdf),
      _ => bail!("Unknown layout {}", value),""",
    1)
if "fn is_office_blob" not in t:
    t = t.replace(
"""  pub fn is_database(&self) -> bool {
    matches!(
      self,
      ViewLayout::Grid | ViewLayout::Board | ViewLayout::Calendar
    )
  }
}""",
"""  pub fn is_database(&self) -> bool {
    matches!(
      self,
      ViewLayout::Grid | ViewLayout::Board | ViewLayout::Calendar
    )
  }

  /// Local office blobs. Not Document CRDT. Values 9–12.
  pub fn is_office_blob(&self) -> bool {
    matches!(
      self,
      ViewLayout::Word | ViewLayout::Excel | ViewLayout::Slides | ViewLayout::Pdf
    )
  }
}""",
    1)
p.write_text(t)
print("patched", p)
PY
