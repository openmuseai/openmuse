use openmuse_storage_qualification::{QualificationStatus, RustfsQualificationEvidence, evaluate};
use std::env;
use std::fs;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: evaluate_report <evidence.json> [--require-eligible]");
        std::process::exit(64);
    });
    let require_eligible = env::args().any(|arg| arg == "--require-eligible");
    let bytes = fs::read(&path).unwrap_or_else(|error| {
        eprintln!("cannot read {path}: {error}");
        std::process::exit(66);
    });
    let evidence: RustfsQualificationEvidence =
        serde_json::from_slice(&bytes).unwrap_or_else(|error| {
            eprintln!("invalid qualification evidence: {error}");
            std::process::exit(65);
        });
    let decision = evaluate(&evidence);
    println!("{}", serde_json::to_string_pretty(&decision).unwrap());
    if require_eligible && decision.status != QualificationStatus::Eligible {
        std::process::exit(2);
    }
}
