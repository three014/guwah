use std::{rc::Rc, cell::RefCell};
use crate::utils;
use self::{file_utils::{NtwkParseState, NtwkErrCode}, node::NtwkNode};

mod file_utils;
mod node; 

#[derive(Debug)]
pub struct Ntwk {
    node_list: Vec<Rc<RefCell<NtwkNode>>>
    //node_graph: Graph<Node>
}

impl Ntwk {
    pub fn from_file(filename: &String) -> Result<Ntwk, NtwkErrCode> {
        let mut status = NtwkErrCode::Okay;
        let mut state = NtwkParseState::NewNode;

        let mut ntwk = Ntwk {
            node_list: Vec::with_capacity(10)
        };

        let lines = match utils::read_lines(filename) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("{0}", e);
                status = NtwkErrCode::BadFile;
                return Err(status);
            }
        };

        for line in lines {
            if status != NtwkErrCode::Okay {
                break;
            }

            let s = line.unwrap_or("".to_string());
            let s = utils::strip_comment(s);
            if s.is_empty() { continue };

            println!("{}", &s);
            
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
            _ => Err(status)
        }
    }


}
