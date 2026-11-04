use ppt_core::cn::wps::drawing::dmlr::model::HandlerCallback::{
    HandlerCallback, HandlerCallback as default_state, HandlerCallbackState, HandlerCallback__mode,
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Fixture {
    value: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NoopCallback {
    state: HandlerCallbackState,
}

macro_rules! callback_types {
    () => {
        type DiagramPair = Fixture;
        type Blip = Fixture;
        type Color = Fixture;
        type Hyperlink = Fixture;
        type Media = Fixture;
        type AudioFile = Fixture;
        type QuickTimeFile = Fixture;
        type VideoFile = Fixture;
        type EmbeddedWAVAudioFile = Fixture;
        type Model3D = Fixture;
        type Shape = Fixture;
        type ShapePr = Fixture;
        type ShapeStyle = Fixture;
        type TextRun = Fixture;
        type TagLst = Fixture;
        type FileManager = Fixture;
    };
}

impl HandlerCallback for NoopCallback {
    callback_types!();

    fn callback_state(&self) -> &HandlerCallbackState {
        &self.state
    }
}

#[test]
fn constructors_and_mode_predicates_match_java() {
    assert_eq!(default_state().mode, 0);

    let cases = [
        (-1, [false, false, false, false]),
        (0, [true, false, false, false]),
        (1, [false, true, false, false]),
        (2, [false, false, true, false]),
        (3, [false, false, false, true]),
        (4, [false, false, false, false]),
    ];
    for (mode, expected) in cases {
        let callback = NoopCallback {
            state: HandlerCallback__mode(mode),
        };
        assert_eq!(
            [
                callback.isPPTX(),
                callback.isPPT14Slides(),
                callback.isGVml(),
                callback.isArtTable(),
            ],
            expected,
            "mode={mode}"
        );
    }
}

#[test]
fn identity_sentinels_and_null_collectors_match_java() {
    let mut callback = NoopCallback {
        state: default_state(),
    };
    for id in [None, Some(-1), Some(0), Some(42)] {
        assert_eq!(callback.getCoreShapeId(id), id);
    }

    let mut files = Fixture { value: 9 };
    assert_eq!(callback.onOleObject(None), -1);
    assert_eq!(callback.onOleObject(Some("rId7")), -1);
    assert_eq!(callback.onInk(None, None), -1);
    assert_eq!(callback.onInk(Some("rId8"), Some(&mut files)), -1);
    assert!(callback.getChartMap().is_none());
    assert!(callback.getDiagramPairs().is_none());
    assert_eq!(files.value, 9);
}

#[test]
fn every_default_hook_is_a_no_op() {
    let mut callback = NoopCallback {
        state: HandlerCallback__mode(3),
    };
    let before_callback = callback.clone();
    let mut model = Fixture { value: 17 };
    let before_model = model.clone();
    let style = Fixture { value: 23 };

    callback.load();
    callback.onTags(Some("tag"), Some(&mut model));
    callback.onBlipEmbed(Some("embed"), Some(&mut model));
    callback.onBlipLink(Some("link"), Some(&mut model));
    callback.onAudioFileLink(Some("audio"), Some(&mut model));
    callback.onExtMediaFileLink(Some("media"), Some(&mut model));
    callback.onQuickTimeFileLink(Some("quick"), Some(&mut model));
    callback.onVideoFileLink(Some("video"), Some(&mut model));
    callback.onEmbeddedWAVAudioFileEmbed(Some("wav"), Some(&mut model));
    callback.onHyperlinkRid(Some("rid"), Some(&mut model));
    callback.onHyperlinkAction(Some("action"), Some(&mut model));
    callback.onShapeId(Some(1), Some(2));
    callback.onConnectorShape(Some(3));
    callback.onChartRelationShip(Some("chart"), Some(&mut model));
    callback.addDiagramPair(
        4,
        Some("colors"),
        Some("data"),
        Some("layout"),
        Some("quick"),
    );
    callback.onModel3D(Some("model"), Some(&mut model));
    callback.onAdjustThemeColor(Some(&mut model), 5);
    callback.onMergeThemeTypeface(Some(&mut model));
    callback.onConvertStyle2ShapeProps(Some(&style), Some(&mut model));

    // Repeat with nullable Java inputs to lock down the other observable boundary.
    callback.onTags(None, None);
    callback.onBlipEmbed(None, None);
    callback.onBlipLink(None, None);
    callback.onAudioFileLink(None, None);
    callback.onExtMediaFileLink(None, None);
    callback.onQuickTimeFileLink(None, None);
    callback.onVideoFileLink(None, None);
    callback.onEmbeddedWAVAudioFileEmbed(None, None);
    callback.onHyperlinkRid(None, None);
    callback.onHyperlinkAction(None, None);
    callback.onShapeId(None, None);
    callback.onConnectorShape(None);
    callback.onChartRelationShip(None, None);
    callback.addDiagramPair(0, None, None, None, None);
    callback.onModel3D(None, None);
    callback.onAdjustThemeColor(None, 0);
    callback.onMergeThemeTypeface(None);
    callback.onConvertStyle2ShapeProps(None, None);

    assert_eq!(callback, before_callback);
    assert_eq!(model, before_model);
}

#[derive(Debug)]
struct RecordingCallback {
    state: HandlerCallbackState,
    events: Vec<String>,
}

impl HandlerCallback for RecordingCallback {
    callback_types!();

    fn callback_state(&self) -> &HandlerCallbackState {
        &self.state
    }

    fn load(&mut self) {
        self.events.push("load".into());
    }

    fn onShapeId(&mut self, origin: Option<i32>, core: Option<i32>) {
        self.events.push(format!("shape:{origin:?}->{core:?}"));
    }

    fn onOleObject(&mut self, rid: Option<&str>) -> i32 {
        self.events.push(format!("ole:{rid:?}"));
        77
    }
}

type DynCallback<'a> = dyn HandlerCallback<
        DiagramPair = Fixture,
        Blip = Fixture,
        Color = Fixture,
        Hyperlink = Fixture,
        Media = Fixture,
        AudioFile = Fixture,
        QuickTimeFile = Fixture,
        VideoFile = Fixture,
        EmbeddedWAVAudioFile = Fixture,
        Model3D = Fixture,
        Shape = Fixture,
        ShapePr = Fixture,
        ShapeStyle = Fixture,
        TextRun = Fixture,
        TagLst = Fixture,
        FileManager = Fixture,
    > + 'a;

fn drive_through_trait_object(callback: &mut DynCallback<'_>) -> i32 {
    callback.load();
    callback.onShapeId(Some(11), Some(22));
    callback.onOleObject(Some("rId5"))
}

#[test]
fn overrides_dispatch_dynamically_through_callback_trait() {
    let mut callback = RecordingCallback {
        state: HandlerCallback__mode(2),
        events: Vec::new(),
    };

    assert_eq!(drive_through_trait_object(&mut callback), 77);
    assert_eq!(
        callback.events,
        ["load", "shape:Some(11)->Some(22)", "ole:Some(\"rId5\")"]
    );
    assert!(callback.isGVml());
}
