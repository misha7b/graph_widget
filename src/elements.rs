use eframe::egui;
use egui::Pos2;
pub struct Vertex {
    pub pos: Pos2,
    pub colour: egui::Color32,
    pub dragging: bool,
}

pub struct Edge {
    pub start: Pos2,
    pub end: Pos2,
}
