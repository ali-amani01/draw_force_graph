extern crate draw_force_graph;
use nannou::prelude::*;
use draw_force_graph::DrawableGraph;

static MOVEMENT_SPEED: f32 = 20.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    drawable_graph: DrawableGraph,
    x: f32,
    y: f32,
    x0: f32,
    y0: f32,
}

// contains several sample dot files.
fn model(app: &App) -> Model {
    app.new_window().event(event_a).view(view).build().unwrap();

    //MANIFOLD LAYER GRAPHS
    let mut _drawable_graph = draw_force_graph::read_dot("dot/layer_kosarak-test_d=5.dot", 10.0);
    // let mut _drawable_graph = draw_force_graph::read_dot("dot/layer_arrhythmia_d=5.dot", 7.0);
    // let mut _drawable_graph = draw_force_graph::read_dot("dot/layer_gist-test_d=5.dot", 5.0);
    // let mut _drawable_graph = draw_force_graph::read_dot("dot/layer_fashion-mnist-test_d=5.dot", 2.0);


    //TREE GRAPHS
    // let mut _drawable_graph = draw_force_graph::read_dot("dot/tree_kosarak-test_d=6.dot", 5.0);
    // let mut _drawable_graph = draw_force_graph::read_dot("dot/tree_glove-200-test_d=6.dot", 5.0);
    // let mut _drawable_graph = draw_force_graph::read_dot("dot/tree_sift-train_d=6.dot", 1.0);
    // let mut _drawable_graph = draw_force_graph::read_dot("dot/tree_mnist-train_d=6.dot", 1.0);


    let _updated_drawable_graph = draw_force_graph::update_graph(_drawable_graph);

    return Model { drawable_graph: _updated_drawable_graph, x: 0.0, y: 0.0, x0: 0.0, y0: 0.0 };
}

// If the graph is too big for the screen to completely show, then
// this function allows for navigating the graph using the arrow keys
fn event_a(_app: &App, model: &mut Model, event: WindowEvent) {
    if let KeyPressed(direction) = event {
        if direction == nannou::event::Key::Down {
            model.y = model.y + MOVEMENT_SPEED;
        }
        if direction == nannou::event::Key::Up {
            model.y = model.y - MOVEMENT_SPEED;
        }
        if direction == nannou::event::Key::Right {
            model.x = model.x - MOVEMENT_SPEED;
        }
        if direction == nannou::event::Key::Left {
            model.x = model.x + MOVEMENT_SPEED;
        }
     }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.x0 = model.x0 + model.x0.cos()/10.0;
    model.y0 = model.y0 + model.x0.cos()/10.0;
}


// drawing the edges and nodes on a Nannou frame
fn view(app: &App, model: &Model, frame: Frame){

    frame.clear(WHITE);
    let draw = app.draw();

    draw_force_graph::draw_graph(&draw, &model.drawable_graph, model.x, model.y);

    draw.to_frame(app, &frame).unwrap();
}