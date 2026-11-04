#![allow(non_snake_case)]

// Java: import cn.wps.moffice.writer.cache.TypoSnapshot
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
// Java: import cn.wps.moffice.writer.cache.TypoTableRow
use crate::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow;

// java final class TableCellBrcTool
pub struct TableCellBrcTool;

impl TableCellBrcTool {
    // java public static void removeOverlappedBrcs(TypoSnapshot snapshot, int lines)
    pub fn removeOverlappedBrcs(snapshot: &mut TypoSnapshot, lines: i32) {
      let raw = snapshot as *mut TypoSnapshot;
      let size = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::size__int_typosnapshot(lines, unsafe { &*raw });
      let mut prev = 0i32;
      let mut parent = 0i32;
      for i in 0..size {
        let line = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::quickGet__int_int_typosnapshot(i, lines, unsafe { &mut *raw });
        if crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj::getType__int_typosnapshot(line, unsafe { &*raw }) != 5 {
          continue;
        }
        let mut row = TypoTableRow::new();
        row.assignment(line, raw);
        if row.isRevisionDelete() || row.isWrap() {
          continue;
        }
        if parent == 0 {
          parent = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getPage__int_typosnapshot(line, unsafe { &*raw });
        }
        Self::doRemoveOverlappedBrcs(raw, parent, prev, line);
        prev = line;
      }
    }

    fn doRemoveOverlappedBrcs(raw: *mut TypoSnapshot, parent: i32, prev: i32, cur: i32) {
      if prev == 0 && cur == 0 {
        return;
      }
      let mut cur_row = TypoTableRow::new();
      cur_row.assignment(cur, raw);
      let mut prev_row = TypoTableRow::new();
      if prev != 0 {
        prev_row.assignment(prev, raw);
      }
      let adjacent = prev != 0 && prev_row.getBottom() == cur_row.getTop();
      let prev_rtl = if adjacent { prev_row.isRTL() } else { false };
      let cur_rtl = cur_row.isRTL();
      let mut most_left = f32::MAX;
      let mut prev_cell = 0i32;
      let mut lower_pos = 0i32;
      let count = cur_row.getCellsCount();
      while lower_pos < count {
        let cell_ptr = cur_row.getCell(lower_pos);
        lower_pos += 1;
        let mut cell = crate::cn::wps::moffice::writer::cache::TypoTableCell::TypoTableCell::new();
        cell.assignment(cell_ptr, raw);
        let mut previous = crate::cn::wps::moffice::writer::cache::TypoTableCell::TypoTableCell::new();
        previous.assignment(prev_cell, raw);
        let is_first = !previous.isValid();
        let is_last = lower_pos >= count;
        Self::removeOverlappedHoriBrc(raw, &mut previous, &mut cell, is_first, is_last);
        let hidden = cell.isVertMergeCell() && !cell.isFirstVertMergeCell();
        if hidden {
          cell.setBrcDiagonalUp(0);
          cell.setBrcDiagonalDown(0);
        }
        if adjacent {
          Self::removeOverlappedVertBrc(raw, parent, &prev_row, &mut cell, cur_rtl, prev_rtl, hidden, &mut most_left);
        }
        if cell.getSHD() != 0 {
          let top = Self::getBrcStrokeWidth(Self::brc(raw, cell.getBrcTop()));
          let bottom = Self::getBrcStrokeWidth(Self::brc(raw, cell.getBrcBottom()));
          cell.setShdMarginTop(top);
          cell.setShdMarginBottom(bottom);
        }
        prev_cell = cell_ptr;
      }
    }

    fn removeOverlappedVertBrc(
      raw: *mut TypoSnapshot,
      parent: i32,
      prev_row: &TypoTableRow,
      lower: &mut crate::cn::wps::moffice::writer::cache::TypoTableCell::TypoTableCell,
      cur_rtl: bool,
      prev_rtl: bool,
      hidden: bool,
      most_left: &mut f32,
    ) {
      let mut rect = crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl::default();
      Self::calc(unsafe { &*raw }, parent, cur_rtl, lower, &mut rect);
      let lower_left = rect.super_.left;
      let lower_right = rect.super_.right;
      let lower_top = Self::brc(raw, lower.getBrcTop());
      let pre_count = prev_row.getCellsCount();
      let mut upper_pos = 0i32;
      while upper_pos >= 0 && upper_pos < pre_count {
        let mut upper = crate::cn::wps::moffice::writer::cache::TypoTableCell::TypoTableCell::new();
        upper.assignment(prev_row.getCell(upper_pos), raw);
        Self::calc(unsafe { &*raw }, parent, prev_rtl, &upper, &mut rect);
        let upper_left = rect.super_.left;
        let upper_right = rect.super_.right;
        let adjacent_merge = hidden && upper.isVertMergeCell();
        *most_left = most_left.min(upper_left as f32);
        let keep_lower_top = (lower_left as f32) < *most_left;
        if adjacent_merge && upper_left == lower_left && upper_right == lower_right {
          upper.setBrcBottom(0);
        } else if Self::containsPos(upper_left, lower_left, lower_right)
          && Self::containsPos(upper_right, lower_left, lower_right)
          && Self::isOverlappable(lower_top.as_ref(), Self::brc(raw, upper.getBrcBottom()).as_ref())
          && !keep_lower_top
          && lower_top.is_some()
        {
          upper.setBrcBottom(lower.getBrcTop());
        }
        if upper_right >= lower_right {
          let bottom = Self::brc(raw, upper.getBrcBottom());
          let clear = (bottom.as_ref().is_some_and(|brc| brc.isValidTableBrc())
            || !Self::containsPos(lower_left, upper_left, upper_right)
            || !Self::containsPos(lower_right, upper_left, upper_right)
            || adjacent_merge)
            && !keep_lower_top;
          if clear {
            lower.setBrcTop(0);
          }
          break;
        }
        if upper.getSHD() != 0 {
          upper.setShdMarginBottom(Self::getBrcStrokeWidth(Self::brc(raw, upper.getBrcBottom())));
        }
        upper_pos += 1;
      }
    }

    fn calc(
      snapshot: &TypoSnapshot,
      page: i32,
      rtl: bool,
      cell: &crate::cn::wps::moffice::writer::cache::TypoTableCell::TypoTableCell,
      out: &mut crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl,
    ) {
      cell.rectForPageCore(out);
      if rtl {
        let margin_right = crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::getCoreWidth__int_typosnapshot(page, snapshot) - out.super_.right;
        let cell_width = cell.width();
        out.super_.left = margin_right;
        out.super_.right = margin_right + cell_width;
      }
    }

    fn removeOverlappedHoriBrc(
      raw: *mut TypoSnapshot,
      prev: &mut crate::cn::wps::moffice::writer::cache::TypoTableCell::TypoTableCell,
      cur: &mut crate::cn::wps::moffice::writer::cache::TypoTableCell::TypoTableCell,
      is_first: bool,
      is_last: bool,
    ) {
      if is_first && cur.getSHD() != 0 {
        let half = Self::getBrcStrokeWidth(Self::brc(raw, cur.getBrcLeft())) / 2;
        if half != 0 {
          cur.setShdMarginLeft(half);
        }
      }
      if is_last && cur.getSHD() != 0 {
        let half = Self::getBrcStrokeWidth(Self::brc(raw, cur.getBrcRight())) / 2;
        if half != 0 {
          cur.setShdMarginRight(half);
        }
      }
      if prev.isValid() && prev.getRight() == cur.getLeft() {
        let winner = Self::getTopMostBrcOwned(Self::brc(raw, prev.getBrcRight()), Self::brc(raw, cur.getBrcLeft()));
        let index = match &winner {
          Some(brc) => unsafe { (*raw).index_brc(brc) },
          None => 0,
        };
        prev.setBrcRight(index);
        cur.setBrcLeft(0);
        let half = Self::getBrcStrokeWidth(winner) / 2;
        if half != 0 && prev.getSHD() != 0 {
          prev.setShdMarginRight(prev.getShdMarginRight().max(half));
        }
        if half != 0 && cur.getSHD() != 0 {
          cur.setShdMarginLeft(cur.getShdMarginLeft().max(half));
        }
      }
    }

    fn containsPos(pos: i32, left: i32, right: i32) -> bool {
      pos >= left && pos <= right
    }

    fn brc(raw: *mut TypoSnapshot, index: i32) -> Option<crate::cn::wps::moffice::writer::data::value::BRC::BRC> {
      unsafe { (*raw).get_brc(index) }
    }

    fn getBrcStrokeWidth(brc: Option<crate::cn::wps::moffice::writer::data::value::BRC::BRC>) -> i32 {
      let Some(brc) = brc else { return 0; };
      util::moffice::util::UnitUtil::UnitUtil::PT2TWIPS(
        crate::cn::wps::moffice::writer::layout::base::tools::BRCBorderTool::BRCBorderTool::getActualStrokeWidth(
          brc.getBorderType(),
          brc.getLineWidth(),
        ),
      )
    }

    fn isOverlappable(
      left: Option<&crate::cn::wps::moffice::writer::data::value::BRC::BRC>,
      right: Option<&crate::cn::wps::moffice::writer::data::value::BRC::BRC>,
    ) -> bool {
      if left.is_none() && right.is_none() {
        return true;
      }
      match Self::getTopMostBrc(left, right) {
        Some(winner) => left.is_some_and(|brc| std::ptr::eq(brc, winner)),
        None => false,
      }
    }

    fn getTopMostBrc<'a>(
      left: Option<&'a crate::cn::wps::moffice::writer::data::value::BRC::BRC>,
      right: Option<&'a crate::cn::wps::moffice::writer::data::value::BRC::BRC>,
    ) -> Option<&'a crate::cn::wps::moffice::writer::data::value::BRC::BRC> {
      let (Some(a), Some(b)) = (left, right) else {
        return if left.is_none() { right } else { left };
      };
      let type1 = a.getBorderType();
      let type2 = b.getBorderType();
      if type1 == 0 || type1 == 255 {
        return Some(b);
      }
      if type2 == 0 || type2 == 255 {
        return Some(a);
      }
      let width_delta = a.getLineWidth() * (type1 as f32) - b.getLineWidth() * (type2 as f32);
      if width_delta != 0.0 {
        return if width_delta > 0.0 { Some(a) } else { Some(b) };
      }
      let type_delta = type1 - type2;
      if type_delta != 0 {
        return if type_delta < 0 { Some(a) } else { Some(b) };
      }
      let (r1, g1, b1) = Self::rgb(a.getColor());
      let (r2, g2, b2) = Self::rgb(b.getColor());
      let bright = Self::getHexColorBrightness1(r1, g1, b1).wrapping_sub(Self::getHexColorBrightness1(r2, g2, b2));
      if bright != 0 {
        return if bright < 0 { Some(a) } else { Some(b) };
      }
      let bright = Self::getHexColorBrightness2(r1, g1, b1).wrapping_sub(Self::getHexColorBrightness2(r2, g2, b2));
      if bright != 0 {
        return if bright < 0 { Some(a) } else { Some(b) };
      }
      let bright = Self::getHexColorBrightness3(g1).wrapping_sub(Self::getHexColorBrightness3(g2));
      if bright != 0 {
        return if bright < 0 { Some(a) } else { Some(b) };
      }
      Some(a)
    }

    fn getTopMostBrcOwned(
      left: Option<crate::cn::wps::moffice::writer::data::value::BRC::BRC>,
      right: Option<crate::cn::wps::moffice::writer::data::value::BRC::BRC>,
    ) -> Option<crate::cn::wps::moffice::writer::data::value::BRC::BRC> {
      let pick_left = Self::isOverlappable(left.as_ref(), right.as_ref());
      if pick_left { left } else { right }
    }

    fn rgb(color: i32) -> (i32, i32, i32) {
      (
        crate::cn::wps::moffice::util::IntBitSet::IntBitSet::new(16, 23).get(color),
        crate::cn::wps::moffice::util::IntBitSet::IntBitSet::new(8, 15).get(color),
        crate::cn::wps::moffice::util::IntBitSet::IntBitSet::new(0, 7).get(color),
      )
    }

    fn getHexColorBrightness1(r: i32, g: i32, b: i32) -> i32 { r.wrapping_add(b).wrapping_add(g.wrapping_mul(2)) }
    fn getHexColorBrightness2(_r: i32, g: i32, b: i32) -> i32 { b.wrapping_add(g.wrapping_mul(2)) }
    fn getHexColorBrightness3(g: i32) -> i32 { g }
}
