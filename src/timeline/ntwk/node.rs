use super::file_utils::NtwkErrCode;

const MAX_CONN_COUNT: usize = 20;

#[derive(Debug)]
pub struct NtwkNode {
    id: u32,
    conn_count: usize,
    conn_list: Vec<u32>,
    queue_delay: u32,
}

impl NtwkNode {
    pub fn new(id: u32, conn_count: u32, queue_delay: u32) -> Result<NtwkNode, NtwkErrCode> {
        let mut create_node_status = NtwkErrCode::Okay;

        let cc = conn_count.try_into().unwrap_or(MAX_CONN_COUNT);
        let n = NtwkNode {
            id,
            conn_count: cc,
            conn_list: Vec::with_capacity(cc),
            queue_delay,
        };
        if conn_count == 0 {
            create_node_status = NtwkErrCode::HasZeroConnections;
        }

        match create_node_status {
            NtwkErrCode::Okay => Ok(n),
            _ => Err(create_node_status),
        }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn conn_count(&self) -> usize {
        self.conn_count
    }
    pub fn conn_len(&self) -> usize {
        self.conn_list.len()
    }
    pub fn push(&mut self, conn: u32) {
        self.conn_list.push(conn);
    }
}
