use self::{
    file_utils::{NtwkErrCode, NtwkParseState},
    node::NtwkNode,
};
use crate::timeline::utils;
use std::{cell::RefCell, rc::Rc};

mod file_utils;
mod node;

const DEFAULT_NUM_NODES: usize = 20;

#[derive(Debug)]
pub struct Ntwk {
    node_list: Vec<Rc<RefCell<NtwkNode>>>, //node_graph: Graph<Node>
}

impl Ntwk {
    pub fn from_file(filename: &String) -> Result<Ntwk, NtwkErrCode> {
        let mut status = NtwkErrCode::Okay;
        let mut state = NtwkParseState::NewNode;

        let mut ntwk = Ntwk {
            node_list: Vec::with_capacity(DEFAULT_NUM_NODES),
        };

        let lines = match utils::internal_utils::read_lines(filename) {
            Ok(lines) => lines,
            Err(e) => {
                eprintln!("{e}");
                status = NtwkErrCode::BadFile;
                return Err(status);
            }
        };

        for line in lines {
            if status != NtwkErrCode::Okay {
                break;
            }

            let s = line.unwrap_or("".to_string());
            let s = utils::internal_utils::strip_comment(s);
            if s.is_empty() {
                continue;
            };

            //println!("{}", &s);

            (status, state) = match state {
                NtwkParseState::NewNode => NtwkParseState::parse_new_node(&mut ntwk, &s),
                NtwkParseState::GetConnections => NtwkParseState::parse_conn_list(&mut ntwk, &s),
                NtwkParseState::EndNode => NtwkParseState::handle_end_node(&s),
                NtwkParseState::EndNet => NtwkParseState::handle_end_net(&mut ntwk, &s),
                NtwkParseState::StopParse => break,
            }
        }

        match status {
            NtwkErrCode::Okay => Ok(ntwk),
            _ => Err(status),
        }
    }
}
