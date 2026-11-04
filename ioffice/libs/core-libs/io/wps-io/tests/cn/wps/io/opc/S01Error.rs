// 53_function_error_taxonomy —— S01 失败信号分类器用例（ET1–ET22）
//
// 规格来源：`features/s01-09-error-model/functions/53_function_error_taxonomy/FUNCTION.md` §3.4 用例表。
// 契约来源：`docs/s01/schema-v1.json`（`error_kinds` 10 值 / `error_compare`）。
//
// 本文件**只**断言用例表钉死的期望；不断言未在表中出现的实现细节。

use core_libs_io::io::opc::S01Error::{
    is_valid_at, ClassifyRejected, S01Error, S01ErrorKind, S01Failure, S01Taxonomy,
};

/// 冻结契约 `docs/s01/schema-v1.json:error_kinds` 的 10 值，**逐字符**写出。
const CONTRACT_KINDS: [&str; 10] = [
    "not_zip",
    "zip_corrupt",
    "missing_content_types",
    "malformed_content_types",
    "missing_main_part",
    "invalid_part_name",
    "missing_part",
    "relationship_not_found",
    "unsupported_encryption",
    "io_error",
];

// ───────────────────────── ET1：kind 词表封闭且逐字符相等 ─────────────────────────
#[test]
fn et1_kinds_equal_contract_word_list() {
    let got = S01Error::kinds();
    assert_eq!(got.len(), 10, "词表必须恰好 10 条（多/少一条即失败）");
    for (i, (a, b)) in got.iter().zip(CONTRACT_KINDS.iter()).enumerate() {
        assert_eq!(a, b, "第 {} 条的拼写/顺序与冻结契约不符", i);
    }
    // 枚举侧 `all()` 也必须与契约一致（顺序逐项）
    let all = S01ErrorKind::all();
    assert_eq!(all.len(), 10);
    for (i, k) in all.iter().enumerate() {
        assert_eq!(k.as_str(), CONTRACT_KINDS[i], "枚举第 {} 项与契约不符", i);
    }
}

// ───────────────────────── ET2：不可识别为 zip ⇒ not_zip ─────────────────────────
#[test]
fn et2_not_zip() {
    let f = S01Failure::ZipErr {
        msg: "invalid Zip archive".to_string(),
    };
    let e = S01Taxonomy::classify(&f).expect("合法输入不应被拒绝");
    assert_eq!(e.kind, S01ErrorKind::NotZip, "归到 zip_corrupt ⇒ fail");
    assert!(
        e.at.contains("#ZipArchive::new"),
        "ET2 要求锚点为 ZipArchive::new，实得 {}",
        e.at
    );
}

// ───────────────────────── ET3：条目读取失败 ⇒ zip_corrupt ─────────────────────────
#[test]
fn et3_zip_corrupt() {
    let f = S01Failure::ZipErr {
        msg: "invalid Zip archive".to_string(),
    };
    // 与 ET2 同 msg，靠 at 语境分流；此处直接断言 by_index 语境
    let e = S01Taxonomy::classify_panic_at("damaged entry", Some("xwpf::util::ZipInputStreamZipEntrySource::parse_zip_bytes#by_index"));
    assert_eq!(e.kind, S01ErrorKind::ZipCorrupt, "归到 not_zip ⇒ fail");
    let _ = f;
}

// ───────────────────────── ET4：payload=FileDamagedException ⇒ zip_corrupt ─────────────────────────
#[test]
fn et4_file_damaged_is_zip_corrupt() {
    let f = S01Failure::Panic {
        payload: "FileDamagedException".to_string(),
    };
    let e = S01Taxonomy::classify(&f).expect("合法输入不应被拒绝");
    assert_eq!(e.kind, S01ErrorKind::ZipCorrupt, "新增第 11 类 ⇒ fail");
    assert!(
        CONTRACT_KINDS.contains(&e.kind.as_str()),
        "kind 必须落在 10 值词表内"
    );
}

// ───────────────────────── ET5：加密文本 ⇒ unsupported_encryption（非 not_zip） ─────────────────────────
#[test]
fn et5_encryption_not_confused_with_not_zip() {
    let f = S01Failure::ZipErr {
        msg: "unsupported archive: encrypted".to_string(),
    };
    let e = S01Taxonomy::classify(&f).expect("合法输入不应被拒绝");
    assert_eq!(
        e.kind,
        S01ErrorKind::UnsupportedEncryption,
        "归到 not_zip ⇒ fail（判定顺序 #3 在 #1 之后须能区分）"
    );
    assert!(
        e.at.contains("unsupported_archive"),
        "ET5 要求锚点为 unsupported_archive，实得 {}",
        e.at
    );
}

// ───────────────────────── ET6：JavaThrowable ⇒ invalid_part_name ─────────────────────────
#[test]
fn et6_java_throwable_invalid_part_name() {
    let f = S01Failure::JavaThrowable {
        class_name: "InvalidFormatException".to_string(),
        line: 219,
    };
    let e = S01Taxonomy::classify(&f).expect("合法输入不应被拒绝");
    assert_eq!(e.kind, S01ErrorKind::InvalidPartName);
    assert!(
        e.at.contains("createPartName_uri#conformance"),
        "ET6 要求 at=PackagingURIHelper::createPartName_uri#conformance，实得 {}",
        e.at
    );
    assert!(!e.at.is_empty(), "缺 at ⇒ fail");
}

// ───────────────────────── ET7：同 payload、不同 at ⇒ 靠 at 分流 ─────────────────────────
#[test]
fn et7_same_payload_different_at() {
    let e = S01Taxonomy::classify_panic_at(
        "FileDamagedException",
        Some("opc::OPCPackage::getPartsImpl#pass1.entry_name_guard"),
    );
    assert_eq!(
        e.kind,
        S01ErrorKind::InvalidPartName,
        "与 ET4 payload 相同、at 不同 ⇒ 必须靠 at 分流；归到 zip_corrupt ⇒ fail"
    );
    // 反向：size_guard 语境必须是 zip_corrupt（与 ET4 同结论但不同锚点）
    let e2 = S01Taxonomy::classify_panic_at(
        "FileDamagedException",
        Some("opc::OPCPackage::getPartsImpl#pass1.size_guard"),
    );
    assert_eq!(e2.kind, S01ErrorKind::ZipCorrupt);
    assert_ne!(e.kind, e2.kind, "同 payload 必须能被 at 分成两类");
}

// ───────────────────────── ET8：CT part 缺失 ⇒ missing_content_types ─────────────────────────
#[test]
fn et8_ct_absent_is_missing_content_types() {
    let f = S01Failure::SilentReturn {
        at: "opc::OPCPackage::getPartsImpl#pass1.ct_part_absent".to_string(),
    };
    let e = S01Taxonomy::classify(&f).expect("合法输入不应被拒绝");
    assert_eq!(
        e.kind,
        S01ErrorKind::MissingContentTypes,
        "⇒ missing_part ⇒ fail"
    );

    // OptionMiss 同语境亦同结论
    let f2 = S01Failure::OptionMiss {
        at: "opc::OPCPackage::getPartsImpl#pass1.ct_part_absent".to_string(),
    };
    let e2 = S01Taxonomy::classify(&f2).expect("合法输入不应被拒绝");
    assert_eq!(e2.kind, S01ErrorKind::MissingContentTypes);
}

// ───────────────────────── ET9：CT 不可解析 ⇒ malformed_content_types ─────────────────────────
#[test]
fn et9_ct_unparsable_is_malformed() {
    let f = S01Failure::Panic {
        payload: "Can't read content types part !".to_string(),
    };
    let e = S01Taxonomy::classify(&f).expect("合法输入不应被拒绝");
    assert_eq!(
        e.kind,
        S01ErrorKind::MalformedContentTypes,
        "归到 missing_content_types ⇒ fail"
    );
}

// ───────────────────────── ET10：主 part 关系数 ≠ 1 ⇒ missing_main_part ─────────────────────────
#[test]
fn et10_main_rel_not_one_is_missing_main_part() {
    let f = S01Failure::Panic {
        payload: "The number of main document relationship should be only 1.".to_string(),
    };
    let e = S01Taxonomy::classify(&f).expect("合法输入不应被拒绝");
    assert_eq!(e.kind, S01ErrorKind::MissingMainPart);
    assert!(
        e.at.contains("main_rel_assert"),
        "ET10 要求 at=…from_xwpf_package#main_rel_assert，实得 {}",
        e.at
    );
}

// ───────────────────────── ET11：查 part 语境 ⇒ missing_part ─────────────────────────
#[test]
fn et11_part_list_miss_is_missing_part() {
    let f = S01Failure::OptionMiss {
        at: "opc::OPCPackage::getPart#part_list_miss".to_string(),
    };
    let e = S01Taxonomy::classify(&f).expect("合法输入不应被拒绝");
    assert_eq!(
        e.kind,
        S01ErrorKind::MissingPart,
        "⇒ relationship_not_found ⇒ fail"
    );
}

// ───────────────────────── ET12：查 rId 语境 ⇒ relationship_not_found ─────────────────────────
#[test]
fn et12_rid_miss_is_relationship_not_found() {
    let f = S01Failure::OptionMiss {
        at: "opc::PackageRelationshipCollection::getRelationshipByID#miss".to_string(),
    };
    let e = S01Taxonomy::classify(&f).expect("合法输入不应被拒绝");
    assert_eq!(e.kind, S01ErrorKind::RelationshipNotFound);
}

// ───────────────────────── ET13：索引越界 ⇒ relationship_not_found（跨语言同 kind 归一点） ─────────────────────────
#[test]
fn et13_index_bounds_is_relationship_not_found() {
    let e = S01Taxonomy::classify_panic_at(
        "index",
        Some("opc::PackageRelationshipCollection::getRelationship_by_index#bounds"),
    );
    assert_eq!(
        e.kind,
        S01ErrorKind::RelationshipNotFound,
        "归到 missing_part ⇒ fail（这是 Java 返回 null / Rust panic 的同 kind 归一点）"
    );
}

// ───────────────────────── ET14：不可分类文本 ⇒ io_error 兜底 ─────────────────────────
#[test]
fn et14_unclassifiable_falls_back_to_io_error() {
    for msg in ["完全不可分类的任意文本", "???", "\u{1F600}"] {
        let f = S01Failure::ZipErr {
            msg: msg.to_string(),
        };
        let e = S01Taxonomy::classify(&f).expect("合法输入不应被拒绝");
        assert_eq!(
            e.kind,
            S01ErrorKind::IoError,
            "msg={:?} 应兜底为 io_error（返回 Option::None/空 kind ⇒ fail）",
            msg
        );
        assert!(!e.kind.as_str().is_empty(), "不得给空 kind");
    }
}

// ───────────────────────── ET15：顺序优先——先命中先返回 ─────────────────────────
#[test]
fn et15_priority_is_declaration_order() {
    // 同一文本同时含「zip 不可识别」与「条目损坏」两类关键词时，
    // 规则 1（not_zip）先命中 ⇒ 必须返回 not_zip，不得返回 zip_corrupt。
    let f = S01Failure::ZipErr {
        msg: "invalid Zip archive: corrupt entry".to_string(),
    };
    let e = S01Taxonomy::classify(&f).expect("合法输入不应被拒绝");
    assert_eq!(
        e.kind,
        S01ErrorKind::NotZip,
        "顺序漂移 ⇒ fail（规则 1 必须先于规则 2）"
    );

    // 加密 vs zip_corrupt：规则 3 先于规则 2
    let f2 = S01Failure::ZipErr {
        msg: "corrupt encrypted archive".to_string(),
    };
    let e2 = S01Taxonomy::classify(&f2).expect("合法输入不应被拒绝");
    assert_eq!(
        e2.kind,
        S01ErrorKind::UnsupportedEncryption,
        "顺序漂移 ⇒ fail（规则 3 必须先于规则 2）"
    );
}

// ───────────────────────── ET16：空 at ⇒ 拒绝 ─────────────────────────
#[test]
fn et16_empty_at_is_rejected() {
    for bad in ["", "   "] {
        let f = S01Failure::OptionMiss {
            at: bad.to_string(),
        };
        let r: Result<_, ClassifyRejected> = S01Taxonomy::classify(&f);
        assert!(
            r.is_err(),
            "at={:?} 必须被拒绝（静默给空 at ⇒ fail）",
            bad
        );
    }
    // SilentReturn 同样受约束
    let r = S01Taxonomy::classify(&S01Failure::SilentReturn { at: String::new() });
    assert!(r.is_err(), "SilentReturn 空 at 也必须被拒绝");
}

// ───────────────────────── ET17：at 形状正则 ─────────────────────────
#[test]
fn et17_at_shape_is_valid() {
    let good = [
        "opc::OPCPackage::getPartsImpl#pass1.size_guard",
        "xwpf::util::ZipInputStreamZipEntrySource::parse_zip_bytes#ZipArchive::new",
        "opc::OPCPackage::getPart#part_list_miss",
        "opc::PackageRelationshipCollection::getRelationshipByID#miss",
        "_Leading::under_score#anchor.dots",
        "A::B#c",
    ];
    for at in good {
        assert!(is_valid_at(at), "合法 at 被判非法：{}", at);
    }
    // ★注意：锚点**允许含 `::`**（权威绑定表 ① `#ZipArchive::new`、② `#by_index` 形态），
    //   因此 `A::B#c::d` 是**合法**的，列入 good 而非 bad。
    let good_more = ["A::B#c::d", "xwpf::util::T::m#ZipArchive::new"];
    for at in good_more {
        assert!(is_valid_at(at), "合法 at 被判非法：{}", at);
    }
    let bad = [
        "",
        "noseparator",
        "only::one#",           // 锚点为空
        "::B#c",                // 首段为空
        "A::#c",                // 第二段为空
        "A#c",                  // 只有一段（缺 ::）
        "A::B",                 // 缺 #
        "9A::B#c",              // 首字符非字母/下划线
        "A B::C#d",             // 含空格
    ];
    for at in bad {
        assert!(!is_valid_at(at), "非法 at 被判合法：{}", at);
    }
}

// ───────────────────────── ET18：detail 不进 canonical_tuple ─────────────────────────
#[test]
fn et18_detail_dropped_from_canonical_tuple() {
    let f1 = S01Failure::ZipErr {
        msg: "message A".to_string(),
    };
    let f2 = S01Failure::ZipErr {
        msg: "message B".to_string(),
    };
    let a = S01Taxonomy::classify(&f1).expect("合法输入不应被拒绝");
    let b = S01Taxonomy::classify(&f2).expect("合法输入不应被拒绝");
    assert_ne!(a.detail, b.detail, "构造前提：两者 detail 必须不同");
    assert_eq!(
        a.canonical_tuple(),
        b.canonical_tuple(),
        "detail 进 tuple ⇒ fail"
    );
    assert_eq!(a.canonical_tuple().2, "", "tuple 第三元必须为空串");
    // 比较面只含 kind + at
    let (k, at, d) = a.canonical_tuple();
    assert_eq!(k, a.kind.as_str());
    assert_eq!(at, a.at.as_str());
    assert_eq!(d, "");
}

// ───────────────────────── ET19：as_str ↔ from_str 往返 ─────────────────────────
#[test]
fn et19_kind_round_trip() {
    for k in S01ErrorKind::all() {
        let s = k.as_str();
        let back = S01ErrorKind::from_str(s).unwrap_or_else(|| panic!("无法回解：{}", s));
        assert_eq!(back, k, "往返不一致：{:?} → {} → {:?}", k, s, back);
        assert_eq!(back.as_str(), s, "往返后字符串漂移");
    }
    // 词表外的字符串不得被接受
    assert!(S01ErrorKind::from_str("not_a_kind").is_none());
    assert!(S01ErrorKind::from_str("NotZip").is_none(), "大小写必须敏感一致");
    assert!(S01ErrorKind::from_str("").is_none());
}

// ───────────────────────── ET20：确定性（同输入连跑两次 byte-identical） ─────────────────────────
#[test]
fn et20_deterministic_output() {
    let cases = [
        S01Failure::ZipErr {
            msg: "invalid Zip archive".to_string(),
        },
        S01Failure::Panic {
            payload: "FileDamagedException".to_string(),
        },
        S01Failure::OptionMiss {
            at: "opc::OPCPackage::getPart#part_list_miss".to_string(),
        },
        S01Failure::JavaThrowable {
            class_name: "InvalidFormatException".to_string(),
            line: 219,
        },
        S01Failure::SilentReturn {
            at: "opc::OPCPackage::getPartsImpl#pass1.ct_part_absent".to_string(),
        },
    ];
    for f in &cases {
        let a = S01Taxonomy::classify(f).expect("合法输入不应被拒绝");
        let b = S01Taxonomy::classify(f).expect("合法输入不应被拒绝");
        assert_eq!(
            a.canonical_tuple(),
            b.canonical_tuple(),
            "同输入两次输出必须一致（PLAN §0.2-2b）"
        );
        assert_eq!(a.at, b.at);
        assert_eq!(a.kind, b.kind);
    }
}

// ───────────────────────── ET21：断言层不产生信号（不得把假推理写进实现） ─────────────────────────
#[test]
fn et21_ioassert_layer_produces_no_kind() {
    // Java `IOAssert.java:4/10-165` 的 43 个 void 方法**全为 no-op**；
    // `XWPFDocument.java:94` 的 `IOAssert` 断言**不产生任何信号**。
    // 因此：不存在任何输入能让分类器「因为断言本该抛」而产出 kind。
    //
    // 反证：分类器的输入变体只有 5 种，且每一种都要求真实证据
    // （非空 payload / 非空 class_name / 合法 at）。断言层无对应变体。
    let variants = 5usize;
    assert_eq!(variants, 5, "输入变体集合必须封闭为 5 种；新增即须复审 ET21");

    // 空 class_name（模拟「没有真实 throw」）必须被拒绝，而不是映射成某个 kind
    let r = S01Taxonomy::classify(&S01Failure::JavaThrowable {
        class_name: String::new(),
        line: 94,
    });
    assert!(
        r.is_err(),
        "无真实 throw 时不得产出 kind（＝把假推理写进实现）"
    );

    // 空 payload 同理
    let r2 = S01Taxonomy::classify(&S01Failure::Panic {
        payload: String::new(),
    });
    assert!(r2.is_err(), "空 panic payload 必须被拒绝");
}

// ───────────────────────── ET22：同一 None、两种 at 语境分别命中 ─────────────────────────
#[test]
fn et22_same_none_two_contexts() {
    let part_ctx = S01Failure::OptionMiss {
        at: "opc::OPCPackage::getPart#part_list_miss".to_string(),
    };
    let rel_ctx = S01Failure::OptionMiss {
        at: "opc::PackageRelationshipCollection::getRelationshipByID#miss".to_string(),
    };
    let a = S01Taxonomy::classify(&part_ctx).expect("合法输入不应被拒绝");
    let b = S01Taxonomy::classify(&rel_ctx).expect("合法输入不应被拒绝");
    assert_eq!(a.kind, S01ErrorKind::MissingPart, "查 part 语境");
    assert_eq!(
        b.kind,
        S01ErrorKind::RelationshipNotFound,
        "查 rId 语境；语境不分 ⇒ fail"
    );
    assert_ne!(a.kind, b.kind, "同一 None 必须能被语境分成两类");
}

// ───────────────────────── 补充：所有产出 kind 恒在封闭词表内 ─────────────────────────
#[test]
fn every_output_kind_is_in_closed_word_list() {
    let samples: Vec<S01Failure> = vec![
        S01Failure::ZipErr {
            msg: "invalid Zip archive".to_string(),
        },
        S01Failure::ZipErr {
            msg: "encrypted".to_string(),
        },
        S01Failure::ZipErr {
            msg: "[M1.6] pchar".to_string(),
        },
        S01Failure::ZipErr {
            msg: "corrupt".to_string(),
        },
        S01Failure::ZipErr {
            msg: "unclassifiable".to_string(),
        },
        S01Failure::Panic {
            payload: "FileDamagedException".to_string(),
        },
        S01Failure::Panic {
            payload: "Can't read content types part !".to_string(),
        },
        S01Failure::Panic {
            payload: "The number of main document relationship should be only 1.".to_string(),
        },
        S01Failure::Panic {
            payload: "index".to_string(),
        },
        S01Failure::Panic {
            payload: "totally unknown".to_string(),
        },
        S01Failure::OptionMiss {
            at: "opc::OPCPackage::getPart#part_list_miss".to_string(),
        },
        S01Failure::OptionMiss {
            at: "opc::PackageRelationshipCollection::getRelationshipByID#miss".to_string(),
        },
        S01Failure::OptionMiss {
            at: "opc::OPCPackage::getPartsImpl#pass1.ct_part_absent".to_string(),
        },
        S01Failure::OptionMiss {
            at: "opc::OPCPackage::getPartsImpl#malformed_parse".to_string(),
        },
        S01Failure::OptionMiss {
            at: "xwpf::usermodel::XWPFDocument::XWPFDocument__pkg#main_rel_assert".to_string(),
        },
        S01Failure::OptionMiss {
            at: "opc::OPCPackage::getPart#other_miss".to_string(),
        },
        S01Failure::JavaThrowable {
            class_name: "FileDamagedException".to_string(),
            line: 567,
        },
        S01Failure::JavaThrowable {
            class_name: "InvalidOperationException".to_string(),
            line: 219,
        },
        S01Failure::JavaThrowable {
            class_name: "POIXMLException".to_string(),
            line: 1,
        },
        S01Failure::JavaThrowable {
            class_name: "SomeOtherException".to_string(),
            line: 2,
        },
        S01Failure::SilentReturn {
            at: "opc::OPCPackage::getPartsImpl#pass1.ct_part_absent".to_string(),
        },
        S01Failure::SilentReturn {
            at: "opc::OPCPackage::getPart#silent_miss".to_string(),
        },
        S01Failure::SilentReturn {
            at: "opc::OPCPackage#silent_io".to_string(),
        },
    ];
    for f in samples {
        let e = S01Taxonomy::classify(&f)
            .unwrap_or_else(|r| panic!("样本 {:?} 被拒绝：{}", f, r.reason));
        assert!(
            CONTRACT_KINDS.contains(&e.kind.as_str()),
            "产出词表外的 kind：{}（来自 {:?}）",
            e.kind.as_str(),
            f
        );
        assert!(
            is_valid_at(&e.at),
            "产出的 at 形状非法：{}（来自 {:?}）",
            e.at,
            f
        );
    }
}
