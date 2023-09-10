//use std::collections::HashMap;
use std::fmt;

mod graph {
    use core::fmt;
    use std::collections::HashMap;


    pub struct Graph<'a>{
        //m_node_list: Vec<Node<'a>>,
        m_name: String,
        m_nodes: HashMap<u16, Node<'a>>,
        m_edges: HashMap<u16, Edge>,
    }

    /*
    impl <'a> fmt::Display  for Graph<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}",self.m_name)
        }
    }
    */

    impl <'a>Graph<'a>{

        pub fn new(name: String) -> Graph<'a> {
            // Create nodes
            let mut nodes: HashMap<u16, Node<'a>> = HashMap::new();
            let mut edges: HashMap<u16, Edge> = HashMap::new();

            let node_idx1: u16 = 1; 
            let node_idx2: u16 = 2;
            let node_idx3: u16 = 3;
            let mut all_node_idxs =  vec![node_idx1,node_idx2,node_idx3];
            let mut node1: Node = Node::new(node_idx1);
            let mut node2: Node = Node::new(node_idx2);
            let mut node3: Node = Node::new(node_idx3);

            nodes.insert(node1.idx(), node1);
            nodes.insert(node2.idx(), node2);
            nodes.insert(node3.idx(), node3);


            let edge_idx1: u16 = 1; 
            let edge_idx2: u16 = 2;
            let edge_idx3: u16 = 3;
            let mut all_edge_idxs =  vec![edge_idx1,edge_idx2,edge_idx3];
            let mut edge12: Edge = Edge::new(0,node_idx1,node_idx2);
            let mut edge13: Edge = Edge::new(1,node_idx1,node_idx3);
            let mut edge23: Edge = Edge::new(2,node_idx2,node_idx3);

            edges.insert(edge12.idx(),edge12);
            edges.insert(edge13.idx(),edge13);
            edges.insert(edge23.idx(),edge23);

            let mut build_graph = Graph{m_name: name,m_nodes: nodes, m_edges: edges};


            //let mut edge12: graph::Edge = graph::Edge::new(0);
            // Define nodes 
            for idx in all_node_idxs.iter() {
                match build_graph.m_nodes.get_mut(idx) {
                    Some(x) => {
                        for (key,value) in build_graph.m_edges.iter(){
                            if value.m_node_idx_end == x.idx() {
                                x.add_edge(value);
                            }
                            else if  value.m_node_idx_start == x.idx(){
                                x.add_edge(value);
                            }        
            
                        }
                            
                    },
                    None => panic!("Idx {} unknown",node_idx1)
                }

            }



            return build_graph;	
        }

        pub fn build_graph(&'a mut self) {
        
        
    
        //self.m_node_list[1].add_edge(&self.m_edge_list[0]);
        //self.m_node_list[0].add_edge(&self.m_edge_list[1]);
        //self.m_node_list[2].add_edge(&self.m_edge_list[1]);
        //self.m_node_list[1].add_edge(&self.m_edge_list[2]);
        //self.m_node_list[2].add_edge(&self.m_edge_list[2]);

        }

        pub fn print_graph(&'a mut self) {
            

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

    pub struct Node<'a> {
        m_idx: u16,
        m_incident_edges: Vec<&'a Edge>,
    }

    impl <'a> Node <'a>{
        pub fn new(idx: u16) -> Node<'a>{
            //let mut edge_list: Vec<&Edge> = vec![];
            return Node {m_idx: idx, m_incident_edges: vec![]};
        }
        
        pub fn add_edge(&mut self, edge: &'a Edge) {
            self.m_incident_edges.push(edge);
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

    //let mut example_graph: Box<graph::Graph>=  Box::new(graph::Graph::new("Example".to_string()));
    let mut example_graph: graph::Graph=  graph::Graph::new("Example".to_string());
    example_graph.build_graph();
    //example_graph.print_graph()
    
    print!("tt");
    //println!("Graph: {}", example_graph);
}
