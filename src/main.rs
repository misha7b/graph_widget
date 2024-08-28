mod elements;
mod layout;

use eframe::egui;
use egui::{Color32, Pos2};
use elements::{Vertex, Edge};

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


use nalgebra::{DMatrix, SymmetricEigen};

#[derive(Default)]
struct GraphApp {

    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    adj_matrix: DMatrix<f32>,
    laplacian: DMatrix<f32>,
    dragged_vertex: Option<usize>,
    

}

impl GraphApp {

    fn name() -> &'static str {
        "GraphApp"
    }

    fn partition_graph(&mut self) {
        let fiedler_vector = calc_fiedler_vector(&self.laplacian);

        let mut group1 = Vec::new();
        let mut group2 = Vec::new();

        for (i, vertex) in self.vertices.iter_mut().enumerate() {
            if fiedler_vector[i] < 0.0 {
                vertex.colour = Color32::from_rgb(255, 99, 71);
                group1.push(i);
            } else {
                vertex.colour = Color32::from_rgb(70, 130, 180); 
                group2.push(i);
            }
        }

        let (vertices1, _) = layout::circle_layout(&self.adj_matrix.select_rows(&group1), 100.0, Pos2::new(150.0, 300.0));
        let (vertices2, _) = layout::circle_layout(&self.adj_matrix.select_rows(&group2), 100.0, Pos2::new(450.0, 300.0));

        for (i, &index) in group1.iter().enumerate() {
            self.vertices[index].pos = vertices1[i].pos;
        }

        for (i, &index) in group2.iter().enumerate() {
            self.vertices[index].pos = vertices2[i].pos;
        }

    }

    fn reset_graph(&mut self) {
        let (vertices, edges) = layout::circle_layout(&self.adj_matrix, 200.0, Pos2::new(300.0, 300.0));
        self.vertices = vertices;
        self.edges = edges;
        for vertex in &mut self.vertices {
            vertex.colour = Color32::WHITE;
        }
    }

}


impl eframe::App for GraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {


        egui::CentralPanel::default().show(ctx, |ui| {


        
            if ui.button("Quit").clicked() {
                std::process::exit(0);
            }

            if ui.button("Reset").clicked() {
                self.reset_graph();
            }

            if ui.button("Partition Graph").clicked() {
                self.partition_graph();
            }

            let painter = ui.painter();
            
            
            let (hover_pos, any_down) =
                ctx.input(|input| (input.pointer.hover_pos(), input.pointer.any_down()));

            if let Some(mouse_pos) = hover_pos {
                if any_down {
                    let mut new_pos = mouse_pos;
                    new_pos.x = new_pos.x.clamp(10.0, 790.0);
                    new_pos.y = new_pos.y.clamp(10.0, 790.0);
            
                    if let Some(dragged_vertex) = self.dragged_vertex {
                        self.vertices[dragged_vertex].pos = new_pos;
                    } else {
                        for (i, vertex) in self.vertices.iter_mut().enumerate() {
                            let distance = (vertex.pos - mouse_pos).length();
                            if distance < 10.0 {
                                self.dragged_vertex = Some(i);
                                vertex.pos = new_pos;
                                break;
                            }
                        }
                    }
                } else {
                    self.dragged_vertex = None;
                }
            }

            for edge in &self.edges {
                let start = self.vertices[edge.start].pos;
                let end = self.vertices[edge.end].pos;
                painter.line_segment([start, end], (1.0, Color32::GRAY));
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
        viewport: egui::ViewportBuilder::default().with_inner_size((600.0, 600.0)),
        ..eframe::NativeOptions::default()
    };


    let adj_matrix = read_matrix_from_file("karate.txt").expect("Failed to read matrix from file");
    

    let laplacian = calc_laplacian(&adj_matrix);

    let (vertices, edges) = layout::spring_layout(&adj_matrix, 600.0, 600.0);
    //let (vertices, edges) = layout::circle_layout(&adj_matrix, 200.0, Pos2::new(300.0, 300.0));

    let mut app = GraphApp {
        vertices: Vec::new(),
        edges: Vec::new(),
        adj_matrix: adj_matrix,
        laplacian: laplacian,
        dragged_vertex: None,

    };

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


fn calc_laplacian(adj_matrix: &DMatrix<f32>) -> DMatrix<f32> {

    let n = adj_matrix.nrows();
    let mut deg_matrix = DMatrix::zeros(n, n);
    let mut deg_inv_sqrt = DMatrix::zeros(n, n);
    


    for i in 0..n {
        let d: f32 = adj_matrix.row(i).sum();
        deg_matrix[(i, i)] = d;
        deg_inv_sqrt[(i, i)] = 1.0 / d.sqrt();
    }
    
    //let laplacian = deg_matrix - adj_matrix;
    let norm_laplacian = &deg_inv_sqrt * (deg_matrix - adj_matrix) * &deg_inv_sqrt;

    norm_laplacian

}

fn calc_fiedler_vector(laplacian: &DMatrix<f32>) -> Vec<f32> {

    let eigen = SymmetricEigen::new(laplacian.clone());
    let eigenvalues = eigen.eigenvalues;
    let eigenvectors = eigen.eigenvectors;

    let mut eigenvalue_indices: Vec<usize> = (0..eigenvalues.len()).collect();
    eigenvalue_indices.sort_by(|&i, &j| eigenvalues[i].partial_cmp(&eigenvalues[j]).unwrap());

    let fiedler_vector = eigenvectors.column(eigenvalue_indices[1]).iter().cloned().collect();

    fiedler_vector

}

fn read_matrix_from_file<P: AsRef<Path>>(file_path: P) -> io::Result<DMatrix<f32>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut rows: Vec<Vec<f32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let row: Vec<f32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap_or(0.0))
            .collect();
        rows.push(row);
    }

    let nrows = rows.len();
    let ncols = rows[0].len();
    let data: Vec<f32> = rows.into_iter().flatten().collect();

    Ok(DMatrix::from_vec(nrows, ncols, data))
}


    
