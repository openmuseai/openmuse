use anyhow::{Context, Error, Result};
use helix_loader::VERSION_AND_GIT_HASH;
use helix_term::application::Application;
use helix_term::args::Args;
use helix_term::config::{Config, ConfigLoadError};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

fn openmuse_trace(event: &str, start: Instant) {
    let Ok(path) = std::env::var("OPENMUSE_HELIX_TRACE") else {
        return;
    };
    use std::io::Write;
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let at_unix_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |duration| duration.as_millis());
        let _ = writeln!(
            file,
            "{}",
            serde_json::json!({
                "at_unix_ms": at_unix_ms,
                "source": "hx",
                "event": event,
                "pid": std::process::id(),
                "elapsed_ms": start.elapsed().as_millis(),
            })
        );
    }
}

fn setup_logging(verbosity: u64) -> Result<()> {
    let level = match verbosity {
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _3_or_more => log::LevelFilter::Trace,
    };

    helix_term::logging::init_file(level, &helix_loader::log_file())?;

    Ok(())
}

/// ConPTY on a Chinese Windows install starts at code page 936. Helix writes
/// UTF-8, and that code page treats the following bytes as one GBK character,
/// so the editor surface turns into mojibake. UTF-8 (65001) keeps ASCII and
/// the drawing sequences intact for the host terminal.
#[cfg(windows)]
fn enable_utf8_console() {
    #[link(name = "kernel32")]
    extern "system" {
        fn SetConsoleOutputCP(code_page: u32) -> i32;
        fn SetConsoleCP(code_page: u32) -> i32;
    }
    unsafe {
        SetConsoleOutputCP(65001);
        SetConsoleCP(65001);
    }
}

fn main() -> Result<()> {
    #[cfg(windows)]
    enable_utf8_console();
    let exit_code = main_impl()?;
    std::process::exit(exit_code);
}

#[tokio::main]
async fn main_impl() -> Result<i32> {
    let start = Instant::now();
    let args = Args::parse_args().context("could not parse arguments")?;
    openmuse_trace("args_parsed", start);

    helix_loader::initialize_config_file(args.config_file.clone());
    helix_loader::initialize_log_file(args.log_file.clone());

    // Help has a higher priority and should be handled separately.
    if args.display_help {
        print!(
            "\
{} {}
{}
{}

USAGE:
    hx [FLAGS] [files]...

ARGS:
    <files>...    Set the input file to use, position can also be specified via file[:row[:col]]

FLAGS:
    -h, --help                     Print help information
    --strict                       Bail on error for commands that can fail.
    --tutor                        Load the tutorial
    --health [CATEGORY]            Check for potential errors in editor setup
                                   CATEGORY can be a language or one of 'clipboard', 'languages',
                                   'all-languages' or 'all'. 'languages' is filtered according to
                                   user config, 'all-languages' and 'all' are not. If not specified,
                                   the default is the same as 'all', but with languages filtering.
    -g, --grammar {{fetch|build}}    Fetch or builds tree-sitter grammars listed in languages.toml.
    -c, --config <file>            Specify a file to use for configuration
    -v                             Increase logging verbosity each use for up to 3 times
    --log <file>                   Specify a file to use for logging
                                   (default file: {})
    -V, --version                  Print version information
    --vsplit                       Split all given files vertically into different windows
    --hsplit                       Split all given files horizontally into different windows
    -w, --working-dir <path>       Specify an initial working directory
    +[N]                           Open the first given file at line number N, or the last line, if
                                   N is not specified.
",
            env!("CARGO_PKG_NAME"),
            VERSION_AND_GIT_HASH,
            env!("CARGO_PKG_AUTHORS"),
            env!("CARGO_PKG_DESCRIPTION"),
            helix_loader::default_log_file().display(),
        );
        std::process::exit(0);
    }

    if args.display_version {
        println!("helix {} openmuse-nonmodal.3", VERSION_AND_GIT_HASH);
        std::process::exit(0);
    }

    if args.health {
        if let Err(err) = helix_term::health::print_health(args.health_arg) {
            // Piping to for example `head -10` requires special handling:
            // https://stackoverflow.com/a/65760807/7115678
            if err.kind() != std::io::ErrorKind::BrokenPipe {
                return Err(err.into());
            }
        }

        std::process::exit(0);
    }

    if args.fetch_grammars {
        helix_loader::grammar::fetch_grammars(args.strict)?;
        return Ok(0);
    }

    if args.build_grammars {
        helix_loader::grammar::build_grammars(None, args.strict)?;
        return Ok(0);
    }

    setup_logging(args.verbosity).context("failed to initialize logging")?;

    // NOTE: Set the working directory early so the correct configuration is loaded. Be aware that
    // Application::new() depends on this logic so it must be updated if this changes.
    if let Some(path) = &args.working_directory {
        helix_stdx::env::set_current_working_dir(path)?;
    } else if let Some((path, _)) = args.files.first().filter(|p| p.0.is_dir()) {
        // If the first file is a directory, it will be the working directory unless -w was specified
        helix_stdx::env::set_current_working_dir(path)?;
    } else if let Err(err) = std::env::current_dir() {
        eprintln!("Couldn't determine the current working directory: {err}");
        eprintln!("Check that it still exists, or pass an initial directory with `--working-dir`");
        return Ok(1);
    }

    let config = match Config::load_default() {
        Ok(config) => config,
        Err(ConfigLoadError::Error(err)) if err.kind() == std::io::ErrorKind::NotFound => {
            Config::default()
        }
        Err(ConfigLoadError::Error(err)) => return Err(Error::new(err)),
        Err(ConfigLoadError::BadConfig(err)) => {
            eprintln!("Bad config: {}", err);
            eprintln!("Press <ENTER> to continue with default config");
            use std::io::Read;
            let _ = std::io::stdin().read(&mut []);
            Config::default()
        }
    };
    openmuse_trace("config_loaded", start);

    let workspace_trust =
        helix_loader::workspace_trust::WorkspaceTrust::new((&config.editor.workspace_trust).into());

    let lang_loader =
        helix_core::config::user_lang_loader(&workspace_trust).unwrap_or_else(|err| {
            eprintln!("{}", err);
            eprintln!("Press <ENTER> to continue with default language config");
            use std::io::Read;
            // This waits for an enter press.
            let _ = std::io::stdin().read(&mut []);
            helix_core::config::default_lang_loader()
        });
    openmuse_trace("languages_loaded", start);

    // TODO: use the thread local executor to spawn the application task separately from the work pool
    let mut app = Application::new(args, config, lang_loader, workspace_trust)
        .context("unable to start Helix")?;
    openmuse_trace("application_created", start);
    let mut events = app.event_stream();

    let exit_code = app.run(&mut events).await?;

    Ok(exit_code)
}
