mod elements;
mod layout;

use eframe::egui;
use egui::{Color32, Pos2};
use elements::{Vertex, Edge};

#[derive(Default)]
struct GraphApp {

    vertices: Vec<Vertex>,
    edges: Vec<Edge>,

}

impl GraphApp {

    fn name() -> &'static str {
        "GraphApp"
    }


}


impl eframe::App for GraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Quit").clicked() {
                std::process::exit(0);
            }

            let painter = ui.painter();
            let (hover_pos, any_down) =
                ctx.input(|input| (input.pointer.hover_pos(), input.pointer.any_down()));
            
           
            for edge in &self.edges {
                painter.line_segment([edge.start, edge.end], (1.0, Color32::WHITE));
            }   

            for vertex in &self.vertices {
                painter.circle_filled(vertex.pos, 5.0, Color32::from_rgb(74, 178, 191));
            }

            ui.label(format!("{:?}", hover_pos));
            ui.label(format!("{}", any_down));
        
        });

        
    

        ctx.request_repaint();
    }
}


fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((400.0, 400.0)),
        ..eframe::NativeOptions::default()
    };

    let mut app = GraphApp::default();


    let center = Pos2::new(200.0, 200.0);
    let radius = 150.0;
    let n = 15;
    let (vertices, edges) = layout::circle_plot(n, radius, center);

    for vertex in vertices {
        app.vertices.push(vertex);
    }

    for edge in edges {
        app.edges.push(edge);
    }

    //app.edges.push(Edge { start: Pos2::new(100.0, 100.0), end: Pos2::new(200.0, 200.0) });
    //app.edges.push(Edge { start: Pos2::new(200.0, 200.0), end: Pos2::new(300.0, 300.0) });
    //app.edges.push(Edge { start: Pos2::new(300.0, 300.0), end: Pos2::new(400.0, 400.0) });

    
    eframe::run_native(
        GraphApp::name(),
        native_options,
        Box::new(|_| Box::new(app)),
    )
}