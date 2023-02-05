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
    EmptyContents,
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
        Some(str_match) => match str_match.as_str() {
            "msg" => match id {
                Some(msg_id_match) => match start_node {
                    Some(start_node_match) => match end_node {
                        Some(end_node_match) => instr = Instr::Msg(MsgInstr::new(time.unwrap(), 
                                                                               msg_id_match, 
                                                                               start_node_match, 
                                                                               end_node_match)),
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