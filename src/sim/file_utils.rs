use super::instr::{Instr, MsgInstr, RepInstr, EndInstr};

#[derive(PartialEq)]
pub enum SimErrCode {
    Okay,
    MismatchEndSimToken,
    UnknownInstrKind,
    NoId,
    NoTimestamp,
    NoStartNode,
    NoEndNode,
    BadFile,
    BadLine,
}

pub fn parse_instr(str: &String) -> Result<Instr, SimErrCode> {
    let mut time: Option<u32> = None;
    let mut kind: Option<String> = None;
    let mut id: Option<u32> = None;
    let mut start_node: Option<u32> = None;
    let mut end_node: Option<u32> = None;
    let tokens = str.split(',');

    for (idx, tok) in tokens.enumerate() {
        match idx {
            0 => time = parse_tok(tok),
            1 => kind = parse_tok(tok),
            2 => id = parse_tok(tok),
            3 => start_node = parse_tok(tok),
            4 => end_node = parse_tok(tok),
            _ => break
        }
    }

    let instr: Instr;

    if time.is_none() { return Err(SimErrCode::NoTimestamp) };

    match kind {
        Some(s) => match s.as_str() {
            "msg" => match id {
                Some(i) => match start_node {
                    Some(sn) => match end_node {
                        Some(en) => instr = Instr::Msg(MsgInstr::new(time.unwrap(), 
                                                                               i, 
                                                                               sn, 
                                                                               en)),
                        None => return Err(SimErrCode::NoEndNode)
                    },
                    None => return Err(SimErrCode::NoStartNode)
                },
                None => return Err(SimErrCode::NoId)
            },
            "rep" => instr = Instr::Rep(RepInstr::new(time.unwrap(), id)),
            "endSim" => instr = Instr::EndSim(EndInstr::new(time.unwrap())),
            _ => return Err(SimErrCode::UnknownInstrKind),
        },
        None => return Err(SimErrCode::BadLine)
    };

    Ok(instr)
}


fn parse_tok<T: std::str::FromStr>(tok: &str) -> Option<T> {
    match tok.parse::<T>() {
        Ok(t) => Some(t),
        Err(_) => None,
    }
}