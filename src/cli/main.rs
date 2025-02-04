//! CLI entry point

use std::{io, time::Instant};
use databoxer::{cli, log_success, log_info};

fn main() -> io::Result<()> {
    let start_time = Instant::now();
    let global_args = &cli::get_command().get_matches();

    cli::logger::configure_logger(&global_args);

    /* BOX */
    if let Some(args) = global_args.subcommand_matches("box") {
        let (total, error) = cli::commands::handle_box(global_args, args);

        log_success!("[{}/{}] files encrypted", total - error, total);
        if total == error {
            std::process::exit(1);
        }
    }

    /* UNBOX */
    if let Some(args) = global_args.subcommand_matches("unbox") {
        let (total, error) = cli::commands::handle_unbox(global_args, args);

        log_success!("[{}/{}] files decrypted", total - error, total);
        if total == error {
            std::process::exit(1);
        }
    }

    /* PROFILE */
    if let Some(args) = global_args.subcommand_matches("profile") {
        /* PROFILE CREATE */
        if let Some(args) = args.subcommand_matches("new") {
            cli::commands::handle_profile_create(global_args, args);
        }
        /* PROFILE DELETE */
        if let Some(args) = args.subcommand_matches("delete") {
            cli::commands::handle_profile_delete(global_args, args);
        }
        /* PROFILE SET */
        if let Some(args) = args.subcommand_matches("set") {
            cli::commands::handle_profile_set(global_args, args);
        }
        /* PROFILE GET */
        if let Some(args) = args.subcommand_matches("get") {
            cli::commands::handle_profile_get(global_args, args);
        }
        /* PROFILE LIST */
        if let Some(args) = args.subcommand_matches("list") {
            cli::commands::handle_profile_list(global_args, args);
        }
    }

    /* KEY */
    if let Some(args) = global_args.subcommand_matches("key") {
        /* KEY NEW */
        if let Some(args) = args.subcommand_matches("new") {
            cli::commands::handle_key_new(global_args, args);
        }
        /* KEY GET */
        if let Some(args) = args.subcommand_matches("get") {
            cli::commands::handle_key_get(global_args, args);
        }
		/* KEY SET */
		if let Some(args) = args.subcommand_matches("set") {
            cli::commands::handle_key_set(global_args, args);
        }
    }

    let duration = start_time.elapsed();
    log_info!("Time taken: {:.2?}", duration);
    Ok(())
}