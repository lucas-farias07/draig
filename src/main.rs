use draig::create;
use std::{env, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args[0] == "create" {
        create(args[1].to_owned())?;
    }
    Ok(())
}
