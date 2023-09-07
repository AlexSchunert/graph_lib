use std::collections::HashMap;


mod graph {
    use std::collections::HashMap;


    pub struct Graph<'a>{
        //m_node_list: Vec<Node<'a>>,
        m_node_list: HashMap<u16, Node<'a>>,
        m_edge_list: HashMap<u16, Edge>,
    }

    impl <'a>Graph<'a>{

        pub fn new() -> Graph<'a> {
            return Graph{m_node_list: HashMap::new(), m_edge_list:HashMap::new()};	
        }

        pub fn build_graph(&'a mut self) {
        // Create nodes
        
        let node_idx1: u16 = 1; 
        let node_idx2: u16 = 2;
        let node_idx3: u16 = 3;
        let mut all_node_idxs =  vec![node_idx1,node_idx2,node_idx3];
        let mut node1: Node = Node::new(node_idx1);
        let mut node2: Node = Node::new(node_idx2);
        let mut node3: Node = Node::new(node_idx3);

        self.m_node_list.insert(node1.idx(), node1);
        self.m_node_list.insert(node2.idx(), node2);
        self.m_node_list.insert(node3.idx(), node3);


        let edge_idx1: u16 = 1; 
        let edge_idx2: u16 = 2;
        let edge_idx3: u16 = 3;
        let mut all_edge_idxs =  vec![edge_idx1,edge_idx2,edge_idx3];
        let mut edge12: Edge = Edge::new(0,node_idx1,node_idx2);
        let mut edge13: Edge = Edge::new(1,node_idx1,node_idx3);
        let mut edge23: Edge = Edge::new(2,node_idx2,node_idx3);

        self.m_edge_list.insert(edge12.idx(),edge12);
        self.m_edge_list.insert(edge13.idx(),edge13);
        self.m_edge_list.insert(edge23.idx(),edge23);

        //let mut edge12: graph::Edge = graph::Edge::new(0);
        // Define nodes 
        for idx in all_node_idxs.iter() {
            match self.m_node_list.get_mut(&node_idx1) {
                Some(x) => self.build_node_edge_list(x),
                None => panic!("Idx {} unknown",node_idx1)
            }

        }
    
        //self.m_node_list[1].add_edge(&self.m_edge_list[0]);
        //self.m_node_list[0].add_edge(&self.m_edge_list[1]);
        //self.m_node_list[2].add_edge(&self.m_edge_list[1]);
        //self.m_node_list[1].add_edge(&self.m_edge_list[2]);
        //self.m_node_list[2].add_edge(&self.m_edge_list[2]);

        }


        fn build_node_edge_list(&self ,node: &mut Node){
            for (key,value) in self.m_edge_list{
                if value.m_node_idx_end == node.idx() {
                    node.add_edge(&value);
                }
                else if  value.m_node_idx_start == node.idx(){
                    node.add_edge(&value);
                }        

            }

        }

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

    pub struct Node<'a> {
        m_idx: u16,
        m_edge_list: Vec<&'a Edge>,
    }

    impl <'a> Node <'a>{
        pub fn new(idx: u16) -> Node<'a>{
            //let mut edge_list: Vec<&Edge> = vec![];
            return Node {m_idx: idx, m_edge_list: vec![]};
        }
        
        pub fn add_edge(&mut self, edge: &'a Edge) {
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

        pub fn idx(&self) -> u16 {
            return self.m_idx;
        }

    }


}


fn main() {

    let mut example_graph =  graph::Graph::new();
    example_graph.build_graph();
    println!("Hello, world!");
}
