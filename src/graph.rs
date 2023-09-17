use core::fmt;
use std::collections::HashMap;
use crate::file_io;

pub struct Graph {
    //m_node_list: Vec<Node<'a>>,
    m_name: String,
    m_nodes: HashMap<String, Node>,
    m_edges: HashMap<String, Edge>,
}
impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Graph name: {}\n", self.m_name)?;
        write!(f, "Number of nodes: {}\n", self.m_nodes.len())?;
        for (_key, node) in self.m_nodes.iter() {
            write!(f, "{}", node)?;
        }
        write!(f, "Number of Edges: {}\n", self.m_edges.len())?;
        for (_key, edge) in self.m_edges.iter() {
            write!(f, "{}", edge)?;
        }
        write!(f, "")
    }
}
impl Graph {
    pub fn new(name: String) -> Graph {
        return Graph {
            m_name: name,
            m_nodes: HashMap::new(),
            m_edges: HashMap::new(),
        };
    }
    pub fn build_graph(&mut self) {
        // Create nodes
        //let mut nodes: HashMap<u16, Node> = HashMap::new();
        //let mut edges: HashMap<u16, Edge> = HashMap::new();
        let node_id1: String = "1".to_string();
        let node_id2: String = "2".to_string();
        let node_id3: String = "3".to_string();
        //let mut all_node_ids =  vec![node_id1,node_id2,node_id3];
        let mut node1: Node = Node::new(node_id1);
        let mut node2: Node = Node::new(node_id2);
        let mut node3: Node = Node::new(node_id3);
        self.m_nodes.insert(node1.get_id_copy(), node1);
        self.m_nodes.insert(node2.get_id_copy(), node2);
        self.m_nodes.insert(node3.get_id_copy(), node3);
        let edge_id1: String = "1-2".to_string();
        let edge_id2: String = "1-3".to_string();
        let edge_id3: String = "2-3".to_string();
        //let mut all_edge_ids =  vec![edge_id1,edge_id2,edge_id3];
        let mut edge12: Edge = Edge::new(edge_id1, "1".to_string(), "2".to_string());
        let mut edge13: Edge = Edge::new(edge_id2, "1".to_string(), "3".to_string());
        let mut edge23: Edge = Edge::new(edge_id3, "2".to_string(), "3".to_string());
        self.m_edges.insert(edge12.get_id_copy(), edge12);
        self.m_edges.insert(edge13.get_id_copy(), edge13);
        self.m_edges.insert(edge23.get_id_copy(), edge23);
        for (node_id, node) in self.m_nodes.iter_mut() {
            for (edge_id, edge) in self.m_edges.iter() {
                if edge.m_node_id_end == *node_id {
                    node.add_edge(edge);
                } else if edge.m_node_id_start == *node_id {
                    node.add_edge(edge);
                }
            }
        }
    }
    pub fn build_graph_from_file(&mut self, filename: String) {
        let file_content_string = file_io::read_txt_file(filename);
        let root: minidom::Element;
        print!("{}",file_content_string);
        match file_content_string.parse() {
            Ok(x) => root = x,
            Err(e) => panic!("XML parsing error {}",e),
        }
    
        println!("{:#?}", root);
    }
    /*
    fn build_node_edge_list(&mut self ,node: &mut Node){
        for (key,value) in self.m_edges{
            if value.m_node_idx_end == node.idx() {
                node.add_edge(&value);
            }
            else if  value.m_node_idx_start == node.idx(){
                node.add_edge(&value);
            }
        }
    }
     */
    /*
    pub fn add_node(&mut self,node: Node<'a>) {
        self.m_node_list.push(node);
    }
    pub fn add_edge(&mut self, edge: Edge) {
        self.m_edge_list.push(edge);
    }
    pub fn sort_node_list(&mut self) {
        self.m_node_list.sort_by(|a,b|a.m_idx.cmp(&b.m_idx));
    }
     */
}
pub struct Node {
    m_id: String,
    m_incident_edges_ids: Vec<String>,
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node idx: {}, Edges: [", self.m_id)?;
        for edge_idx in self.m_incident_edges_ids.iter() {
            write!(f, " {} ", edge_idx)?;
        }
        write!(f, "]\n")
    }
}
impl Node {
    pub fn new(idx: String) -> Node {
        //let mut edge_list: Vec<&Edge> = vec![];
        return Node {
            m_id: idx,
            m_incident_edges_ids: vec![],
        };
    }
    pub fn add_edge(&mut self, edge: &Edge) {
        self.m_incident_edges_ids.push(edge.get_id_copy());
    }
    pub fn get_id_copy(&self) -> String {
        return self.m_id.clone();
    }
}
pub struct Edge {
    m_id: String,
    m_node_id_start: String,
    m_node_id_end: String,
}
impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Edge idx: {}, Idx Node Start: {}, Idx Node End {}\n",
            self.m_id, self.m_node_id_start, self.m_node_id_end
        )
    }
}
impl Edge {
    //pub fn new(idx: u16, node_start: &'a Node, node_end: &'a Node) -> Edge<'a>{
    //    return Edge{m_idx: idx, m_node_start: node_start, m_node_end: node_end};
    //}
    pub fn new(id: String, node_id_start: String, node_id_end: String) -> Edge {
        return Edge {
            m_id: id,
            m_node_id_start: node_id_start,
            m_node_id_end: node_id_end,
        };
    }
    pub fn get_id_copy(&self) -> String {
        return self.m_id.clone();
    }
}