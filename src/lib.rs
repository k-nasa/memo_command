pub mod commands;
pub mod config;
pub mod dir_tree;
pub mod file_or_dir;
pub mod utils;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

use crate::commands::{
    build_app, cmd_config::cmd_config, cmd_delete::cmd_delete, cmd_edit::cmd_edit,
    cmd_grep::cmd_grep, cmd_list::cmd_list, cmd_new::cmd_new, cmd_quick::cmd_quick,
};

use crate::config::Config;
use std::fs::create_dir_all;

pub fn run() {
    let mut app = build_app();
    let config = match Config::load_config() {
        Ok(config) => config,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let memo_dir = config.memos_dir();
    create_dir_all(memo_dir).expect("faild create memos_dir");

    match app.clone().get_matches().subcommand() {
        ("config", Some(_)) => cmd_config(&config),

        ("delete", Some(matches)) => cmd_delete(matches, &config),

        ("edit", Some(matches)) => cmd_edit(matches, &config),

        ("grep", Some(matches)) => cmd_grep(matches, &config),

        ("list", Some(matches)) => cmd_list(matches, &config),

        ("new", Some(matches)) => cmd_new(matches, &config),

        ("quick", Some(matches)) => cmd_quick(matches, &config),

        _ => app.print_help().expect("faild print help"),
    };
}
