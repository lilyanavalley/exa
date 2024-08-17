
use std::{fmt::Display, path::PathBuf};
use clap::*;



#[derive(Parser, Debug)]
#[command(about, version)]
pub struct Cli {
    
    /// Headless Mode
    /// 
    /// Specifies if the application is to be run in `headless` mode, i.e. without a graphical window.
    #[arg(
        short('H'),
        long,
        default_value_t = false
    )]
    headless:       bool,

    /// Tracing Mode
    /// 
    /// Enable tracing and profiling features using Tracy profiler.
    #[arg(
        short('t'),
        long,
        default_value_t = false
    )]
    tracing:        bool,

    /// Game Scene Override
    /// 
    /// Specifies a path to a game scene file to be dynamically loaded and executed when the game launches.
    #[arg(
        short('s'),
        long
    )]
    scene:          Option<PathBuf>,

    /// Game Update Rate
    /// 
    /// Specifies the update rate.
    #[arg(
        short('R'),
        long,
        default_value_t = 60.0
    )]
    rate:           f32,

    /// Launch with Vertical Sync
    /// 
    /// Reduce screen tearing by locking the refresh rate to your monitor's refresh rate.
    #[arg(
        short('v'),
        long,
        default_value_t = true
    )]
    vsync:          bool,

    /// Multisampling level (MSAA)
    /// 
    /// Multisampling level, with options including `off`, `x2`, `x4`, and `x8`.
    #[arg(
        short('m'),
        long,
        default_value_t = MsaaLevel::default()
    )]
    msaa:           MsaaLevel,

    /// Developer Overlay Mode
    /// 
    /// Enable developer overlay feature.
    #[arg(
        short('d'),
        long,
        default_value_t = false
    )]
    developer:      bool,

    #[command(subcommand)]
    _subcommand:    Option<Subcommands>

    // /// Verbosity
    // /// 
    // /// Adjusts the verbosity of the application StdOut / log level.
    // /// 
    // /// *Possible options include, in order from most verbose to least verbose*: `trace`, `debug`, `info`, `warn`, `error`.
    // /// 
    // /// This application runs with a default of `error` if not otherwise set; therefore all messages bellow `error` will not be output.
    // #[arg(
    //     long,
    //     default_value_t = tracing::Level::ERROR,
    //     value_parser = clap::builder::PossibleValuesParser::new(["trace", "debug", "info", "warn", "error"])
    //     .map(|s| s.parse::<TracingLevelMapper>().unwrap())
    // )]
    // verbosity:      tracing::Level,

}

impl Cli {

    pub fn headless(&self) -> bool {
        self.headless
    }

    pub fn tracing(&self) -> bool {
        self.tracing
    }

    pub fn rate(&self) -> f32 {
        self.rate
    }

    pub fn msaa(&self) -> MsaaLevel {
        self.msaa
    }

    pub fn developer(&self) -> bool {
        self.developer
    }

    pub fn vsync(&self) -> bool {
        self.vsync
    }

}

#[derive(Subcommand, Debug)]
pub enum Subcommands {

    

}

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum MsaaLevel {
    Off,
    X2,
    X4,
    X8
}

impl MsaaLevel {

    const LANG_TITLE_EN_US: &'static str = "Multisampling Anti-Aliasing (MSAA)";
    const LANG_DESCRIPTION_EN_US: &'static str = "
        Reduces jaggies and improves visual quality on rendered frames by performing multiple render passes, per frame.

        MSAA levels include x2 (two), x4 (four) and x8 (eight) passes. Higher MSAA levels perform best on beefier machines. Lower MSAA levels trade visual quality for better performance.
    ";

    /// Convert into fyrox MSAA level.
    pub fn into(&self) -> Option<u8> {
        match self {
            &MsaaLevel::Off     => None,
            &MsaaLevel::X2      => Some(2),
            &MsaaLevel::X4      => Some(4),
            &MsaaLevel::X8      => Some(8),
        }
    }
}

impl Display for MsaaLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &MsaaLevel::Off     => f.write_str("off"),
            &MsaaLevel::X2      => f.write_str("x2"),
            &MsaaLevel::X4      => f.write_str("x4"),
            &MsaaLevel::X8      => f.write_str("x8"),
        }
    }
}

impl Default for MsaaLevel {
    fn default() -> Self {
        Self::Off
    }
}

#[derive(Debug, Copy, Clone, ValueEnum)]
enum TracingLevelMapper {
    Trace,
    Debug,
    Info,
    Warn,
    Error
}

impl TracingLevelMapper {

    fn into(&self) -> tracing::Level {
        match self {
            Self::Trace             => tracing::Level::TRACE,
            Self::Debug             => tracing::Level::DEBUG,
            Self::Info              => tracing::Level::INFO,
            Self::Warn              => tracing::Level::WARN,
            Self::Error             => tracing::Level::ERROR
        }
    }

    fn from(tracing: tracing::Level) -> Self {
        match tracing {
            tracing::Level::TRACE   => Self::Trace,
            tracing::Level::DEBUG   => Self::Debug,
            tracing::Level::INFO    => Self::Info,
            tracing::Level::WARN    => Self::Warn,
            tracing::Level::ERROR   => Self::Error,
        }
    }

}

impl Display for TracingLevelMapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::Debug    => f.write_str("Debug"),
            &Self::Trace    => f.write_str("Trace"),
            &Self::Info     => f.write_str("Info"),
            &Self::Warn     => f.write_str("Warn"),
            &Self::Error    => f.write_str("Error"),
        }
    }
}
