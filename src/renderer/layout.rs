use image::Rgba;

pub struct Layout {
    pub header_day_pos: (u32, u32),
    pub header_date_pos: (u32, u32),

    pub first_row_y: u32,
    pub row_height: u32,

    pub impact_icon_pos_x: u32,
    pub title_pos_x: u32,
    pub right_text_pos_x: u32,

    pub title_color: Rgba<u8>,
    pub meta_color: Rgba<u8>,
}

impl Layout {
    pub fn default() -> Self {
        Self {
            header_day_pos: (170, 115),   // center-based later
            header_date_pos: (115, 245),

            first_row_y: 390,
            row_height: 90,

            impact_icon_pos_x: 50,
            title_pos_x: 90,
            right_text_pos_x: 415,

            title_color: Rgba([30, 30, 30, 255]),
            meta_color: Rgba([120, 120, 120, 255]),
        }
    }
}
