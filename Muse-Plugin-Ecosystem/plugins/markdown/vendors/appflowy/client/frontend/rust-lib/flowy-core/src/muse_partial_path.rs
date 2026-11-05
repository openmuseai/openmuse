//! Bounded, deterministic fuzzy resolution of a **partial** Mount-relative path.
//!
//! DSH hands the Host a locator in the Host's own coordinates — a `mountRef` and
//! a Mount-relative POSIX path — and that path is not always complete. A
//! conversation can cite `architecture/08-view-support.md` when the file lives
//! deeper, or cite nothing but `08-view-support.md`. The correct answer is never
//! "open something that looks close": it is either the one file the partial path
//! provably denotes, or a visible refusal.
//!
//! # What this module does
//!
//! Given the directory the Host granted for a Mount, and the partial path DSH
//! supplied, it returns one of:
//!
//! - [`PartialPathResolution::Exact`] — the supplied path already names a file
//!   inside the Mount. The caller must dispatch it **unchanged**; fuzzy matching
//!   is a strict fallback and never rewrites an exact hit.
//! - [`PartialPathResolution::Resolved`] — exactly one file matched, and the
//!   caller dispatches the **resolved** Mount-relative path instead.
//! - [`PartialPathResolution::Ambiguous`] — several files matched equally well.
//!   The caller must fail closed and report the candidate count; this module
//!   never picks the first hit.
//! - [`PartialPathResolution::NotFound`] / [`PartialPathResolution::Unavailable`]
//!   — nothing matched, or the Mount directory is unusable. The caller keeps the
//!   supplied path, so the surface answers its own honest not-found.
//!
//! # Matching order and ranking
//!
//! 1. the whole supplied path, as a file inside the Mount (exact);
//! 2. **suffix match**: the candidate's segments end with *all* the supplied
//!    segments (`architecture/08-view-support.md` matches
//!    `docs/architecture/08-view-support.md`, and `…/xarchitecture/08-view-support.md`
//!    does not, because the match is segment-wise, never a raw string suffix);
//! 3. **basename match**: the candidate's file name is the supplied file name,
//!    even though the supplied middle segments are wrong — the tier that answers
//!    "the supplied value contains just the file name".
//!
//! A better tier always wins. Inside a tier the candidate with **fewer extra
//! leading segments** wins, so the shallowest real file is preferred. Candidates
//! that tie on `(tier, extra segments)` are *equally good*: they produce
//! [`PartialPathResolution::Ambiguous`], never a guess. Lexicographic order is
//! used only to keep the reported sample stable, never to break a tie.
//!
//! Matching is case-sensitive and byte-exact, which is what makes the ranking a
//! total order over the candidate set rather than a platform-dependent one.
//!
//! # Bounds and containment
//!
//! Every walk is bounded by [`PartialPathLimits`]: depth, directory entries read,
//! and collected candidates. A huge Mount cannot hang the Host, and a truncated
//! walk **fails closed** as soon as it has any candidate, because the cut could
//! have hidden an equally good file. At most two bounded passes run: the second
//! one includes the heavy directories and only happens when the first found
//! nothing.
//!
//! Only files that are really inside the granted Mount are ever offered:
//! directories are canonicalized before they are entered, files are
//! canonicalized before they are accepted, and a symlink that leaves the Mount
//! is refused. Directory cycles are cut by a visited set of canonical paths.
//! Heavy directories (`.git`, `node_modules`, build outputs, …) are skipped on
//! the first pass and visited only when that pass found nothing else.

use std::{
  collections::HashSet,
  path::{Path, PathBuf},
};

/// Directory names a first pass skips, because they hold build output or VCS
/// metadata rather than the documents a reader cites.
pub(crate) const HEAVY_DIRECTORIES: &[&str] = &[
  ".git",
  ".hg",
  ".svn",
  ".gradle",
  ".dart_tool",
  ".venv",
  "__pycache__",
  ".mypy_cache",
  ".pytest_cache",
  ".next",
  "node_modules",
  "target",
  "build",
  "dist",
  "out",
  "coverage",
  "vendor",
];

/// Bound on one fuzzy search. `Default` is the production bound; tests use a
/// smaller one to prove the bounds hold without building a real 20k-entry tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PartialPathLimits {
  /// Deepest directory relative to the Mount root that is entered; a file
  /// directly in the root has one segment.
  pub max_depth: usize,
  /// Directory entries read by one pass.
  pub max_entries: usize,
  /// Candidate files collected before the search stops and fails closed.
  pub max_candidates: usize,
}

impl Default for PartialPathLimits {
  fn default() -> Self {
    Self {
      max_depth: 12,
      max_entries: 20_000,
      max_candidates: 32,
    }
  }
}

/// How a candidate matched the supplied partial path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PartialPathMatch {
  /// The candidate's segments end with every supplied segment.
  Suffix,
  /// The candidate's file name is the supplied file name.
  Basename,
}

/// Outcome of one fuzzy resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PartialPathResolution {
  /// The supplied path names a file: dispatch it unchanged.
  Exact(String),
  /// Dispatch this Mount-relative path instead of the supplied one.
  Resolved {
    relative_path: String,
    matched: PartialPathMatch,
    /// Leading segments the resolved path adds, the ranking key.
    extra_segments: usize,
  },
  /// Equally good candidates: fail closed and name the count.
  Ambiguous {
    candidates: usize,
    /// Up to [`MAX_REPORTED_CANDIDATES`] Mount-relative paths, ordered.
    sample: Vec<String>,
    /// True when a bound cut the search short, so `candidates` is a lower bound.
    truncated: bool,
  },
  /// No file matched; the caller keeps the supplied path.
  NotFound,
  /// The granted Mount directory is missing or is not a directory.
  Unavailable,
}

/// How many candidate paths an ambiguous answer reports back.
pub(crate) const MAX_REPORTED_CANDIDATES: usize = 4;

/// Resolve one partial Mount-relative path inside the granted Mount.
///
/// `partial` must already be a Mount-relative POSIX path (no leading `/`, no
/// drive letter, no `..`, no backslash); this module re-checks every segment
/// anyway, so a path-shaped value can never reach the filesystem.
pub(crate) fn resolve_partial_path(root: &Path, partial: &str) -> PartialPathResolution {
  resolve_partial_path_with(root, partial, PartialPathLimits::default())
}

/// [`resolve_partial_path`] with explicit bounds.
pub(crate) fn resolve_partial_path_with(
  root: &Path,
  partial: &str,
  limits: PartialPathLimits,
) -> PartialPathResolution {
  let Some(canonical_root) = canonical_directory(root) else {
    return PartialPathResolution::Unavailable;
  };
  let wanted: Vec<&str> = if partial.is_empty() {
    Vec::new()
  } else {
    partial.split('/').collect()
  };
  if wanted.is_empty() || !wanted.iter().all(|segment| is_safe_segment(segment)) {
    // The declared schema and the provider already refuse these shapes; never
    // search for them, not even as a fallback.
    return PartialPathResolution::NotFound;
  }
  let exact = join_segments(&canonical_root, &wanted);
  if is_contained_file(&canonical_root, &exact) {
    return PartialPathResolution::Exact(partial.to_string());
  }

  let mut scan = Scan::new(&canonical_root, &wanted, limits);
  scan.run(true);
  if scan.candidates.is_empty() && scan.skipped_heavy {
    // Nothing outside the heavy directories matched; look inside them too.
    scan.run(false);
  }
  scan.finish()
}

fn join_segments(root: &Path, segments: &[&str]) -> PathBuf {
  let mut path = root.to_path_buf();
  for segment in segments {
    path.push(segment);
  }
  path
}

fn canonical_directory(path: &Path) -> Option<PathBuf> {
  let canonical = std::fs::canonicalize(path).ok()?;
  std::fs::metadata(&canonical)
    .ok()
    .filter(|metadata| metadata.is_dir())
    .map(|_| canonical)
}

/// Whether `path` is an existing **file** whose canonical location stays inside
/// `root`. This is the one containment check every offered path passes.
fn is_contained_file(root: &Path, path: &Path) -> bool {
  let Ok(canonical) = std::fs::canonicalize(path) else {
    return false;
  };
  canonical.starts_with(root)
    && std::fs::metadata(&canonical)
      .map(|metadata| metadata.is_file())
      .unwrap_or(false)
}

/// A Mount-relative segment this module will ever look for.
fn is_safe_segment(segment: &str) -> bool {
  !segment.is_empty()
    && segment != "."
    && segment != ".."
    && !segment.contains('/')
    && !segment.contains('\\')
    && !segment.contains(':')
    && !segment.chars().any(char::is_control)
}

/// One file that matched the supplied partial path.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Candidate {
  /// 0 = suffix match, 1 = basename match.
  tier: u8,
  /// Leading segments the candidate adds, the ranking key inside a tier.
  extra_segments: usize,
  relative_path: String,
}

const TIER_SUFFIX: u8 = 0;
const TIER_BASENAME: u8 = 1;

/// One bounded walk of the granted Mount.
struct Scan<'a> {
  root: &'a Path,
  wanted: &'a [&'a str],
  limits: PartialPathLimits,
  candidates: Vec<Candidate>,
  /// Entries read by the pass in flight.
  entries: usize,
  /// The pass in flight was cut short by a bound.
  pass_truncated: bool,
  /// Any pass was cut short by a bound.
  truncated: bool,
  /// The candidate bound was reached: stop the whole walk, not just this branch.
  saturated: bool,
  skipped_heavy: bool,
  visited: HashSet<PathBuf>,
}

impl<'a> Scan<'a> {
  fn new(root: &'a Path, wanted: &'a [&'a str], limits: PartialPathLimits) -> Self {
    Self {
      root,
      wanted,
      limits,
      candidates: Vec::new(),
      entries: 0,
      pass_truncated: false,
      truncated: false,
      saturated: false,
      skipped_heavy: false,
      visited: HashSet::new(),
    }
  }

  /// Run one pass, with its own entry budget. `skip_heavy` keeps
  /// [`HEAVY_DIRECTORIES`] out of it.
  fn run(&mut self, skip_heavy: bool) {
    self.entries = 0;
    self.pass_truncated = false;
    self.visited.clear();
    self.visited.insert(self.root.to_path_buf());
    self.walk(self.root, 0, skip_heavy);
    self.truncated |= self.pass_truncated;
  }

  fn walk(&mut self, directory: &Path, depth: usize, skip_heavy: bool) {
    if self.saturated || self.entries >= self.limits.max_entries {
      self.pass_truncated = true;
      return;
    }
    let Ok(read) = std::fs::read_dir(directory) else {
      return;
    };
    let mut entries = Vec::new();
    for entry in read {
      self.entries += 1;
      if self.entries > self.limits.max_entries {
        self.pass_truncated = true;
        break;
      }
      // A directory whose entries cannot be read is skipped, never fatal.
      if let Ok(entry) = entry {
        entries.push(entry);
      }
    }
    // Sorted, so a truncated scan is deterministic: the same Mount and the same
    // partial path always cut at exactly the same place.
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
      if self.saturated {
        self.pass_truncated = true;
        return;
      }
      let Ok(name) = entry.file_name().into_string() else {
        continue;
      };
      let path = entry.path();
      let Ok(file_type) = entry.file_type() else {
        continue;
      };
      // `file_type` does not follow links, so a symlink is classified by its
      // target here and re-checked for containment below either way.
      let (is_directory, is_file) = if file_type.is_symlink() {
        match std::fs::metadata(&path) {
          Ok(metadata) => (metadata.is_dir(), metadata.is_file()),
          Err(_) => (false, false),
        }
      } else {
        (file_type.is_dir(), file_type.is_file())
      };

      if is_directory {
        if depth + 1 > self.limits.max_depth {
          continue;
        }
        if skip_heavy && HEAVY_DIRECTORIES.contains(&name.as_str()) {
          self.skipped_heavy = true;
          continue;
        }
        let Some(canonical) = canonical_directory(&path) else {
          continue;
        };
        if !canonical.starts_with(self.root) {
          continue;
        }
        if !self.visited.insert(canonical.clone()) {
          continue;
        }
        self.walk(&canonical, depth + 1, skip_heavy);
        continue;
      }

      if !is_file {
        continue;
      }
      let candidate_segments = self.segments_of(directory, &name);
      let Some((tier, extra_segments)) = classify(&candidate_segments, self.wanted) else {
        continue;
      };
      if !is_contained_file(self.root, &path) {
        continue;
      }
      self.candidates.push(Candidate {
        tier,
        extra_segments,
        relative_path: candidate_segments.join("/"),
      });
      if self.candidates.len() >= self.limits.max_candidates {
        self.pass_truncated = true;
        self.saturated = true;
        return;
      }
    }
  }

  /// Mount-relative segments of `directory/name`, all of them real directory
  /// entries below the granted root.
  fn segments_of(&self, directory: &Path, name: &str) -> Vec<String> {
    let mut segments: Vec<String> = directory
      .strip_prefix(self.root)
      .map(|relative| {
        relative
          .components()
          .map(|component| component.as_os_str().to_string_lossy().into_owned())
          .collect()
      })
      .unwrap_or_default();
    segments.push(name.to_string());
    segments
  }

  fn finish(mut self) -> PartialPathResolution {
    if self.candidates.is_empty() {
      return PartialPathResolution::NotFound;
    }
    if self.truncated {
      // A bound was hit with candidates already found: another file could be
      // equally good or better, so never pick one.
      return PartialPathResolution::Ambiguous {
        candidates: self.candidates.len(),
        sample: report_sample(&mut self.candidates, None),
        truncated: true,
      };
    }
    let best = self
      .candidates
      .iter()
      .map(|candidate| (candidate.tier, candidate.extra_segments))
      .min()
      .expect("a non-empty candidate set has a minimum");
    let equally_good = self
      .candidates
      .iter()
      .filter(|candidate| (candidate.tier, candidate.extra_segments) == best)
      .count();
    if equally_good > 1 {
      return PartialPathResolution::Ambiguous {
        candidates: equally_good,
        sample: report_sample(&mut self.candidates, Some(best)),
        truncated: false,
      };
    }
    let winner = self
      .candidates
      .iter()
      .find(|candidate| (candidate.tier, candidate.extra_segments) == best)
      .expect("exactly one winner");
    PartialPathResolution::Resolved {
      relative_path: winner.relative_path.clone(),
      matched: if winner.tier == TIER_SUFFIX {
        PartialPathMatch::Suffix
      } else {
        PartialPathMatch::Basename
      },
      extra_segments: winner.extra_segments,
    }
  }
}

/// Deterministic, ordered sample of the candidates a scan found. `only` keeps
/// the sample to the equally good rank the ambiguity is about.
fn report_sample(candidates: &mut [Candidate], only: Option<(u8, usize)>) -> Vec<String> {
  candidates.sort_by(|left, right| {
    (left.tier, left.extra_segments, &left.relative_path).cmp(&(
      right.tier,
      right.extra_segments,
      &right.relative_path,
    ))
  });
  candidates
    .iter()
    .filter(|candidate| only.is_none_or(|key| (candidate.tier, candidate.extra_segments) == key))
    .take(MAX_REPORTED_CANDIDATES)
    .map(|candidate| candidate.relative_path.clone())
    .collect()
}

/// `(tier, extra_segments)` when `candidate` matches `wanted`.
///
/// A single-segment `wanted` is a file name, so the suffix tier already covers
/// it and the basename tier never fires for it.
fn classify(candidate: &[String], wanted: &[&str]) -> Option<(u8, usize)> {
  let name = candidate.last()?;
  if candidate.len() >= wanted.len()
    && candidate[candidate.len() - wanted.len()..]
      .iter()
      .map(String::as_str)
      .eq(wanted.iter().copied())
  {
    return Some((TIER_SUFFIX, candidate.len() - wanted.len()));
  }
  if *name == wanted[wanted.len() - 1] {
    return Some((TIER_BASENAME, candidate.len() - 1));
  }
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  /// A temporary directory tree that removes itself when the test ends.
  struct TempTree {
    root: PathBuf,
  }

  impl TempTree {
    fn new(name: &str) -> Self {
      let root = std::env::temp_dir().join(format!("muse-partial-{name}-{}", uuid::Uuid::new_v4()));
      std::fs::create_dir_all(&root).expect("temp root");
      Self { root }
    }

    fn with(name: &str, files: &[&str]) -> Self {
      let tree = Self::new(name);
      for file in files {
        tree.file(file);
      }
      tree
    }

    fn file(&self, relative: &str) -> PathBuf {
      let path = self.path(relative);
      std::fs::create_dir_all(path.parent().expect("parent")).expect("temp parent");
      std::fs::write(&path, "x").expect("temp file");
      path
    }

    fn path(&self, relative: &str) -> PathBuf {
      let mut path = self.root.clone();
      for segment in relative.split('/') {
        path.push(segment);
      }
      path
    }
  }

  impl Drop for TempTree {
    fn drop(&mut self) {
      let _ = std::fs::remove_dir_all(&self.root);
    }
  }

  fn resolved(resolution: PartialPathResolution) -> (String, PartialPathMatch, usize) {
    match resolution {
      PartialPathResolution::Resolved {
        relative_path,
        matched,
        extra_segments,
      } => (relative_path, matched, extra_segments),
      other => panic!("expected a resolution, got {:?}", other),
    }
  }

  fn ambiguous(resolution: PartialPathResolution) -> (usize, Vec<String>, bool) {
    match resolution {
      PartialPathResolution::Ambiguous {
        candidates,
        sample,
        truncated,
      } => (candidates, sample, truncated),
      other => panic!("expected an ambiguous answer, got {:?}", other),
    }
  }

  #[test]
  fn an_exact_hit_is_never_rewritten() {
    let tree = TempTree::with(
      "exact",
      &["docs/architecture/08-view-support.md", "docs/readme.md"],
    );
    assert_eq!(
      resolve_partial_path(&tree.root, "docs/architecture/08-view-support.md"),
      PartialPathResolution::Exact("docs/architecture/08-view-support.md".to_string())
    );
    // A path that names a directory is not an exact file hit, and no file ends
    // with it: nothing changes.
    assert_eq!(
      resolve_partial_path(&tree.root, "docs"),
      PartialPathResolution::NotFound
    );
  }

  #[test]
  fn missing_leading_segments_resolve_by_suffix() {
    let tree = TempTree::with("suffix", &["docs/architecture/08-view-support.md"]);
    assert_eq!(
      resolved(resolve_partial_path(
        &tree.root,
        "architecture/08-view-support.md"
      )),
      (
        "docs/architecture/08-view-support.md".to_string(),
        PartialPathMatch::Suffix,
        1
      )
    );
  }

  #[test]
  fn a_bare_file_name_resolves_by_name() {
    let tree = TempTree::with(
      "basename",
      &[
        "docs/architecture/08-view-support.md",
        "docs/architecture/07-lsp.md",
      ],
    );
    assert_eq!(
      resolved(resolve_partial_path(&tree.root, "08-view-support.md")),
      (
        "docs/architecture/08-view-support.md".to_string(),
        PartialPathMatch::Suffix,
        2
      ),
      "a single-segment value is a file name, and the shallowest real file wins"
    );
  }

  #[test]
  fn a_wrong_middle_segment_falls_back_to_the_file_name() {
    let tree = TempTree::with("middle", &["docs/architecture/08-view-support.md"]);
    assert_eq!(
      resolved(resolve_partial_path(
        &tree.root,
        "wrong/middle/08-view-support.md"
      )),
      (
        "docs/architecture/08-view-support.md".to_string(),
        PartialPathMatch::Basename,
        2
      ),
      "the name tier rescues a citation whose middle segments are wrong"
    );
  }

  #[test]
  fn a_suffix_match_is_segment_wise_not_a_string_suffix() {
    // Only the decoy exists: the file name still matches, but never as a
    // segment suffix, so it can only ever resolve through the name tier.
    let decoy = TempTree::with("segments", &["docs/xarchitecture/08-view-support.md"]);
    assert_eq!(
      resolved(resolve_partial_path(
        &decoy.root,
        "architecture/08-view-support.md"
      )),
      (
        "docs/xarchitecture/08-view-support.md".to_string(),
        PartialPathMatch::Basename,
        2
      ),
      "`xarchitecture` is not the `architecture` segment"
    );

    // With the real segment suffix present, it outranks the decoy.
    let both = TempTree::with(
      "segments-both",
      &[
        "docs/xarchitecture/08-view-support.md",
        "docs/architecture/08-view-support.md",
      ],
    );
    assert_eq!(
      resolved(resolve_partial_path(
        &both.root,
        "architecture/08-view-support.md"
      )),
      (
        "docs/architecture/08-view-support.md".to_string(),
        PartialPathMatch::Suffix,
        1
      )
    );
  }

  #[test]
  fn a_suffix_tier_beats_a_name_tier() {
    let tree = TempTree::with(
      "tiers",
      &[
        "docs/architecture/08-view-support.md",
        "elsewhere/deep/08-view-support.md",
      ],
    );
    assert_eq!(
      resolved(resolve_partial_path(
        &tree.root,
        "architecture/08-view-support.md"
      ))
      .0,
      "docs/architecture/08-view-support.md",
      "a full segment suffix outranks a file-name-only match"
    );
  }

  #[test]
  fn the_shallowest_candidate_wins_and_equal_candidates_fail_closed() {
    // Neither input is exact, so both are real fuzzy answers.
    let shallow = TempTree::with(
      "shallow",
      &[
        "docs/architecture/08-view-support.md",
        "deep/docs/architecture/08-view-support.md",
      ],
    );
    assert_eq!(
      resolved(resolve_partial_path(
        &shallow.root,
        "architecture/08-view-support.md"
      )),
      (
        "docs/architecture/08-view-support.md".to_string(),
        PartialPathMatch::Suffix,
        1
      ),
      "fewer extra segments is strictly better"
    );

    // Two segment suffixes at the same depth are equally good: fail closed.
    let suffix_tie = TempTree::with(
      "suffix-tie",
      &[
        "docs/architecture/08-view-support.md",
        "notes/architecture/08-view-support.md",
      ],
    );
    let (candidates, sample, truncated) = ambiguous(resolve_partial_path(
      &suffix_tie.root,
      "architecture/08-view-support.md",
    ));
    assert_eq!(candidates, 2);
    assert_eq!(
      sample,
      vec![
        "docs/architecture/08-view-support.md".to_string(),
        "notes/architecture/08-view-support.md".to_string()
      ]
    );
    assert!(!truncated);

    // A name-tier tie is equally ambiguous and must fail closed too.
    let name_tie = TempTree::with(
      "name-tie",
      &[
        "docs/architecture/08-view-support.md",
        "docs/reference/08-view-support.md",
        "notes/third/08-view-support.md",
      ],
    );
    let (candidates, sample, truncated) =
      ambiguous(resolve_partial_path(&name_tie.root, "08-view-support.md"));
    assert_eq!(candidates, 3, "every equally shallow file is reported");
    assert_eq!(
      sample,
      vec![
        "docs/architecture/08-view-support.md".to_string(),
        "docs/reference/08-view-support.md".to_string(),
        "notes/third/08-view-support.md".to_string()
      ],
      "the sample is ordered, never a silent pick"
    );
    assert!(!truncated);
  }

  #[test]
  fn ranking_is_deterministic_across_calls() {
    let tree = TempTree::with(
      "deterministic",
      &[
        "docs/architecture/08-view-support.md",
        "docs/reference/08-view-support.md",
        "z/08-view-support.md",
      ],
    );
    let first = resolve_partial_path(&tree.root, "08-view-support.md");
    for _ in 0..8 {
      assert_eq!(resolve_partial_path(&tree.root, "08-view-support.md"), first);
    }
    assert_eq!(resolved(first).0, "z/08-view-support.md");
  }

  #[test]
  fn containment_refuses_device_paths_and_traversal() {
    let tree = TempTree::with("containment", &["docs/readme.md"]);
    for partial in [
      "/etc/passwd",
      "C:/Windows/win.ini",
      "c:/Windows/win.ini",
      "..",
      "../outside.md",
      "docs/../../outside.md",
      "./docs/readme.md",
      "docs//readme.md",
      "docs\\readme.md",
      "docs/readme.md/",
      "\\\\server\\share\\readme.md",
      "docs\u{0}/readme.md",
    ] {
      assert_eq!(
        resolve_partial_path(&tree.root, partial),
        PartialPathResolution::NotFound,
        "a device or traversal path must never be searched: {:?}",
        partial
      );
    }
    assert_eq!(
      resolve_partial_path(&tree.root.join("missing"), "docs/readme.md"),
      PartialPathResolution::Unavailable
    );
  }

  #[test]
  fn a_candidate_outside_the_mount_is_never_offered() {
    let outer = TempTree::with("outer", &["sibling.md"]);
    let tree = TempTree::with("inner", &["docs/readme.md"]);
    // `is_contained_file` compares canonical paths, so the root it is given must
    // be canonical, exactly as `resolve_partial_path_with` canonicalizes it.
    let root = std::fs::canonicalize(&tree.root).expect("canonical mount root");
    assert!(is_contained_file(&root, &tree.path("docs/readme.md")));
    assert!(!is_contained_file(&root, &outer.path("sibling.md")));
    assert!(!is_contained_file(
      &root,
      &root.parent().expect("parent").join("anything.md")
    ));
  }

  #[cfg(windows)]
  fn create_file_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_file(target, link)
  }

  #[cfg(not(windows))]
  fn create_file_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
  }

  #[test]
  fn a_symlink_that_leaves_the_mount_is_refused() {
    let outer = TempTree::with("symlink-outer", &["escaped.md"]);
    let tree = TempTree::with("symlink-inner", &["docs/readme.md"]);
    let link = tree.path("escaped.md");
    if create_file_symlink(&outer.path("escaped.md"), &link).is_err() {
      // Creating a symlink needs a privilege this environment may not grant;
      // the predicate test above still covers the refusal itself.
      eprintln!("skipped: this environment cannot create a file symlink");
      return;
    }
    assert!(
      !is_contained_file(
        &std::fs::canonicalize(&tree.root).expect("canonical mount root"),
        &link
      ),
      "a symlink out of the Mount is not contained"
    );
    assert_eq!(
      resolve_partial_path(&tree.root, "escaped.md"),
      PartialPathResolution::NotFound,
      "the escaped target must not be offered"
    );

    // A symlink that stays inside the Mount is a legitimate candidate; it is
    // exact, because the link itself is a real entry that resolves to the file.
    let inner_link = tree.path("linked.md");
    if create_file_symlink(&tree.path("docs/readme.md"), &inner_link).is_ok() {
      assert_eq!(
        resolve_partial_path(&tree.root, "linked.md"),
        PartialPathResolution::Exact("linked.md".to_string())
      );
    }
  }

  #[test]
  fn heavy_directories_are_skipped_until_nothing_else_matches() {
    let tree = TempTree::with(
      "heavy",
      &[
        "target/debug/08-view-support.md",
        "docs/architecture/08-view-support.md",
      ],
    );
    assert_eq!(
      resolved(resolve_partial_path(&tree.root, "08-view-support.md")).0,
      "docs/architecture/08-view-support.md",
      "build output never outranks a document"
    );

    let only_build = TempTree::with("heavy-only", &["node_modules/pkg/notes.md"]);
    assert_eq!(
      resolved(resolve_partial_path(&only_build.root, "notes.md")).0,
      "node_modules/pkg/notes.md",
      "a heavy directory is searched when nothing else matches"
    );
  }

  #[test]
  fn a_deep_tree_is_bounded_by_depth() {
    let tree = TempTree::new("deep");
    let mut deep = String::new();
    for index in 0..8 {
      deep.push_str(&format!("d{index}/"));
    }
    tree.file(&format!("{deep}buried.md"));
    tree.file("shallow.md");

    let limits = PartialPathLimits {
      max_depth: 4,
      ..PartialPathLimits::default()
    };
    assert_eq!(
      resolve_partial_path_with(&tree.root, "buried.md", limits),
      PartialPathResolution::NotFound,
      "a file below the depth bound is never offered"
    );
    assert_eq!(
      resolved(resolve_partial_path_with(
        &tree.root,
        "buried.md",
        PartialPathLimits::default()
      ))
      .0,
      format!("{deep}buried.md"),
      "within the production bound it is found"
    );

    // The production bound itself: one segment beyond it is never offered.
    let over = TempTree::new("over-deep");
    let mut too_deep = String::new();
    for index in 0..(PartialPathLimits::default().max_depth + 1) {
      too_deep.push_str(&format!("d{index}/"));
    }
    over.file(&format!("{too_deep}lost.md"));
    assert_eq!(
      resolve_partial_path(&over.root, "lost.md"),
      PartialPathResolution::NotFound
    );
  }

  #[test]
  fn a_wide_tree_is_bounded_by_entries() {
    let tree = TempTree::new("wide");
    for index in 0..64 {
      tree.file(&format!("dir-{index:03}/unique-{index:03}.md"));
      tree.file(&format!("dir-{index:03}/shared.md"));
    }
    // A unique name inside a wide tree still resolves, within the budget.
    assert_eq!(
      resolved(resolve_partial_path_with(
        &tree.root,
        "unique-007.md",
        PartialPathLimits::default()
      ))
      .0,
      "dir-007/unique-007.md"
    );

    // A budget that runs out mid-walk fails closed instead of resolving.
    let limits = PartialPathLimits {
      max_entries: 70,
      ..PartialPathLimits::default()
    };
    let (candidates, _, truncated) =
      ambiguous(resolve_partial_path_with(&tree.root, "shared.md", limits));
    assert!(candidates >= 2, "at least two candidates were seen");
    assert!(truncated, "the bound must be reported as truncation");
  }

  #[test]
  fn a_candidate_bound_fails_closed() {
    let tree = TempTree::with("candidate-bound", &["a/x.md", "b/x.md", "c/x.md", "d/x.md"]);
    let limits = PartialPathLimits {
      max_candidates: 2,
      ..PartialPathLimits::default()
    };
    let (candidates, sample, truncated) =
      ambiguous(resolve_partial_path_with(&tree.root, "x.md", limits));
    assert_eq!(candidates, 2);
    assert_eq!(sample, vec!["a/x.md".to_string(), "b/x.md".to_string()]);
    assert!(truncated, "the candidate bound is a lower bound, not a count");
  }

  #[test]
  fn an_empty_partial_path_is_never_searchable() {
    let tree = TempTree::with("empty", &["readme.md"]);
    assert_eq!(
      resolve_partial_path(&tree.root, ""),
      PartialPathResolution::NotFound,
      "the Mount root is not a file"
    );
  }
}
