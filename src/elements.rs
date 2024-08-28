use eframe::egui;
use egui::Pos2;
pub struct Vertex {
    pub pos: Pos2,
    pub colour: egui::Color32,
}

pub struct Edge {
    pub start: usize,
    pub end: usize,
}
