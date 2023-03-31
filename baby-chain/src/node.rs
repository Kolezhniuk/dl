use crate::{block::Block, blockchain::Blockchain};

#[derive(Debug, Clone, Copy)]

pub struct NodeSettings {
    pub number_of_connections: u32,
    pub max_cpu_usage: u32,
    pub max_memory_usage: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum NodeStatus {
    Online,
    Offline,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct NetworkConnection<'a> {
    pub node_id: Vec<u8>,
    pub status: &'a NodeStatus,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Node<'a> {
    pub node_id: Vec<u8>,
    pub blockchain: Blockchain,
    pub ip: String,
    pub connections: Vec<NetworkConnection<'a>>,
    pub settings: NodeSettings,
    pub status: NodeStatus,
}

impl<'a> Node<'a> {
    pub fn new(node_id: Vec<u8>, settings: NodeSettings, ip: String) -> Node<'a> {
        let blockchain = Blockchain::init(100);
        let connections = Vec::new();
        Node {
            node_id,
            blockchain,
            connections,
            ip,
            settings,
            status: NodeStatus::Online,
        }
    }

    pub fn add_connection(&mut self, connection: NetworkConnection<'a>) {
        self.connections.push(connection);
    }

    pub fn add_block(&mut self, block: Block) {
        self.blockchain.add_block(block);
    }

    pub fn start_networking(&mut self) {
        print!("Starting networking module");
    }

    //a function that allows you to update the settings of the current node.
    pub fn update_settings(&mut self, settings: NodeSettings) {
        self.settings = settings;
    }

    pub fn stop_node(&mut self) {
        print!("Stopping node");
    }

    pub fn get_status(&self) -> NodeStatus {
        self.status
    }

    pub fn rescan_blockchain(&self) {
        print!("Rescanning blockchain");
    }
}
