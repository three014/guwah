use crate::ntwk::Ntwk;

mod ntwk;
pub mod utils;

fn main() {
    let exit_code = start();
    std::process::exit(exit_code);
}

fn start() -> i32 {
    // Get cmd args -> global settings
    let args: Vec<String> = std::env::args().collect();
    let rns_settings = match guwah::Settings::from_args(args) {
        Ok(settings) => settings,
        Err(guwah::ErrCode::ShowHelpSign) => {
            println!("help board");
            return guwah::ErrCode::Okay as i32;
        },
        Err(err) => {
            println!("parse error");
            return err as i32;
        },
    };

    dbg!(&rns_settings);

    // Open ntwk file and parse according to settings
    let _ntwk = match Ntwk::from_file(rns_settings.ntwk_file()) {
        Ok(n) => n,
        Err(err) => {
            println!("ntwk parse error");
            return err as i32;
        }
    };

    // Open sim file and parse according to settings
    guwah::ErrCode::Okay as i32
}