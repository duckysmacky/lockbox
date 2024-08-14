use std::{io, time::Instant};
use lockbox::{cli::{self, commands, logger}, log_fatal, log_success};

fn main() -> io::Result<()> {
    let start_time = Instant::now();
    let args = cli::get_command().get_matches();

    logger::configure_logger(&args);

    /* BOX */
    if let Some(args) = args.subcommand_matches("box") {
        match cli::commands::r#box(args) {
            Ok(file_count) => log_success!("Total files encrypted: {}", file_count),
            Err(err) => log_fatal!("Encryption failed: {}", err)
        }
    }

    /* UNBOX */
    if let Some(args) = args.subcommand_matches("unbox") {
        match commands::unbox(args) {
            Ok(file_count) => log_success!("Total files decrypted: {}", file_count),
            Err(err) => log_fatal!("Decryption failed: {}", err)
        }
    }

    /* KEY */
    if let Some(args) = args.subcommand_matches("key") {
        commands::key(args)
    }

    let duration = start_time.elapsed();
    log_success!("Time taken: {:.2?}", duration);
    Ok(())
}