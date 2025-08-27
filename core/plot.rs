#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LineStyle {
    Solid,
    Dashed,
    Dotted,
}

impl From<LineStyle> for String {
    fn from(value: LineStyle) -> Self {
        match value {
            LineStyle::Solid => "solid".to_string(),
            LineStyle::Dashed => "dashed".to_string(),
            LineStyle::Dotted => "dotted".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Position {
    TopCenter,
    BottomCenter,
}

impl From<Position> for String {
    fn from(value: Position) -> Self {
        match value {
            Position::TopCenter => "top_center".to_string(),
            Position::BottomCenter => "bottom_center".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pane {
    pub boxes: Vec<Box>,
    pub labels: Vec<Label>,
}

impl Pane {
    pub fn new() -> Self {
        Self {
            boxes: Vec::new(),
            labels: Vec::new(),
        }
    }

    pub fn add_box(&mut self, box_: Box) {
        self.boxes.push(box_);
    }

    pub fn add_label(&mut self, label: Label) {
        self.labels.push(label);
    }
}

#[derive(Debug, Clone)]
pub struct Box {
    pub start_bar_index: usize,
    pub start_value: f64,
    pub end_bar_index: usize,
    pub end_value: f64,
    pub fill_color: Option<String>,
    pub line_color: Option<String>,
    pub line_style: LineStyle,
    pub line_width: usize,
}

impl Default for Box {
    fn default() -> Self {
        Self {
            start_bar_index: 0,
            start_value: 0.0,
            end_bar_index: 0,
            end_value: 0.0,
            fill_color: None,
            line_color: None,
            line_style: LineStyle::Solid,
            line_width: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Label {
    pub bar_index: usize,
    pub text: String,
    pub color: Option<String>,
    pub position: Position,
}

impl Default for Label {
    fn default() -> Self {
        Self {
            bar_index: 0,
            text: String::new(),
            color: None,
            position: Position::BottomCenter,
        }
    }
}
