use eframe::egui;
use egui::Pos2;
use crate::elements::{Vertex, Edge};
use nalgebra::DMatrix;
use rand::Rng;

const C1: f32 = 2.0;
const C2: f32 = 1.0;
const C3: f32 = 1.0;
const C4: f32 = 0.1;
const M: usize = 100;


pub fn spring_layout(adj_matrix: &DMatrix<f32>, width: f32, height: f32) -> (Vec<Vertex>, Vec<Edge>) {

    let n = adj_matrix.nrows();
    let mut rng = rand::thread_rng();

    let mut vertices = Vec::with_capacity(n);
    let mut edges = Vec::with_capacity(n);

    for i in 0..n {
        let x = rng.gen_range(0.0..width);
        let y = rng.gen_range(0.0..height);
        vertices.push(Vertex { pos: Pos2::new(x, y), colour: egui::Color32::WHITE });

        for j in 0..n {
            if adj_matrix[(i, j)] != 0.0 && i != j {
                edges.push(Edge { start: i, end: j });
            }
        }
    }

    
    
    (vertices, edges)
}



    




pub fn circle_layout(adj_matrix: &DMatrix<f32>, radius: f32, center: Pos2) -> (Vec<Vertex>, Vec<Edge>) {
    
    let n = adj_matrix.nrows();

    let mut vertices = Vec::with_capacity(n);
    let mut edges = Vec::with_capacity(n);

    let step = 2.0 * std::f32::consts::PI / n as f32;

    for i in 0..n {
        let angle = i as f32 * step;
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        vertices.push(Vertex { pos: Pos2::new(x, y), colour: egui::Color32::WHITE });

        for j in 0..n {
            if adj_matrix[(i, j)] != 0.0 && i != j {
                edges.push(Edge { start: i, end: j });
            }
        }
    }    
    (vertices, edges)
}

