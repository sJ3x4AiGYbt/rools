mod args;
mod commands;
mod global;

use clap::Parser;

fn main() {
  let parsed = args::Args::parse();

  match parsed.command {
    args::Commands::SysViz(sys_args) => {
      commands::sysviz::run(sys_args);
    }

    args::Commands::WebScout(web_args) => {
      commands::webscout::run(web_args);
    }
  }
}
