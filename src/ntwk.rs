use std::io::Lines;

use crate::utils;

pub enum NtwkErrCode {
    Okay,
    HasZeroConnections,
    BadFile,
}

pub struct Ntwk {
    node_list: Vec<NtwkNode>,
    //node_graph: Graph<Node>
}

impl Ntwk {
    pub fn from_file(filename: &String) -> Result<Ntwk, NtwkErrCode> {
        let mut file_parse_status = NtwkErrCode::Okay;

        let ntwk = Ntwk {
            node_list: Vec::new()
        };

        let lines = match utils::read_lines(filename.to_string()) {
            Ok(l) => l,
            Err(e) => {
                println!("{0}", e);
                file_parse_status = NtwkErrCode::BadFile;
                return Err(file_parse_status);
            }
        };
        for line in lines {
            let s = match line {
                Ok(l) => l,
                Err(_) => "".to_owned()
            };

        }
        

        match file_parse_status {
            NtwkErrCode::Okay => Ok(ntwk),
            _ => Err(file_parse_status)
        }
    }


}

enum NtwkParseState {
    
}
struct NtwkNode {
    id: u32,
    conn_list: Vec<u32>
}

impl NtwkNode {
    fn new(id: u32, conn_count: u32) -> Result<NtwkNode, NtwkErrCode> {
        let mut create_node_status = NtwkErrCode::Okay;

        let n = NtwkNode {
            id,
            conn_list: Vec::with_capacity(conn_count.try_into().unwrap_or_default())
        };
        if conn_count == 0 {
            create_node_status = NtwkErrCode::HasZeroConnections;
        }
        
        match create_node_status {
            NtwkErrCode::Okay => Ok(n),
            _ => Err(create_node_status)
        }
    }
}

