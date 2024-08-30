use std::{io, time::Instant};
use lockbox::{cli, log_success};

fn main() -> io::Result<()> {
    let start_time = Instant::now();
    let global_args = &cli::get_command().get_matches();

    cli::logger::configure_logger(&global_args);

    /* BOX */
    if let Some(args) = global_args.subcommand_matches("box") {
        let (total, error) = cli::commands::r#box(global_args, args);

        log_success!("[{}/{}] files encrypted", total - error, total);
        if total == error {
            std::process::exit(1);
        }
    }

    /* UNBOX */
    if let Some(args) = global_args.subcommand_matches("unbox") {
        let (total, error) = cli::commands::unbox(global_args, args);

        log_success!("[{}/{}] files decrypted", total - error, total);
        if total == error {
            std::process::exit(1);
        }
    }

    /* PROFILE */
    if let Some(args) = global_args.subcommand_matches("profile") {
        /* PROFILE CREATE */
        if let Some(args) = args.subcommand_matches("new") {
            cli::commands::profile_create(global_args, args);
        }
        /* PROFILE DELETE */
        if let Some(args) = args.subcommand_matches("delete") {
            cli::commands::profile_delete(global_args, args);
        }
        /* PROFILE SET */
        if let Some(args) = args.subcommand_matches("set") {
            cli::commands::profile_set(global_args, args);
        }
        /* PROFILE GET */
        if let Some(args) = args.subcommand_matches("get") {
            cli::commands::profile_get(global_args, args);
        }
        /* PROFILE LIST */
        if let Some(args) = args.subcommand_matches("list") {
            cli::commands::profile_list(global_args, args);
        }
    }

    /* KEY */
    if let Some(args) = global_args.subcommand_matches("key") {
        /* KEY NEW */
        if let Some(args) = args.subcommand_matches("new") {
            cli::commands::key_new(global_args, args);
        }
        /* KEY GET */
        if let Some(args) = args.subcommand_matches("get") {
            cli::commands::key_get(global_args, args);
        }
		/* KEY SET */
		if let Some(args) = args.subcommand_matches("set") {
            cli::commands::key_set(global_args, args);
        }
    }

    let duration = start_time.elapsed();
    log_success!("Time taken: {:.2?}", duration);
    Ok(())
}