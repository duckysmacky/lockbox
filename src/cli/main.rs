//! CLI entry point

use std::{io, time::Instant};
use databoxer::{cli, log_info, log_success};
use databoxer::cli::{command, handlers};

fn main() -> io::Result<()> {
    let start_time = Instant::now();
    let global_args = &command::get_command().get_matches();

    cli::logger::configure_logger(&global_args);

    /* BOX */
    if let Some(args) = global_args.subcommand_matches("box") {
        let (total, error) = handlers::handle_box(args);

        log_success!("[{}/{}] files encrypted", total - error, total);
        if total == error {
            std::process::exit(1);
        }
    }

    /* UNBOX */
    if let Some(args) = global_args.subcommand_matches("unbox") {
        let (total, error) = handlers::handle_unbox(args);

        log_success!("[{}/{}] files decrypted", total - error, total);
        if total == error {
            std::process::exit(1);
        }
    }

    /* PROFILE */
    if let Some(args) = global_args.subcommand_matches("profile") {
        /* PROFILE CREATE */
        if let Some(args) = args.subcommand_matches("new") {
            handlers::handle_profile_create(args);
        }
        /* PROFILE DELETE */
        if let Some(args) = args.subcommand_matches("delete") {
            handlers::handle_profile_delete(args);
        }
        /* PROFILE SET */
        if let Some(args) = args.subcommand_matches("set") {
            handlers::handle_profile_set(args);
        }
        /* PROFILE GET */
        if let Some(args) = args.subcommand_matches("get") {
            handlers::handle_profile_get(args);
        }
        /* PROFILE LIST */
        if let Some(args) = args.subcommand_matches("list") {
            handlers::handle_profile_list(args);
        }
    }

    /* KEY */
    if let Some(args) = global_args.subcommand_matches("key") {
        /* KEY NEW */
        if let Some(args) = args.subcommand_matches("new") {
            handlers::handle_key_new(args);
        }
        /* KEY GET */
        if let Some(args) = args.subcommand_matches("get") {
            handlers::handle_key_get(args);
        }
		/* KEY SET */
		if let Some(args) = args.subcommand_matches("set") {
            handlers::handle_key_set(args);
        }
    }

    let duration = start_time.elapsed();
    log_info!("Time taken: {:.2?}", duration);
    Ok(())
}