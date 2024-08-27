mod elements;
mod layout;

use eframe::egui;
use egui::{Color32, Pos2};
use elements::{Vertex, Edge};

use nalgebra::{DMatrix, SymmetricEigen};

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

            /* 
            if let Some(pos) = hover_pos {
                if any_down {
                    for vertex in &mut self.vertices {
                        if vertex.pos.distance(pos) < 20.0 {
                            vertex.pos = pos;
                            break;
                        }
                    }
                }
            }     
            */
            
           
            for edge in &self.edges {
                painter.line_segment([edge.start, edge.end], (1.0, Color32::WHITE));
            }   

            for vertex in &self.vertices {
                painter.circle_filled(vertex.pos, 5.0, vertex.colour);
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
    
    let adj_list = vec![
        vec![1, 2],
        vec![0, 2],
        vec![0, 1, 3],
        vec![2, 4, 6],
        vec![3, 5],
        vec![4, 6],
        vec![3, 5],
    ];

    let laplacian = calc_laplacian(&adj_list);

    let eigen = SymmetricEigen::new(laplacian.clone());
    let eigenvalues = eigen.eigenvalues;
    let eigenvectors = eigen.eigenvectors;

    let mut eigenvalue_indices: Vec<usize> = (0..eigenvalues.len()).collect();
    eigenvalue_indices.sort_by(|&i, &j| eigenvalues[i].partial_cmp(&eigenvalues[j]).unwrap());

    let fiedler_vector = eigenvectors.column(eigenvalue_indices[1]).iter().cloned().collect();



    let (vertices, edges) = layout::circle_plot(&adj_list, radius, center, &fiedler_vector);

    for vertex in vertices {
        app.vertices.push(vertex);
    }

    for edge in edges {
        app.edges.push(edge);
    }

    eframe::run_native(
        GraphApp::name(),
        native_options,
        Box::new(|_| Box::new(app)),
    )
}


fn calc_laplacian(adj_lst: &Vec<Vec<usize>>) -> DMatrix<f32> {

    let n = adj_lst.len();
    let mut deg_matrix = DMatrix::zeros(n, n);
    let mut deg_inv_sqrt = DMatrix::zeros(n, n);
    let mut adj_matrix = DMatrix::zeros(n, n);
    


    for (i, neighbours) in adj_lst.iter().enumerate() {

        let d = neighbours.len() as f32;
        deg_matrix[(i, i)] = d;
        deg_inv_sqrt[(i, i)] = 1.0 / d.sqrt();
        for &j in neighbours {
            adj_matrix[(i, j)] = 1.0;
        }
    
    }
    //let laplacian = deg_matrix - adj_matrix;
    let norm_laplacian = &deg_inv_sqrt * (deg_matrix - adj_matrix) * &deg_inv_sqrt;

    norm_laplacian

}



