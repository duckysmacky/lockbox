//! CLI entry point

use std::{io, time::Instant};
use databoxer::app::AppMode;
use databoxer::cli::{logger, command, handlers};

fn main() -> io::Result<()> {
    databoxer::app::set_app_mode(AppMode::CLI);
    
    let start_time = Instant::now();
    let global_args = &command::get_command().get_matches();

    logger::configure_logger(&global_args);

    /* BOX */
    if let Some(args) = global_args.subcommand_matches("box") {
        let (total, error) = handlers::handle_box(args);

        println!("[{}/{}] files encrypted", total - error, total);
        if total == error {
            std::process::exit(1);
        }
    }

    /* UNBOX */
    if let Some(args) = global_args.subcommand_matches("unbox") {
        let (total, error) = handlers::handle_unbox(args);

        println!("[{}/{}] files decrypted", total - error, total);
        if total == error {
            std::process::exit(1);
        }
    }

    /* INFORMATION */
    if let Some(args) = global_args.subcommand_matches("information") {
        handlers::handle_information(args);
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
    println!("Time taken: {:.2?}", duration);
    Ok(())
}