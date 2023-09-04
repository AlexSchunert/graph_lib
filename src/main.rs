
mod graph {

    pub struct Graph<'a>{
        m_node_list: Vec<Node<'a>>,
        m_edge_list: Vec<Edge<'a>>,
    }

    impl <'a> Graph<'a>{

        pub fn new() -> Graph <'a>{
            

            return Graph{m_node_list: vec![], m_edge_list:vec![]};
        }

        pub fn build(&'a mut self) {
            //return Graph{m_node_list: vec![], m_edge_list: vec![]};


            let mut node1: Node = Node::new(0);
            let mut node2: Node = Node::new(1);
            let mut node3: Node = Node::new(2);
            self.m_node_list.push(node1);
            self.m_node_list.push(node2);
            self.m_node_list.push(node3);

            let mut edge12: Edge = Edge::new(0,&self.m_node_list[0],&self.m_node_list[1]);
            let mut edge13: Edge = Edge::new(1,&self.m_node_list[0],&self.m_node_list[2]);
            let mut edge23: Edge = Edge::new(2,&self.m_node_list[1],&self.m_node_list[2]);
            self.m_edge_list.push(edge12);
            self.m_edge_list.push(edge13);
            self.m_edge_list.push(edge23);


            self.m_node_list[0].add_edge(&self.m_edge_list[0]);
            //let mut edge12: Edge = Edge::new(0,&node1,&node2);
            //let mut edge13: Edge = Edge::new(1,&node1,&node3);d
            //let mut edge23: Edge = Edge::new(2,&node2,&node3);
            
                    
            //let mut edge12: graph::Edge = graph::Edge::new(0);
            // Define nodes 
            //node1.add_edge(&edge12);
            //node2.add_edge(&edge12);
            //node1.add_edge(&edge13);
            //node3.add_edge(&edge13);
            //node2.add_edge(&edge23);
            //node3.add_edge(&edge23);
                    
            // push to vectors
           

            

    

        }


        pub fn add_node(&mut self,node: Node<'a>) {
            self.m_node_list.push(node);

        }

        pub fn add_edge(&mut self, edge: Edge<'a>) {
            self.m_edge_list.push(edge);
        }

        pub fn sort_node_list(&mut self) {
            self.m_node_list.sort_by(|a,b|a.m_idx.cmp(&b.m_idx));

        }

    }

    pub struct Node<'a> {
        m_idx: u16,
        m_edge_list: Vec<&'a Edge<'a>>,
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

    pub struct Edge<'a>{
        m_idx: u16,
        m_node_start: &'a Node<'a>,
        m_node_end: &'a Node<'a>,
    }

    impl <'a> Edge<'a>{
        //pub fn new(idx: u16, node_start: &'a Node, node_end: &'a Node) -> Edge<'a>{    
        //    return Edge{m_idx: idx, m_node_start: node_start, m_node_end: node_end};
        //}
        pub fn new(idx: u16, node_start: &'a Node, node_end: &'a Node) -> Edge<'a>{

            return Edge{m_idx: idx, m_node_start: node_start, m_node_end: node_end};
        }

    

    }


}


fn main() {

    // Make list of nodes 
    

    
    let mut example_graph =  graph::Graph::new();
    example_graph.build();
    //example_graph.add_node(node1);
    //example_graph.add_node(node3);
    //example_graph.add_node(node2);
    //example_graph.add_edge(edge12);
    //example_graph.add_edge(edge13);
    //example_graph.add_edge(edge23);
    //example_graph.sort_node_list();
    println!("Hello, world!");
}
