pub enum ErrCode {
    Okay,
    ShowHelpSign,
    NoSwitchSpecified,
    UnknownSwitch,
    SwitchHasNoArg,
    NtwkFileParseFailed,
    SimFileParseFailed
}

#[derive(Debug)] // TODO: Remove derive
pub struct Settings {
    ntwk_filename: String,
    sim_filename: String
}

impl Settings {
    pub fn from_args(args: std::env::Args) -> Result<Settings, ErrCode> {
        const DEFAULT_NTWK_FILE: &str = "config/basic.ntwk";
        const DEFAULT_SIM_FILE: &str = "config/basic.sim";
        let mut s = Settings {
            ntwk_filename: String::from(DEFAULT_NTWK_FILE),
            sim_filename: String::from(DEFAULT_SIM_FILE)
        };

        let mut cmd_parse_status = ErrCode::Okay;
        let mut ntwk_flag = false;
        let mut sim_flag = false;
        for arg in args.skip(1) {
            match arg.as_str() {
                "-h" => {
                    if ntwk_flag || sim_flag {
                        cmd_parse_status = ErrCode::SwitchHasNoArg;
                    }
                    else {
                        cmd_parse_status = ErrCode::ShowHelpSign;
                    }
                    break;
                },
                "-n" => {
                    if ntwk_flag || sim_flag {
                        cmd_parse_status = ErrCode::SwitchHasNoArg;
                        break;
                    }
                    ntwk_flag = true;
                },
                "-s" => {
                    if sim_flag || ntwk_flag {
                        cmd_parse_status = ErrCode::SwitchHasNoArg;
                        break;
                    }
                    sim_flag = true;
                }
                _ => {
                    if arg.as_str().starts_with('-') && (arg.len() <= 2) {
                        cmd_parse_status = ErrCode::UnknownSwitch;
                        break;
                    }
                    if sim_flag && !ntwk_flag {
                        s.sim_filename = arg.to_string();
                        sim_flag = false;
                    }
                    else if ntwk_flag && !sim_flag {
                        s.ntwk_filename = arg.to_string();
                        ntwk_flag = false;
                    }
                    else {
                        cmd_parse_status = ErrCode::NoSwitchSpecified;
                        break;
                    }
                }
            };
        }

        if sim_flag || ntwk_flag {
            cmd_parse_status = ErrCode::SwitchHasNoArg;
        }

        match cmd_parse_status {
            ErrCode::Okay => Ok(s),
            _ => Err(cmd_parse_status)
        }
    }

    pub fn ntwk_file(&self) -> &String {
        &self.ntwk_filename
    }

    pub fn sim_file(&self) -> &String {
        &self.sim_filename
    }
}