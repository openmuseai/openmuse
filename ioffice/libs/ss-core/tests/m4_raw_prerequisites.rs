use kfc::cn::wps::kfc::numfmt::util::DateUtil::DateUtil;
use ss_core::cn::wps::et::ss::constant::NumfmtCategory::NumfmtCategory;

#[test]
fn original_java_category_date_and_jdk_double_vectors() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let evidence =
        root.join("rewrite-review/milestones/m4-edit-save/evidence/raw-prerequisites-r0b26");
    let source = std::fs::read_to_string(evidence.join("expected-android.tsv")).unwrap();
    let mut rows = Vec::new();
    for (i, line) in source.lines().enumerate() {
        let a: Vec<_> = line.split('\t').collect();
        let units: Vec<_> = (0..a[2].len())
            .step_by(4)
            .map(|i| u16::from_str_radix(&a[2][i..i + 4], 16).unwrap())
            .collect();
        let text = String::from_utf16(&units).unwrap();
        let actual = if a[0] == "M" {
            let mut map=ss_core::java_runtime::ConcurrentLongMap::ConcurrentLongMap::<usize>::default();
            if !text.is_empty(){for (i,key)in text.split(',').enumerate(){if key=="c"{map.clear();}else{map.insert(u64::from_str_radix(key,16).unwrap()as i64,i);}}}
            map.into_iter().map(|(k,v)|format!("{:x}={v}",k as u64)).collect::<Vec<_>>().join(",")
        } else if a[0] == "T" {
            kfc::java_runtime::FloatingDecimal::FloatingDecimal::toJavaFormatString(f64::from_bits(u64::from_str_radix(&text,16).unwrap()))
        } else if a[0] == "F" {
            kfc::java_runtime::FloatToDecimal::FloatToDecimal::toString(f32::from_bits(u32::from_str_radix(&text,16).unwrap()))
        } else if a[0] == "D" {
            ss_core::java_runtime::parseDouble(&text).map_or(
                "EXCEPTION:NumberFormatException".into(),
                |v| {
                    format!(
                        "{:x}",
                        if v.is_nan() {
                            0x7ff8000000000000
                        } else {
                            v.to_bits()
                        }
                    )
                },
            )
        } else {
            let index: i16 = a[1].parse().unwrap();
            format!(
                "{:?}\t{}\t{}",
                NumfmtCategory::getNumfmtCat(index, &text),
                DateUtil::isDateTimeFormat(index as i32, &text),
                DateUtil::isADateFormat(index as i32, &text)
            )
        };
        rows.push(format!("{}\t{}\t{}\t{}", a[0], a[1], a[2], actual));
        assert_eq!(
            actual,
            a[3..].join("\t"),
            "vector {i}: index={} text={text:?}",
            a[1]
        );
    }
    std::fs::write(evidence.join("rust.tsv"), rows.join("\n") + "\n").unwrap();
}
