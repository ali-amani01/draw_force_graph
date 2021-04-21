extern crate nannou;
extern crate force_graph;
extern crate petgraph;
use std::fs::File;
use std::io::{BufRead, BufReader};
use nannou::prelude::*;
use std::collections::HashMap;
use petgraph::graph::NodeIndex;
use force_graph::ForceGraph;
use force_graph::Node;

// the amount of force applied to the force-directed graph
// during each update
static GRAPH_FORCE: f32 = 0.02;

// the range for the initial randomized coordinates of each node
static GRAPH_RIGHT_RANGE: f32 = 500.0;
static GRAPH_LEFT_RANGE: f32 = -500.0;
static GRAPH_UP_RANGE: f32 = GRAPH_RIGHT_RANGE;
static GRAPH_DOWN_RANGE: f32 = GRAPH_LEFT_RANGE;

// the data for each node
#[derive(Clone)]
pub struct NodeData {
    name: String,       // name of the cluster
    cardinality: i32,   // the cluster's cardinality
    radius: f32,        // the radius of the cluster
    lfd: f32,           // the local fractal dimension of the cluster
    color: Rgb,         // the color of the cluster
    degree: i32,        // the degree of the node, or amount of connected edges
}

// the force-directed graph and all variables relevant to drawing it
pub struct DrawableGraph {
    force_graph: ForceGraph<NodeData>,                          // the force-directed graph, to which we can apply the force-direct algorithm
    index_map: HashMap<String, petgraph::prelude::NodeIndex>,   // a hashmap of all node indexes and their corresponding names
    edge_map: HashMap<String, (NodeIndex, NodeIndex)>,          // a hashmap of all edges and the indexes of the nodes they connect
    total_nodes: i32,                                           // the total number of nodes in the graph
    total_disconnected_nodes: i32,                              // the total number of disconnected nodes in the graph
    graph_dimensions: Vec<f32>,                                 // a vector of the graph's top, bottom, rightmost, and leftmost distances from point (0, 0)
    graph_draw_scale: f32,                                       // the scaling of the graph as it will be drawn
}

// calculating the mass of a node based on its degree
// and the drawing scale of the graph.
fn get_node_mass(degree: i32, scale: f32) -> f32{
    return (scale * 0.6) * degree as f32;
}

// reading the dot file and returning a drawable force graph. Takes in the path
// of the dot file and the drawing scale of the graph
pub fn read_dot(dot_file: &str, _graph_draw_scale: f32) -> DrawableGraph {

    let file = File::open(dot_file).unwrap();
    let reader = BufReader::new(file);

    let mut _force_graph: ForceGraph<NodeData> = ForceGraph::new();

    let mut uploaded_nodes: HashMap<String, NodeData> = HashMap::new();
    let mut uploaded_edges: HashMap<String, (String, String)> = HashMap::new();

    let mut _index_map: HashMap<String, petgraph::prelude::NodeIndex> = HashMap::new();
    let mut _edge_map: HashMap<String, (petgraph::prelude::NodeIndex, petgraph::prelude::NodeIndex)> = HashMap::new();

    let is_tree;

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if is_node(&line){

            // reading the nodes from the dot file
            let new_node_data = get_node_data(line);
            let _node_data = new_node_data.clone();
            uploaded_nodes.insert(_node_data.clone().name.to_string(), _node_data);
        }

        else if is_edge(&line){

            // reading the edges from the dot file
            let (node_1, node_2) = get_edge_data(line);

            // incrementing the degree of the nodes each edge connects
            uploaded_nodes.get_mut(&node_1).unwrap().degree += 1;
            uploaded_nodes.get_mut(&node_2).unwrap().degree += 1;

            let edge_index = format!("{:?} -- {:?}", &node_1.to_string(), &node_2.to_string());
            uploaded_edges.insert(edge_index, (node_1, node_2));
        }
    }

    let mut i: i32 = 0;
    let mut j: i32 = 0;

    let mut _disconnected_x = 0.0;
    let mut _disconnected_y = 0.0;

    is_tree = uploaded_nodes.contains_key("root");

    // adding the nodes to the force-directed graph
    for (_name, _node_data) in uploaded_nodes {
        let node_index;

        // getting the node mass from the node's degree and the drawing scale of the graph
        let _node_mass = get_node_mass(_node_data.degree, _graph_draw_scale);

        // each disconnected node is set to be drawn on the lower right side of the connected graph
        if _node_data.degree == 0 {
            node_index = _force_graph.add_node(_disconnected_x, _disconnected_y, _node_data, _node_mass, false);
            _disconnected_y += _graph_draw_scale * 15.0;
            j += 1;
        }
        else{
            // if the current node is either the root of a tree graph or the first node of a
            // non-tree graph, then we define it as an anchor node, or one that will not be
            // moved with each force-directed update
            if (_node_data.name == "root" && is_tree) || (i == 0 && !is_tree) {
                node_index = _force_graph.add_node(0.0, 0.0, _node_data, _node_mass, true);
            }
            else{
                node_index = _force_graph.add_node(random_range::<f32>(GRAPH_LEFT_RANGE, GRAPH_RIGHT_RANGE), random_range::<f32>(GRAPH_DOWN_RANGE, GRAPH_UP_RANGE), _node_data, _node_mass, false);
            }
        }
        _index_map.insert(_name.to_string(), node_index);
        i+=1;
    }

    // adding the edges to the force-directed graph
    for (_name1, (_node1, _node2)) in uploaded_edges {
        let node_index_1 = _index_map[&_node1.to_string()];
        let node_index_2 = _index_map[&_node2.to_string()];
        _edge_map.insert(_name1, (node_index_1, node_index_2));

        _force_graph.add_edge(node_index_1, node_index_2);
    }

    // adding the force-directed graph to the drawable graph
    let _drawable_graph = DrawableGraph {
        force_graph: _force_graph,
        index_map: _index_map,
        edge_map: _edge_map,
        total_disconnected_nodes: j,
        total_nodes: i,
        graph_dimensions: [0.0, 0.0, 0.0, 0.0].to_vec(),    // to be determined after applying force-directed updates
        graph_draw_scale: _graph_draw_scale,
    };

    return _drawable_graph;
}


// reading a line of the dot file and returning true if the line is an edge, and false if not
fn is_edge(file_line: &String) -> bool{
    !file_line.contains("{") && !file_line.contains("}") && file_line.contains(" -")
}

// reading a line of the dot file and returning true if the line is a node, and false if not
fn is_node(file_line: &String) -> bool{
    !file_line.contains("{") && !file_line.contains("}") && !file_line.contains(" -") && !file_line.contains("edge")
}

// parsing the given line to retrieve its node data, and returning it to
// be attached to the appropriate graph node
fn get_node_data(node_line: String) -> NodeData{
    let _node_string_1: Vec<&str> = node_line.split('[').collect();

    let _node_string_2: Vec<&str> = _node_string_1[1].split('\\').collect();
    let _node_string_3: Vec<&str> = _node_string_2[3].split("\", ").collect();

    let _node_name: String = _node_string_1[0].replace(" ", "");
    let _node_cardinality : String = _node_string_2[1].replace("ncardinality ", "");
    let _node_radius: String = _node_string_2[2].replace("nradius ", "");
    let _node_lfd: String = _node_string_3[0].replace("nlfd ", "");
    
    let _node_color_1: String = _node_string_3[1].replace("color=\"", "");
    let _node_color = get_color_from_hex(_node_color_1.clone());
    
    let new_node_data = NodeData {
        name: _node_name,
        cardinality: _node_cardinality.parse::<i32>().unwrap(),
        radius: _node_radius.parse::<f32>().unwrap(),
        lfd: _node_lfd.parse::<f32>().unwrap(),
        color: _node_color,
        degree: 0,
    };
    return new_node_data;
}

// getting the color in RGB form from the hexadecimal color taken from the dot file
fn get_color_from_hex(hex_str: String) -> nannou::color::rgb::Rgb{
    let r_hex = &hex_str[1..3];
    let b_hex = &hex_str[5..7];

    let r = f32::from_str_radix(r_hex, 16).unwrap();
    let b = f32::from_str_radix(b_hex, 16).unwrap();

    return Rgb::new(r, 0.0, b);
}

// parsing the given line, which defines either a directed
// or undirected edge, and returning the names of the two
// nodes the edge connects
fn get_edge_data(edge_line: String) -> (String, String){
    if edge_line.contains(" -> "){
        let _edge_string_1: Vec<&str> = edge_line.split(" -> ").collect();
    
        let _node_1: String = _edge_string_1[0].split_whitespace().collect();
        let _node_2: String = _edge_string_1[1].to_string();
    
        return (_node_1, _node_2);
    }
    else{
        let _edge_string_1: Vec<&str> = edge_line.split(" -- ").collect();
        let _edge_string_2: Vec<&str> = _edge_string_1[1].split(" [").collect();
        
        let _node_1: String = _edge_string_1[0].split_whitespace().collect();
        let _node_2: String = _edge_string_2[0].to_string();
    
        return (_node_1, _node_2);
    }
}

// drawing a node as an ellipse and its labels as drawn text
pub fn draw_node(_draw: &nannou::Draw, node: &Node<NodeData>, center_x: f32, center_y: f32, _graph_draw_scale: f32){
    _draw.ellipse().color(BLACK).w((_graph_draw_scale * 10.0)+(_graph_draw_scale/3.0)).h((_graph_draw_scale * 10.0)+(_graph_draw_scale/3.0)).x(node.x + center_x).y(node.y + center_y);
    _draw.ellipse().color(node.data.color).w(_graph_draw_scale * 10.0).h(_graph_draw_scale * 10.0).x(node.x + center_x).y(node.y + center_y);
    _draw.text(&node.data.name).w(_graph_draw_scale * 10.0).h(_graph_draw_scale * 10.0).font_size(_graph_draw_scale as u32).x(node.x + center_x).y(node.y+((_graph_draw_scale * 10.0)*0.21) + center_y);
    _draw.text(&format!("card: {:?}", &node.data.cardinality)).font_size(_graph_draw_scale as u32).w(_graph_draw_scale * 10.0).h(_graph_draw_scale * 10.0).x(node.x + center_x).y(node.y+((_graph_draw_scale * 10.0)*0.06) + center_y);
    _draw.text(&format!("rad: {:.1$}", &node.data.radius, 2)).font_size(_graph_draw_scale as u32).w(_graph_draw_scale * 10.0).h(_graph_draw_scale * 10.0).x(node.x + center_x).y(node.y-((_graph_draw_scale * 10.0)*0.09) + center_y);
    _draw.text(&format!("lfd: {:.1$}", &node.data.lfd, 2)).font_size(_graph_draw_scale as u32).w(_graph_draw_scale * 10.0).h(_graph_draw_scale * 10.0).x(node.x + center_x).y(node.y-((_graph_draw_scale * 10.0)*0.25) + center_y);
}

// having the graph go through a number of force-directed updates proportional
// to its number of connected nodes, and getting the updated graph's dimensions,
// represented as a vector of it's highest/lowest/rightmost/leftmost distances
// from point (0, 0)
pub fn update_graph(mut _drawable_graph: DrawableGraph) -> DrawableGraph{

    let _connected_graph_nodes = _drawable_graph.total_nodes - _drawable_graph.total_disconnected_nodes;

    let _updates = i32::pow(_connected_graph_nodes, 2) * 5;

    for _i in 0.._updates{
        _drawable_graph.force_graph.update(GRAPH_FORCE);
    }

    let _graph = &_drawable_graph.force_graph;
    let _index_map = &_drawable_graph.index_map;

    let mut _top = 0.0;
    let mut _bottom = 0.0;
    let mut _right = 0.0;
    let mut _left = 0.0;

    // iterating through each node in the graph to get the highest, lowest, rightmost,
    // and leftmost points
    for (_, &index) in _index_map {
        let current_x = *&_graph.get_graph()[index].x as f32;
        let current_y = *&_graph.get_graph()[index].y as f32;

        if current_y < _bottom {
            _bottom = current_y
        }
        if _top < current_y {
            _top = current_y
        }
        if _right < current_x {
            _right = current_x
        }
        if current_x < _left {
            _left = current_x
        }
    }
    
    // updating the graph dimensions
    _drawable_graph.graph_dimensions = vec![_top, _bottom, _left, _right];

    return _drawable_graph;
}

// drawing the graph on a Nannou frame
pub fn draw_graph(draw: &nannou::Draw, _drawable_graph: &DrawableGraph, center_x: f32, center_y: f32){

    let _graph = &_drawable_graph.force_graph;
    let _index_map = &_drawable_graph.index_map;
    let _edge_map = &_drawable_graph.edge_map;

    let _graph_draw_scale = &_drawable_graph.graph_draw_scale;

    // getting the coordinates for the bottom rightmost side of the graph to draw the disconnected nodes
    let mut _rightmost_corner = *&_drawable_graph.graph_dimensions[3];
    let mut _bottom_corner = *&_drawable_graph.graph_dimensions[1];

    // drawing the edges
    for (_, &(node1, node2)) in _edge_map {
        let start_point = pt2(*&_graph.get_graph()[node1].x + center_x, *&_graph.get_graph()[node1].y + center_y);
        let end_point = pt2(*&_graph.get_graph()[node2].x + center_x, *&_graph.get_graph()[node2].y + center_y);
        draw.line().start(start_point).end(end_point).weight(*_graph_draw_scale*0.5).color(BLACK);
    }

    // drawing the nodes
    for (_, &index) in _index_map {
        if _graph.get_graph()[index].data.degree == 0{
            draw_node(&draw, &_graph.get_graph()[index], center_x + _rightmost_corner + (_graph_draw_scale * 15.0), center_y + _bottom_corner, *_graph_draw_scale);
        }
        else{
            draw_node(&draw, &_graph.get_graph()[index], center_x, center_y, *_graph_draw_scale);
        }
    }
}