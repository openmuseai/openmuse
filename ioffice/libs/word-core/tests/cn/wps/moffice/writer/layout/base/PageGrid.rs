use word_core::cn::wps::moffice::writer::layout::base::PageGrid::{
    page_grid, page_index_from_cp, SectionSpec,
};

#[test]
fn g1_single_section() {
    let g = page_grid(&[SectionSpec {
        start_cp: 0,
        end_cp: 100,
        content_width: 8000,
        content_height: 10000,
    }]);
    assert_eq!(g.pages.len(), 1);
    assert_eq!(g.pages[0].content_width, 8000);
}

#[test]
fn g2_multi_section_sizes() {
    let g = page_grid(&[
        SectionSpec {
            start_cp: 0,
            end_cp: 100,
            content_width: 8000,
            content_height: 10000,
        },
        SectionSpec {
            start_cp: 100,
            end_cp: 200,
            content_width: 4000,
            content_height: 5000,
        },
    ]);
    assert_eq!(g.pages[0].content_width, 8000);
    assert_eq!(g.pages[1].content_width, 4000);
}

#[test]
fn g3_cp_index() {
    let g = page_grid(&[
        SectionSpec {
            start_cp: 0,
            end_cp: 100,
            content_width: 1,
            content_height: 1,
        },
        SectionSpec {
            start_cp: 100,
            end_cp: 200,
            content_width: 2,
            content_height: 2,
        },
    ]);
    assert_eq!(page_index_from_cp(&g, 150), 1);
}

#[test]
fn g4_boundary_previous() {
    let g = page_grid(&[
        SectionSpec {
            start_cp: 0,
            end_cp: 100,
            content_width: 1,
            content_height: 1,
        },
        SectionSpec {
            start_cp: 100,
            end_cp: 200,
            content_width: 2,
            content_height: 2,
        },
    ]);
    assert_eq!(page_index_from_cp(&g, 100), 0);
}

#[test]
fn g5_empty_default() {
    let g = page_grid(&[]);
    assert_eq!(g.pages.len(), 1);
    assert_eq!(page_index_from_cp(&g, 0), 0);
}
