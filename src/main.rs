
mod graph;
mod file_io;

fn main() {
    //let mut example_graph: Box<graph::Graph>=  Box::new(graph::Graph::new("Example".to_string()));
    let mut example_graph: graph::Graph = graph::Graph::new("Example".to_string());
    let graph_filename: String = "example_graph.xml".to_string();
    example_graph.build_graph_from_file(graph_filename);
    print!("tt {}", example_graph);

    
    

    //println!("End");
    
}
