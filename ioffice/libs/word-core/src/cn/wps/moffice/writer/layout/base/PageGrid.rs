//! Section → page size table + cp → page index (boundary belongs to previous page).

#[derive(Clone, Debug)]
pub struct SectionSpec {
    pub start_cp: i32,
    pub end_cp: i32,
    pub content_width: i32,
    pub content_height: i32,
}

#[derive(Clone, Debug)]
pub struct PageSlot {
    pub start_cp: i32,
    pub end_cp: i32,
    pub content_width: i32,
    pub content_height: i32,
}

#[derive(Clone, Debug)]
pub struct PageGrid {
    pub pages: Vec<PageSlot>,
}

pub fn page_grid(sections: &[SectionSpec]) -> PageGrid {
    if sections.is_empty() {
        return PageGrid {
            pages: vec![PageSlot {
                start_cp: 0,
                end_cp: i32::MAX,
                content_width: 8306,
                content_height: 13958,
            }],
        };
    }
    let pages = sections
        .iter()
        .map(|s| PageSlot {
            start_cp: s.start_cp,
            end_cp: s.end_cp,
            content_width: s.content_width,
            content_height: s.content_height,
        })
        .collect();
    PageGrid { pages }
}

/// Shared boundary `cp == next.start` belongs to the previous page.
pub fn page_index_from_cp(grid: &PageGrid, cp: i32) -> i32 {
    if grid.pages.is_empty() {
        return 0;
    }
    let mut idx = 0usize;
    for (i, p) in grid.pages.iter().enumerate() {
        if p.start_cp <= cp {
            idx = i;
        }
    }
    if idx > 0 && cp == grid.pages[idx].start_cp {
        let prev = &grid.pages[idx - 1];
        if prev.end_cp == grid.pages[idx].start_cp {
            idx -= 1;
        }
    }
    idx as i32
}
