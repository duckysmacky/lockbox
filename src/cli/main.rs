use std::{io, time::Instant};
use lockbox::cli::{self, commands, logger};
use lockbox::log_success;

fn main() -> io::Result<()> {
    let start_time = Instant::now();
    let args = cli::get_command().get_matches();

    logger::configure_logger(&args);

    /* BOX */
    if let Some(args) = args.subcommand_matches("box") {
        let (total, error) = commands::r#box(args);

        log_success!("[{}/{}] files successfully encrypted", total - error, total);
    }

    /* UNBOX */
    if let Some(args) = args.subcommand_matches("unbox") {
        let (total, error) = commands::unbox(args);

        log_success!("[{}/{}] files successfully decrypted", total - error, total);
    }

    /* KEY */
    if let Some(args) = args.subcommand_matches("key") {
        commands::key(args)
    }

    let duration = start_time.elapsed();
    log_success!("Time taken: {:.2?}", duration);
    Ok(())
}