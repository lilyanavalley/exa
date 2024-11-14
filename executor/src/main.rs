//! Executor with your game connected to it as a plugin.
// use clap::Parser;
use fyrox::engine::executor::Executor;
use game:: { Game } ;
// use firedbg_lib::fire;
use tracing:: { instrument, debug, debug_span, info, info_span, warn, warn_span, error, error_span };
use { tracing, tracing_subscriber, tracing_tracy }; // Application tracing & profiling.
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;


/// Application command-line features.
mod cmd;


/// Number of *Update Ticks*, per second.
const UPDATE_RATE:      f32     = 30.0;


#[instrument]
fn main() {

    // // Read command line arguments and env.
    // let cmds = cmd::Cli::parse();

    let _ = tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
        .with(tracing_tracy::TracyLayer::default())
        .with(tracing_subscriber::fmt::layer())
    ).map_err(|_err| eprintln!("Unable to set global default subscriber"));

    let mut executor = Executor::new();
    let mut game = Game::default();
    
    // Configure game to passed commands/arguments.
    // executor.set_desired_update_rate(cmds.rate());
    // executor.set_headless(cmds.headless());
    // game.developer_mode(cmds.developer());
    
    executor.set_desired_update_rate(UPDATE_RATE);
    let _ = executor.set_frame_size((960, 540))
        .inspect_err(|error| error!("Executor frame size not modifiable: {:?}", error));

    executor.add_plugin(game);
    executor.run()

}
