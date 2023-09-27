
mod graph;
mod file_io;

fn main() {
    //let mut example_graph: Box<graph::Graph>=  Box::new(graph::Graph::new("Example".to_string()));
    let mut example_graph: graph::Graph = graph::Graph::new("Example".to_string());
    let graph_filename: String = "example_graph.xml".to_string();
    let mut dept_first_search_result: Vec<String> = Vec::new();
    example_graph.build_graph_from_file(graph_filename);
    //print!("tt {}", example_graph);

    dept_first_search_result = example_graph.depth_first_search("A".to_string());
    //print!("{}",dept_first_search_result);


    
    

    //println!("End");
    
}
