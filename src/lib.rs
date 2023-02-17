mod timeline;

pub enum ErrCode {
    Okay,
    ShowHelpSign,
    NoSwitchSpecified,
    UnknownSwitch,
    SwitchHasNoArg,
    NtwkFileParseFailed,
    SimFileParseFailed,
}

#[derive(Debug)]
pub struct RnsSettings {
    pub ntwk_filename: String,
    pub sim_filename: String,
}

impl RnsSettings {
    pub fn from_args(args: std::env::Args) -> Result<RnsSettings, ErrCode> {
        const DEFAULT_NTWK_FILE: &str = "config/basic.ntwk";
        const DEFAULT_SIM_FILE: &str = "config/basic.sim";
        let mut s = RnsSettings {
            ntwk_filename: String::from(DEFAULT_NTWK_FILE),
            sim_filename: String::from(DEFAULT_SIM_FILE),
        };

        let mut cmd_parse_status = ErrCode::Okay;
        let mut ntwk_flag = false;
        let mut sim_flag = false;
        for arg in args.skip(1) {
            match arg.as_str() {
                "-h" => {
                    if ntwk_flag || sim_flag {
                        cmd_parse_status = ErrCode::SwitchHasNoArg;
                    } else {
                        cmd_parse_status = ErrCode::ShowHelpSign;
                    }
                    break;
                }
                "-n" => {
                    if ntwk_flag || sim_flag {
                        cmd_parse_status = ErrCode::SwitchHasNoArg;
                        break;
                    }
                    ntwk_flag = true;
                }
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
                    } else if ntwk_flag && !sim_flag {
                        s.ntwk_filename = arg.to_string();
                        ntwk_flag = false;
                    } else {
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
            _ => Err(cmd_parse_status),
        }
    }
}
