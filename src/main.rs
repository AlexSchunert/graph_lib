
mod graph {

    pub struct Graph<'a>{
        m_node_list: Vec<&'a Node<'a>>,
        m_edge_list: Vec<&'a Edge>,
    }

    impl <'a> Graph<'a>{

        pub fn new() -> Graph <'a>{
            return Graph{m_node_list: vec![], m_edge_list: vec![]};	
        }

        pub fn add_node(&mut self,node: &'a Node) {
            self.m_node_list.push(node);

        }

        pub fn add_edge(&mut self, edge: &'a Edge) {
            self.m_edge_list.push(edge);
        }

        pub fn sort_node_list(&mut self) {
            self.m_node_list.sort_by(|a,b|a.m_idx.cmp(&b.m_idx));

        }

    }

    pub struct Node<'a> {
        m_idx: u16,
        m_edge_list: Vec<&'a Edge>,
    }

    impl <'a> Node <'a> {
        pub fn new(idx: u16) -> Node<'a> {
            //let mut edge_list: Vec<&Edge> = vec![];
            return Node {m_idx: idx, m_edge_list: vec![]};
        }
        
        pub fn add_edge(&mut self, edge:&'a Edge) {
            self.m_edge_list.push(edge);
        }

        pub fn idx(&self) -> u16 {
            return self.m_idx;
        }
    }

    pub struct Edge{
        m_idx: u16,
        m_node_idx_start: u16,
        m_node_idx_end: u16,
    }

    impl Edge  {
        //pub fn new(idx: u16, node_start: &'a Node, node_end: &'a Node) -> Edge<'a>{    
        //    return Edge{m_idx: idx, m_node_start: node_start, m_node_end: node_end};
        //}
        pub fn new(idx: u16, node_idx_start: u16, node_idx_end: u16) -> Edge{

            return Edge{m_idx: idx, m_node_idx_start: node_idx_start, m_node_idx_end:node_idx_end};
        }

    

    }


}


fn main() {

    // Make list of nodes 
    let mut node1: graph::Node = graph::Node::new(0);
    let mut node2: graph::Node = graph::Node::new(1);
    let mut node3: graph::Node = graph::Node::new(2);
    let mut edge12: graph::Edge = graph::Edge::new(0,node1.idx(),node2.idx());
    let mut edge13: graph::Edge = graph::Edge::new(1,node1.idx(),node3.idx());
    let mut edge23: graph::Edge = graph::Edge::new(2,node2.idx(),node3.idx());

    //let mut edge12: graph::Edge = graph::Edge::new(0);
    // Define nodes 
    node1.add_edge(&edge12);
    node2.add_edge(&edge12);
    node1.add_edge(&edge13);
    node3.add_edge(&edge13);
    node2.add_edge(&edge23);
    node3.add_edge(&edge23);

    
    let mut example_graph =  graph::Graph::new();
    example_graph.add_node(&node1);
    example_graph.add_node(&node3);
    example_graph.add_node(&node2);
    example_graph.add_edge(&edge12);
    example_graph.add_edge(&edge13);
    example_graph.add_edge(&edge23);
    example_graph.sort_node_list();
    println!("Hello, world!");
}
