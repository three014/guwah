use crate::utils;

use self::{instr::Instr, file_utils::SimErrCode};

mod file_utils;
mod instr;

const DEFAULT_NUM_INSTRS: usize = 10;

#[derive(Debug)]
pub struct Sim {
    instr_set: Vec<Instr>,
}

impl Sim {
    pub fn from_file(filename: &String) -> Result<Sim, SimErrCode> {
        let mut status = SimErrCode::Okay;

        let lines = match utils::read_lines(filename) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("{e}");
                status = SimErrCode::BadFile;
                return Err(status);
            }
        };

        let mut sims = Sim {
            instr_set: Vec::with_capacity(DEFAULT_NUM_INSTRS)
        };

        for line in lines {
            if status != SimErrCode::Okay {
                break;
            }

            let s = line.unwrap_or("".to_string());
            let s = utils::strip_comment(s);
            if s.is_empty() { continue };

            // parse line and turn into sim instruction, then add to sim list
            println!("{}", &s);

            let instr = match file_utils::parse_instr(&s) {
                Ok(i) => i,
                Err(_) => continue
            };

            sims.instr_set.push(instr);
            
        }

        match status {
            SimErrCode::Okay => Ok(sims),
            _ => Err(status)
        }
    }
}