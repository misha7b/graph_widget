use eframe::egui;
use egui::Pos2;
use crate::elements::{Vertex, Edge};

pub fn circle_plot(adj_lst: &Vec<Vec<usize>>, radius: f32, center: Pos2) -> (Vec<Vertex>, Vec<Edge>) {
    let n = adj_lst.len();

    let mut vertices = Vec::with_capacity(n);
    let mut edges = Vec::with_capacity(n);

    let step = 2.0 * std::f32::consts::PI / n as f32;

    for i in 0..n {
        let angle = i as f32 * step;
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        vertices.push(Vertex { pos: Pos2::new(x, y), dragging: false, colour: egui::Color32::BLUE });
    }

    for (i, neighbors) in adj_lst.iter().enumerate() {
        for &j in neighbors {
            edges.push(Edge { start: vertices[i].pos, end: vertices[j].pos });
        }
    }

    (vertices, edges)
}