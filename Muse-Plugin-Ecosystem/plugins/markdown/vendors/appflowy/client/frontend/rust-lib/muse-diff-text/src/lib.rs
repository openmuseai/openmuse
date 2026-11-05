use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Range;
use std::sync::{
  atomic::{AtomicBool, Ordering},
  Arc,
};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NewlineKind {
  Lf,
  CrLf,
  Mixed,
  None,
}

#[derive(Clone, Debug)]
pub struct TextSnapshot {
  text: Arc<str>,
  line_ranges: Arc<[Range<usize>]>,
  newline: NewlineKind,
}

impl TextSnapshot {
  pub fn new(text: String) -> Self {
    let newline = detect_newline(&text);
    let mut line_ranges = Vec::new();
    let bytes = text.as_bytes();
    let mut start = 0;
    for (index, byte) in bytes.iter().enumerate() {
      if *byte == b'\n' {
        let end = if index > start && bytes[index - 1] == b'\r' {
          index - 1
        } else {
          index
        };
        line_ranges.push(start..end);
        start = index + 1;
      }
    }
    if start < bytes.len() {
      line_ranges.push(start..bytes.len());
    }
    Self {
      text: Arc::from(text),
      line_ranges: line_ranges.into(),
      newline,
    }
  }

  pub fn line_count(&self) -> usize {
    self.line_ranges.len()
  }

  pub fn newline(&self) -> NewlineKind {
    self.newline
  }

  pub fn line(&self, index: usize) -> Option<&str> {
    self
      .line_ranges
      .get(index)
      .map(|range| &self.text[range.clone()])
  }

  pub fn viewport(&self, start: usize, count: usize) -> Vec<(usize, &str)> {
    let end = start.saturating_add(count).min(self.line_count());
    (start..end)
      .filter_map(|index| self.line(index).map(|line| (index, line)))
      .collect()
  }

  pub fn copy_lines(&self, range: Range<usize>) -> String {
    let end = range.end.min(self.line_count());
    if range.start >= end {
      return String::new();
    }
    (range.start..end)
      .filter_map(|index| self.line(index))
      .collect::<Vec<_>>()
      .join("\n")
  }

  pub fn search(&self, query: &str, case_sensitive: bool, limit: usize) -> Vec<TextMatch> {
    if query.is_empty() || limit == 0 {
      return Vec::new();
    }
    let needle = if case_sensitive {
      query.to_owned()
    } else {
      query.to_lowercase()
    };
    let mut result = Vec::new();
    for line_index in 0..self.line_count() {
      let Some(line) = self.line(line_index) else {
        continue;
      };
      let haystack = if case_sensitive {
        line.to_owned()
      } else {
        line.to_lowercase()
      };
      for (column, _) in haystack.match_indices(&needle) {
        result.push(TextMatch {
          line: line_index,
          byte_column: column,
          byte_length: needle.len(),
        });
        if result.len() >= limit {
          return result;
        }
      }
    }
    result
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextMatch {
  pub line: usize,
  pub byte_column: usize,
  pub byte_length: usize,
}

#[derive(Clone, Debug)]
pub struct DiffOptions {
  pub max_duration: Duration,
  pub max_trace_bytes: usize,
}

impl Default for DiffOptions {
  fn default() -> Self {
    Self {
      max_duration: Duration::from_secs(10),
      max_trace_bytes: 256 * 1024 * 1024,
    }
  }
}

#[derive(Clone, Default)]
pub struct CancellationToken(Arc<AtomicBool>);

impl CancellationToken {
  pub fn cancel(&self) {
    self.0.store(true, Ordering::Release);
  }

  pub fn is_cancelled(&self) -> bool {
    self.0.load(Ordering::Acquire)
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiffQuality {
  Exact,
  BudgetExceeded,
  Cancelled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChangeKind {
  Insert,
  Delete,
  Replace,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeBlock {
  pub kind: ChangeKind,
  pub old_start: usize,
  pub old_count: usize,
  pub new_start: usize,
  pub new_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimilarBoundary {
  pub old_line: usize,
  pub new_line: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffResult {
  pub quality: DiffQuality,
  pub blocks: Vec<ChangeBlock>,
  pub similar_boundaries: Vec<SimilarBoundary>,
  pub additions: usize,
  pub deletions: usize,
  pub before_newline: NewlineKind,
  pub after_newline: NewlineKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EditKind {
  Equal,
  Delete,
  Insert,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Edit {
  kind: EditKind,
  old_index: Option<usize>,
  new_index: Option<usize>,
}

pub fn compare(
  before: &TextSnapshot,
  after: &TextSnapshot,
  options: &DiffOptions,
  cancellation: &CancellationToken,
) -> DiffResult {
  let started_at = Instant::now();
  let before_lines = (0..before.line_count())
    .filter_map(|index| before.line(index))
    .collect::<Vec<_>>();
  let after_lines = (0..after.line_count())
    .filter_map(|index| after.line(index))
    .collect::<Vec<_>>();
  let computation = myers(
    &before_lines,
    &after_lines,
    options,
    cancellation,
    started_at,
  );
  let (quality, edits) = match computation {
    Ok(edits) => (DiffQuality::Exact, edits),
    Err(DiffQuality::Cancelled) => (DiffQuality::Cancelled, Vec::new()),
    Err(_) => (
      DiffQuality::BudgetExceeded,
      coarse_fallback(before.line_count(), after.line_count()),
    ),
  };
  let additions = edits.iter().filter(|edit| edit.kind == EditKind::Insert).count();
  let deletions = edits.iter().filter(|edit| edit.kind == EditKind::Delete).count();
  let blocks = change_blocks(&edits);
  DiffResult {
    quality,
    similar_boundaries: similar_boundaries(&blocks, before.line_count(), after.line_count()),
    blocks,
    additions,
    deletions,
    before_newline: before.newline(),
    after_newline: after.newline(),
  }
}

fn myers(
  before: &[&str],
  after: &[&str],
  options: &DiffOptions,
  cancellation: &CancellationToken,
  started_at: Instant,
) -> Result<Vec<Edit>, DiffQuality> {
  if before.is_empty() {
    return Ok((0..after.len())
      .map(|new_index| Edit {
        kind: EditKind::Insert,
        old_index: None,
        new_index: Some(new_index),
      })
      .collect());
  }
  if after.is_empty() {
    return Ok((0..before.len())
      .map(|old_index| Edit {
        kind: EditKind::Delete,
        old_index: Some(old_index),
        new_index: None,
      })
      .collect());
  }

  let max = before.len().saturating_add(after.len());
  let mut frontier = HashMap::<isize, isize>::from([(1, 0)]);
  let mut trace = Vec::<HashMap<isize, isize>>::new();
  for distance in 0..=max {
    if cancellation.is_cancelled() {
      return Err(DiffQuality::Cancelled);
    }
    if started_at.elapsed() > options.max_duration {
      return Err(DiffQuality::BudgetExceeded);
    }
    let projected_bytes = trace
      .len()
      .saturating_add(1)
      .saturating_mul(frontier.len().saturating_add(1))
      .saturating_mul(std::mem::size_of::<(isize, isize)>() * 2);
    if projected_bytes > options.max_trace_bytes {
      return Err(DiffQuality::BudgetExceeded);
    }
    trace.push(frontier.clone());
    let mut next = frontier.clone();
    let distance = distance as isize;
    let mut diagonal = -distance;
    while diagonal <= distance {
      let down = diagonal == -distance
        || (diagonal != distance
          && frontier.get(&(diagonal - 1)).copied().unwrap_or(-1)
            < frontier.get(&(diagonal + 1)).copied().unwrap_or(-1));
      let mut x = if down {
        frontier.get(&(diagonal + 1)).copied().unwrap_or(0)
      } else {
        frontier.get(&(diagonal - 1)).copied().unwrap_or(0) + 1
      };
      let mut y = x - diagonal;
      while x >= 0
        && y >= 0
        && (x as usize) < before.len()
        && (y as usize) < after.len()
        && before[x as usize] == after[y as usize]
      {
        x += 1;
        y += 1;
      }
      next.insert(diagonal, x);
      if x as usize >= before.len() && y as usize >= after.len() {
        trace.push(next);
        return Ok(backtrack(&trace, before.len(), after.len()));
      }
      diagonal += 2;
    }
    frontier = next;
  }
  Err(DiffQuality::BudgetExceeded)
}

fn backtrack(
  trace: &[HashMap<isize, isize>],
  before_len: usize,
  after_len: usize,
) -> Vec<Edit> {
  let mut x = before_len as isize;
  let mut y = after_len as isize;
  let mut result = Vec::new();
  for distance in (0..trace.len().saturating_sub(1)).rev() {
    let frontier = &trace[distance];
    let diagonal = x - y;
    let d = distance as isize;
    let down = diagonal == -d
      || (diagonal != d
        && frontier.get(&(diagonal - 1)).copied().unwrap_or(-1)
          < frontier.get(&(diagonal + 1)).copied().unwrap_or(-1));
    let previous_diagonal = if down { diagonal + 1 } else { diagonal - 1 };
    let previous_x = frontier.get(&previous_diagonal).copied().unwrap_or(0);
    let previous_y = previous_x - previous_diagonal;
    while x > previous_x && y > previous_y {
      x -= 1;
      y -= 1;
      result.push(Edit {
        kind: EditKind::Equal,
        old_index: Some(x as usize),
        new_index: Some(y as usize),
      });
    }
    if distance == 0 {
      break;
    }
    if down {
      y -= 1;
      result.push(Edit {
        kind: EditKind::Insert,
        old_index: None,
        new_index: Some(y as usize),
      });
    } else {
      x -= 1;
      result.push(Edit {
        kind: EditKind::Delete,
        old_index: Some(x as usize),
        new_index: None,
      });
    }
  }
  result.reverse();
  result
}

fn coarse_fallback(before_len: usize, after_len: usize) -> Vec<Edit> {
  let mut result = Vec::with_capacity(before_len.saturating_add(after_len));
  result.extend((0..before_len).map(|old_index| Edit {
    kind: EditKind::Delete,
    old_index: Some(old_index),
    new_index: None,
  }));
  result.extend((0..after_len).map(|new_index| Edit {
    kind: EditKind::Insert,
    old_index: None,
    new_index: Some(new_index),
  }));
  result
}

fn change_blocks(edits: &[Edit]) -> Vec<ChangeBlock> {
  let mut result = Vec::new();
  let mut old_cursor = 0;
  let mut new_cursor = 0;
  let mut index = 0;
  while index < edits.len() {
    if edits[index].kind == EditKind::Equal {
      old_cursor += 1;
      new_cursor += 1;
      index += 1;
      continue;
    }
    let old_start = old_cursor;
    let new_start = new_cursor;
    let mut old_count = 0;
    let mut new_count = 0;
    while index < edits.len() && edits[index].kind != EditKind::Equal {
      match edits[index].kind {
        EditKind::Delete => {
          old_count += 1;
          old_cursor += 1;
        },
        EditKind::Insert => {
          new_count += 1;
          new_cursor += 1;
        },
        EditKind::Equal => {},
      }
      index += 1;
    }
    result.push(ChangeBlock {
      kind: change_kind(old_count, new_count),
      old_start,
      old_count,
      new_start,
      new_count,
    });
  }
  result
}

fn change_kind(old_count: usize, new_count: usize) -> ChangeKind {
  match (old_count, new_count) {
    (0, _) => ChangeKind::Insert,
    (_, 0) => ChangeKind::Delete,
    _ => ChangeKind::Replace,
  }
}

fn similar_boundaries(
  blocks: &[ChangeBlock],
  before_len: usize,
  after_len: usize,
) -> Vec<SimilarBoundary> {
  let mut result = vec![SimilarBoundary {
    old_line: 0,
    new_line: 0,
  }];
  for block in blocks {
    result.push(SimilarBoundary {
      old_line: block.old_start,
      new_line: block.new_start,
    });
    result.push(SimilarBoundary {
      old_line: block.old_start.saturating_add(block.old_count),
      new_line: block.new_start.saturating_add(block.new_count),
    });
  }
  result.push(SimilarBoundary {
    old_line: before_len,
    new_line: after_len,
  });
  result.dedup();
  result
}

fn detect_newline(text: &str) -> NewlineKind {
  let crlf = text.matches("\r\n").count();
  let lf = text.matches('\n').count().saturating_sub(crlf);
  match (crlf, lf) {
    (0, 0) => NewlineKind::None,
    (0, _) => NewlineKind::Lf,
    (_, 0) => NewlineKind::CrLf,
    _ => NewlineKind::Mixed,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn indexes_unicode_viewports_and_newlines() {
    let snapshot = TextSnapshot::new("你好\r\nemoji 👩🏽‍💻\r\nend".to_owned());
    assert_eq!(snapshot.newline(), NewlineKind::CrLf);
    assert_eq!(snapshot.line_count(), 3);
    assert_eq!(snapshot.line(1), Some("emoji 👩🏽‍💻"));
    assert_eq!(snapshot.copy_lines(1..3), "emoji 👩🏽‍💻\nend");
  }

  #[test]
  fn emits_compact_exact_change_blocks() {
    let before = TextSnapshot::new("one\ntwo\nthree\n".to_owned());
    let after = TextSnapshot::new("one\nTWO\nthree\nfour\n".to_owned());
    let result = compare(
      &before,
      &after,
      &DiffOptions::default(),
      &CancellationToken::default(),
    );
    assert_eq!(result.quality, DiffQuality::Exact);
    assert_eq!(result.additions, 2);
    assert_eq!(result.deletions, 1);
    assert_eq!(
      result.blocks,
      vec![
        ChangeBlock {
          kind: ChangeKind::Replace,
          old_start: 1,
          old_count: 1,
          new_start: 1,
          new_count: 1,
        },
        ChangeBlock {
          kind: ChangeKind::Insert,
          old_start: 3,
          old_count: 0,
          new_start: 3,
          new_count: 1,
        },
      ],
    );
    assert_eq!(
      result.similar_boundaries.first(),
      Some(&SimilarBoundary {
        old_line: 0,
        new_line: 0,
      })
    );
    assert_eq!(
      result.similar_boundaries.last(),
      Some(&SimilarBoundary {
        old_line: 3,
        new_line: 4,
      })
    );
  }

  #[test]
  fn serialized_result_is_compact_coordinates_only() {
    let before = TextSnapshot::new("UNIQUE_BEFORE_LINE_α\nkeep\n".to_owned());
    let after = TextSnapshot::new("UNIQUE_AFTER_LINE_β\nkeep\nextra\n".to_owned());
    let result = compare(
      &before,
      &after,
      &DiffOptions::default(),
      &CancellationToken::default(),
    );
    let json = serde_json::to_value(&result).expect("json");
    let object = json.as_object().expect("object");
    let allowed = [
      "quality",
      "blocks",
      "similarBoundaries",
      "additions",
      "deletions",
      "beforeNewline",
      "afterNewline",
    ];
    for key in object.keys() {
      assert!(allowed.contains(&key.as_str()), "unexpected field {key}");
    }
    let rendered = json.to_string();
    assert!(!rendered.contains("UNIQUE_BEFORE_LINE"));
    assert!(!rendered.contains("UNIQUE_AFTER_LINE"));
    assert!(!rendered.contains("keep"));
    assert!(!rendered.contains("extra"));
  }

  #[test]
  fn change_blocks_cover_equal_spans() {
    let before = TextSnapshot::new("a\nb\nc\nd\n".to_owned());
    let after = TextSnapshot::new("a\nB\nc\nD\n".to_owned());
    let result = compare(
      &before,
      &after,
      &DiffOptions::default(),
      &CancellationToken::default(),
    );
    let mut old_cursor = 0;
    let mut new_cursor = 0;
    for block in &result.blocks {
      for offset in 0..(block.old_start - old_cursor) {
        assert_eq!(
          before.line(old_cursor + offset),
          after.line(new_cursor + offset)
        );
      }
      old_cursor = block.old_start + block.old_count;
      new_cursor = block.new_start + block.new_count;
    }
    while old_cursor < before.line_count() && new_cursor < after.line_count() {
      assert_eq!(before.line(old_cursor), after.line(new_cursor));
      old_cursor += 1;
      new_cursor += 1;
    }
  }

  #[test]
  fn twenty_thousand_line_replace_stays_exact() {
    let before = (0..20_000)
      .map(|index| format!("line-{index}"))
      .collect::<Vec<_>>()
      .join("\n");
    let mut after_lines = (0..20_000)
      .map(|index| format!("line-{index}"))
      .collect::<Vec<_>>();
    after_lines[1024] = "changed-1024".to_owned();
    let started = Instant::now();
    let result = compare(
      &TextSnapshot::new(before),
      &TextSnapshot::new(after_lines.join("\n")),
      &DiffOptions::default(),
      &CancellationToken::default(),
    );
    assert_eq!(result.quality, DiffQuality::Exact);
    assert_eq!(result.blocks.len(), 1);
    assert!(started.elapsed() < Duration::from_secs(5));
  }

  #[test]
  fn honours_cancellation_and_budget() {
    let before = TextSnapshot::new("a\nb\nc\n".to_owned());
    let after = TextSnapshot::new("x\ny\nz\n".to_owned());
    let token = CancellationToken::default();
    token.cancel();
    let cancelled = compare(&before, &after, &DiffOptions::default(), &token);
    assert_eq!(cancelled.quality, DiffQuality::Cancelled);

    let limited = compare(
      &before,
      &after,
      &DiffOptions {
        max_duration: Duration::from_secs(1),
        max_trace_bytes: 1,
      },
      &CancellationToken::default(),
    );
    assert_eq!(limited.quality, DiffQuality::BudgetExceeded);
    assert_eq!(limited.blocks.len(), 1);
  }

  #[test]
  fn search_is_bounded() {
    let snapshot = TextSnapshot::new("Muse\nopenmuse\nMUSE".to_owned());
    let matches = snapshot.search("muse", false, 2);
    assert_eq!(matches.len(), 2);
    assert_eq!(matches[0].line, 0);
    assert_eq!(matches[1].line, 1);
  }
}
