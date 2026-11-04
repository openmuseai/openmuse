//! Stage1 JSON compare. No Java type. Strict equality; never rewrite dumps to match.

use crate::cn::wps::moffice::writer::api::stage1::PropDump::Stage1Props;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FirstDiff {
    pub path: String,
}

pub struct Stage1Doc {
    pub text: String,
    pub props: Stage1Props,
}

pub struct Stage1Compare {}

impl Stage1Compare {
    pub fn diff(java: &Stage1Doc, rust: &Stage1Doc) -> Result<(), FirstDiff> {
        if java.text != rust.text {
            return Err(FirstDiff {
                path: "text".to_string(),
            });
        }
        if java.props.papx.len() != rust.props.papx.len() {
            return Err(FirstDiff {
                path: "papx.len".to_string(),
            });
        }
        for (i, (a, b)) in java.props.papx.iter().zip(rust.props.papx.iter()).enumerate() {
            if a != b {
                return Err(FirstDiff {
                    path: format!("papx[{}]", i),
                });
            }
        }
        if java.props.chpx.len() != rust.props.chpx.len() {
            return Err(FirstDiff {
                path: "chpx.len".to_string(),
            });
        }
        for (i, (a, b)) in java.props.chpx.iter().zip(rust.props.chpx.iter()).enumerate() {
            if a != b {
                return Err(FirstDiff {
                    path: format!("chpx[{}]", i),
                });
            }
        }
        Ok(())
    }

    pub fn diff_json(java: &str, rust: &str) -> Result<(), FirstDiff> {
        let j: Stage1DocJson = serde_json::from_str(java).map_err(|_| FirstDiff {
            path: "json.parse.java".to_string(),
        })?;
        let r: Stage1DocJson = serde_json::from_str(rust).map_err(|_| FirstDiff {
            path: "json.parse.rust".to_string(),
        })?;
        Self::diff(
            &Stage1Doc {
                text: j.text,
                props: j.props,
            },
            &Stage1Doc {
                text: r.text,
                props: r.props,
            },
        )
    }
}

#[derive(serde::Deserialize)]
struct Stage1DocJson {
    text: String,
    #[serde(default)]
    props: Stage1Props,
}
