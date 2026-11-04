use ss_core::cn::wps::et::ss::formula::functions::util::MathX::MathX;
use ss_core::org::apache::poi::ss::util::NumberToTextConverter::NumberToTextConverter;
#[test]
fn original_java_numeric_text_and_round15_vectors() {
    let gold = include_str!(
        "../rewrite-review/milestones/m1-xlsx-grid-read/evidence/java-numeric-values-r0b21.tsv"
    );
    for line in gold.lines() {
        let f: Vec<_> = line.split('\t').collect();
        let bits = u64::from_str_radix(f[0], 16).unwrap();
        let n = f64::from_bits(bits);
        assert_eq!(
            NumberToTextConverter::toText(n, '.'),
            f[2],
            "text bits={bits:x}"
        );
        assert_eq!(
            NumberToTextConverter::toText(n, ','),
            f[3],
            "comma bits={bits:x}"
        );
        if f[1] != "-" {
            assert_eq!(
                MathX::round15(n).doubleValue().to_bits(),
                u64::from_str_radix(f[1], 16).unwrap(),
                "round15 bits={bits:x}"
            );
        }
    }
}
