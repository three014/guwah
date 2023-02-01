use guwah::{ErrCode, Settings};
use crate::ntwk::Ntwk;

mod ntwk;
mod utils;

fn main() {
    let exit_code = run();
    std::process::exit(exit_code);
}

fn run() -> i32 {
    // Get cmd args -> global settings
    let args = std::env::args();
    let rns_settings = match Settings::from_args(args) {
        Ok(settings) => settings,
        Err(ErrCode::ShowHelpSign) => {
            println!("help board");
            return ErrCode::Okay as i32;
        },
        Err(err) => {
            eprintln!("parse error");
            return err as i32;
        },
    };

    //dbg!(&rns_settings);

    // Open ntwk file and parse according to settings
    let ntwk = match Ntwk::from_file(rns_settings.ntwk_file()) {
        Ok(n) => n,
        Err(err) => {
            eprintln!("ntwk parse error");
            return err as i32;
        }
    };

    dbg!(&ntwk);

    // Open sim file and parse according to settings
    ErrCode::Okay as i32
}