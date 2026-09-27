#[cfg(feature = "integration")]
mod test {
    mod helpers;

    use helix_core::{syntax::config::AutoPairConfig, Selection};
    use helix_term::config::{Config, ConfigLoadError};

    use indoc::indoc;

    use self::helpers::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn openmuse_nonmodal_types_without_normal_dispatch_and_checkpoints() -> anyhow::Result<()>
    {
        use helix_loader::workspace_trust::WorkspaceTrust;
        use helix_term::{application::Application, args::Args};
        use helix_view::{current, document::Mode, editor::InputProfile, input::parse_macro};
        use tokio_stream::wrappers::UnboundedReceiverStream;

        #[cfg(windows)]
        use crossterm::event::{Event, KeyEvent};
        #[cfg(not(windows))]
        use termina::event::{Event, KeyEvent};

        let parsed = Config::load(
            Ok(&"[editor]\ninput-profile = \"standard-nonmodal\"".to_string()),
            Err(ConfigLoadError::default()),
        )
        .expect("versioned input profile must parse");
        assert_eq!(parsed.editor.input_profile, InputProfile::StandardNonmodal);
        assert!(Config::load(
            Ok(&"[editor]\ninput-profile = \"unknown\"".to_string()),
            Err(ConfigLoadError::default()),
        )
        .is_err());

        let mut config = test_config();
        config.editor.input_profile = InputProfile::StandardNonmodal;
        let mut app = Application::new(
            Args::default(),
            config,
            test_syntax_loader(None),
            WorkspaceTrust::fully_trusted(),
        )?;
        assert_eq!(app.editor.mode(), Mode::Insert);

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let mut input = UnboundedReceiverStream::new(rx);
        for key in parse_macro("i:")? {
            tx.send(Ok(Event::Key(KeyEvent::from(key))))?;
        }
        assert!(app.event_loop_until_idle(&mut input).await);
        assert_eq!(app.editor.mode(), Mode::Insert);
        let (_, doc) = current!(app.editor);
        assert!(doc.text().to_string().starts_with("i:"));

        for key in parse_macro("<esc>z")? {
            tx.send(Ok(Event::Key(KeyEvent::from(key))))?;
        }
        assert!(app.event_loop_until_idle(&mut input).await);
        assert_eq!(app.editor.mode(), Mode::Insert);
        let (view, doc) = current!(app.editor);
        assert!(doc.text().to_string().starts_with("i:z"));
        assert!(doc.undo(view));
        assert!(doc.text().to_string().starts_with("i:"));
        let errors = app.close().await;
        assert!(errors.is_empty());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn hello_world() -> anyhow::Result<()> {
        test(("#[\n|]#", "ihello world<esc>", "hello world#[|\n]#")).await?;
        Ok(())
    }

    mod auto_pairs;
    mod command_line;
    mod commands;
    mod movement;
    mod splits;
}
