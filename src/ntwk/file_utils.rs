use std::{cell::RefCell, rc::Rc};
use scanf::sscanf;
use super::{node::NtwkNode, Ntwk};

#[derive(PartialEq)]
pub enum NtwkErrCode {
    Okay,
    HasZeroConnections,
    MismatchEndNodeToken,
    BadFile,
    BadLine,
}

pub enum NtwkParseState {
    NewNode,
    GetConnections,
    EndNode,
    EndNet,
    StopParse,
}

impl NtwkParseState {
    pub fn parse_new_node(ntwk: &mut super::Ntwk, str: &String) -> (NtwkErrCode, NtwkParseState) {
        use NtwkParseState::GetConnections as get_conns; // Because I'm lazy and didn't want to retype 30 chars
        let mut err = NtwkErrCode::Okay;
        
        let mut temp_id: u32 = 0;
        let mut temp_conn_count: u32 = 0;
        let mut temp_queue_delay: u32 = 0;

        let result = sscanf!(str, "{},{},{}", temp_id, temp_conn_count, temp_queue_delay);
        match result {
            Ok(_) => {
                let node = match NtwkNode::new(temp_id, temp_conn_count, temp_queue_delay) {
                    Ok(n) => n,
                    Err(e) => {
                        err = e;
                        return (err, get_conns);
                    },
                };
                let cell = RefCell::new(node);
                let rc = Rc::new(cell);
                ntwk.node_list.push(rc);
            },
            Err(e) => {
                eprintln!("{e}");
                err = NtwkErrCode::BadLine;
            }, 
        }
        (err, get_conns)
    }

    pub fn parse_conn_list(ntwk: &mut super::Ntwk, str: &String) -> (NtwkErrCode, NtwkParseState) {
        let mut next_state = NtwkParseState::GetConnections;
        let mut err = NtwkErrCode::Okay;

        let node_ref = ntwk.node_list.last().unwrap();

        let mut temp_id: u32 = 0;
        let result = sscanf!(str, "{}", temp_id);
        match result {
            Ok(_) => {
                node_ref.borrow_mut().push(temp_id);
                if node_ref.borrow().conn_len() >= node_ref.borrow().conn_count() {
                    next_state = NtwkParseState::EndNode;
                }
            },
            Err(e) => {
                eprintln!("{e}");
                err = NtwkErrCode::BadLine;
            }
        }
        (err, next_state)
    }

    pub fn handle_end_node(str: &String) -> (NtwkErrCode, NtwkParseState) {
        const END_NODE_TOKEN: &str = "endNode";
        if str == END_NODE_TOKEN {
            (NtwkErrCode::Okay, NtwkParseState::EndNet)
        }
        else {
            (NtwkErrCode::MismatchEndNodeToken, NtwkParseState::EndNet)
        }
    }

    pub fn handle_end_net(ntwk: &mut super::Ntwk, str: &String) -> (NtwkErrCode, NtwkParseState) {
        const END_NET_TOKEN: &str = "endNet";
        if str == END_NET_TOKEN {
            (NtwkErrCode::Okay, NtwkParseState::StopParse)
        }
        else {
            Self::parse_new_node(ntwk, str)
        }
    }
}