//! Calls the original-class Rust formatter pipeline for the Java oracle's TSV inputs.
use kfc::cn::wps::kfc::numfmt::{
    format::NumFmtResult::NumFmtResult, usermodel::NumberFormatter::NumberFormatter,
};
fn decode(text: &str) -> String {
    let units = (0..text.len())
        .step_by(4)
        .map(|i| u16::from_str_radix(&text[i..i + 4], 16).unwrap())
        .collect::<Vec<_>>();
    String::from_utf16(&units).unwrap()
}
fn encode(text: &str) -> String {
    text.encode_utf16().map(|u| format!("{u:04x}")).collect()
}
fn main() {
    kfc::cn::wps::kfc::numfmt::i18n::KmoLang::KmoLang::setLocale("zh_CN".into());
    let path = std::env::args().nth(1).expect("Java TSV path");
    for line in std::fs::read_to_string(path).unwrap().lines() {
        let a: Vec<_> = line.split('\t').collect();
        let mut formatter = NumberFormatter::createInstance(a[0].parse().unwrap());
        let mut r = NumFmtResult::new();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if a[1] == "v" {
                r.mNumFmtStr = format!(
                    "{:016x}",
                    formatter
                        .formatValue(
                            f64::from_bits(u64::from_str_radix(a[2], 16).unwrap()),
                            a[5] == "true",
                            &decode(a[3])
                        )
                        .to_bits()
                );
            } else if a[1] == "n" {
                formatter.format_num(
                    f64::from_bits(u64::from_str_radix(a[2], 16).unwrap()),
                    &decode(a[3]),
                    a[4].parse().unwrap(),
                    a[5] == "true",
                    &mut r,
                );
            } else {
                formatter.format_str(&decode(a[2]), &decode(a[3]), a[4].parse().unwrap(), &mut r);
            }
        }));
        let input = a[..6].join("\t");
        match result {
            Ok(()) => println!(
                "{input}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                encode(&r.mNumFmtStr),
                encode(&r.mNumRealValue),
                r.mNumFmtColorIdx
                    .map_or_else(|| "null".into(), |c| c.to_string()),
                r.richFormatInvalid,
                r.mNumericAsText,
                r.mFillPos,
                r.mFillChar as u32
            ),
            Err(_) => println!("{input}\tEXCEPTION\tRustPanic"),
        }
    }
}
