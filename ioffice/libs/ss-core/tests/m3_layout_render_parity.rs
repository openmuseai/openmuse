use ss_core::cn::wps::moss::app::UnitsConverter::UnitsConverter;
#[test]
fn original_java_units_float_bits() {
    let root = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/rewrite-review/milestones/m3-layout-render/fixtures/units/expected.tsv"
    );
    for (line, s) in std::fs::read_to_string(root).unwrap().lines().enumerate() {
        let a = s.split('\t').collect::<Vec<_>>();
        let parse = |s: &str| f32::from_bits(u32::from_str_radix(s, 16).unwrap());
        if a[0] == "C" {
            let (v, cw) = (parse(a[1]), parse(a[2]));
            assert_eq!(
                UnitsConverter::columnUnitsToPixels(v, cw),
                a[3].parse::<i32>().unwrap(),
                "{line}"
            );
            assert_eq!(
                UnitsConverter::columnUnitsToCharWidth(v, cw).to_bits(),
                parse(a[4]).to_bits(),
                "{line}"
            );
        } else {
            let mut u = UnitsConverter::UnitsConverter__2(parse(a[0]), parse(a[0]));
            u.setZoom(a[1].parse().unwrap());
            let v = parse(a[2]);
            for (actual, expected) in [
                u.TwipsToPixels(v),
                u.TwipsToPixelsY(v),
                u.twipsToPixelsYUnScale(v),
                u.PointsToPixels(v),
                u.getZoom(),
            ]
            .iter()
            .zip(&a[3..8])
            {
                assert_eq!(actual.to_bits(), parse(expected).to_bits(), "{line}");
            }
            assert_eq!(u.PixelsToTwips(v), a[8].parse::<i32>().unwrap(), "{line}");
        }
    }
}

#[test]
fn pane_and_rich_run_writes_use_the_live_atom_snapshot() {
    use kfc::cn::wps::memor4j::KmoAtomStore::{KmoAtomStore, KmoAtomStoreTrait};
    use ss_core::cn::wps::moss::{
        core::sheet::KmoPaneInfo::KmoPaneInfo, filefmt::biff8::record::common::FormatRun::FormatRun,
    };
    KmoAtomStore::enable();
    KmoAtomStore::clearUp();
    let pane = KmoPaneInfo::new();
    let run = FormatRun::new(3, 2);
    let store = |f: &mut dyn FnMut(&mut dyn KmoAtomStoreTrait)| {
        let mut s = KmoAtomStore::sharedStore();
        f(s.as_mut().unwrap().as_mut());
    };
    store(&mut |s| s.start());
    pane.setX(7);
    pane.setY(4);
    run.setFontIndex(9);
    run.setCharPos(5);
    store(&mut |s| assert!(s.commit()));
    assert_eq!(
        (
            pane.x(),
            pane.y(),
            run.getCharacterPos(),
            run.getFontIndex()
        ),
        (7, 4, 5, 9)
    );
    store(&mut |s| {
        s.undo();
    });
    assert_eq!(
        (
            pane.x(),
            pane.y(),
            run.getCharacterPos(),
            run.getFontIndex()
        ),
        (0, 0, 3, 2)
    );
    store(&mut |s| {
        s.redo();
    });
    assert_eq!(
        (
            pane.x(),
            pane.y(),
            run.getCharacterPos(),
            run.getFontIndex()
        ),
        (7, 4, 5, 9)
    );
    KmoAtomStore::clearUp();
}

#[test]
fn original_java_grid_cache_block_boundaries_and_integer_wrap() {
    use ss_core::cn::wps::moffice::spreadsheet::control::grid::core::{
        GridCache::GridCache, GridRowCache::GridRowCache,
    };
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/rewrite-review/milestones/m3-layout-render/fixtures/grid-cache/expected.tsv"
    ))
    .unwrap();
    let mut cache = GridRowCache::new();
    for (line, text) in text.lines().enumerate() {
        let p = text.split('\t').collect::<Vec<_>>();
        let num = |s: &str| s.parse::<i32>().unwrap();
        match p[0] {
            "R" => {
                let len = num(p[1]);
                let def = num(p[2]);
                let rows = if len < 0 {
                    None
                } else {
                    let mut sum = 0i32;
                    Some(
                        (0..len)
                            .map(|i| {
                                let v = sum;
                                sum = sum.wrapping_add(if i % 17 == 0 {
                                    0
                                } else if i % 31 == 0 {
                                    49
                                } else {
                                    def
                                });
                                v
                            })
                            .collect::<Vec<_>>(),
                    )
                };
                cache.reset(8192, rows.as_deref(), def);
                let expected = p[3].split(',').map(num).collect::<Vec<_>>();
                assert_eq!(cache.mCache.as_ref().unwrap(), &expected, "{line}");
                let mut copy = GridRowCache::new();
                copy.copyFrom(&cache);
                assert_eq!(copy.mCache, cache.mCache, "{line}");
            }
            "Q" => {
                let pos = num(p[1]);
                let info = cache.search(pos);
                assert_eq!(
                    [
                        GridCache::search__static(pos, cache.mCache.as_ref().unwrap()),
                        info.mStartIndex,
                        info.mEndIndex,
                        info.mStartPos,
                        info.mEndPos
                    ],
                    [num(p[2]), num(p[3]), num(p[4]), num(p[5]), num(p[6])],
                    "{line}"
                );
            }
            "I" => {
                let row = num(p[1]);
                assert_eq!(
                    [cache.startIndex(row), cache.startPos(row)],
                    [num(p[2]), num(p[3])],
                    "{line}"
                );
            }
            _ => panic!("invalid Java oracle"),
        }
    }
}
