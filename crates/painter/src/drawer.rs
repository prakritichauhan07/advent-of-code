use crate::buffer::CircularOutputBuffer;
use advent_of_code::painter::Painter;

enum Command {
    BeginPath = 1,
    Clear,
    ClosePath,
    FillRect,
    FillSquare,
    EndFrame,
    FillStyleRGB,
    LineWidth,
    StrokeSquare,
    StrokeStyleRGB,
    StatusText,
    ShadowBlur,
    ShadowColor,
    Done,
    Delay,
    SwitchLayer,
    FillStyleRGBA,
    SetAspectRatio,
    Arc,
    Fill,
    Stroke,
    LineTo,
    MoveTo,
}

pub struct ToBufferDrawer {
    pub output_buffer: CircularOutputBuffer,
    aspect_ratio: f64,
}

impl ToBufferDrawer {
    pub fn new() -> ToBufferDrawer {
        Self {
            output_buffer: CircularOutputBuffer::new(),
            aspect_ratio: 1.0,
        }
    }
}

impl Painter for ToBufferDrawer {
    fn clear(&mut self) {
        self.output_buffer.write(Command::Clear as i32);
    }

    fn begin_path(&mut self) {
        self.output_buffer.write(Command::BeginPath as i32);
    }

    fn close_path(&mut self) {
        self.output_buffer.write(Command::ClosePath as i32);
    }

    fn fill_rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
        self.output_buffer.write(Command::FillRect as i32);
        self.output_buffer.write_float4(x, y, w, h);
    }

    fn fill_square(&mut self, x: f64, y: f64, size: f64) {
        self.output_buffer.write(Command::FillSquare as i32);
        self.output_buffer.write_float3(x, y, size);
    }

    fn status_text(&mut self, text: &str) {
        self.output_buffer.write(Command::StatusText as i32);
        self.output_buffer.write_text(text);
    }

    fn fill(&mut self) {
        self.output_buffer.write(Command::Fill as i32);
    }

    fn stroke_square(&mut self, x: i32, y: i32, size: i32) {
        self.output_buffer.write(Command::StrokeSquare as i32);
        self.output_buffer.write(x);
        self.output_buffer.write(y);
        self.output_buffer.write(size);
    }

    fn stroke_style_rgb(&mut self, r: i32, g: i32, b: i32) {
        self.output_buffer.write(Command::StrokeStyleRGB as i32);
        self.output_buffer.write(r);
        self.output_buffer.write(g);
        self.output_buffer.write(b);
    }

    fn stroke(&mut self) {
        self.output_buffer.write(Command::Stroke as i32);
    }

    fn line_width(&mut self, width: f64) {
        self.output_buffer.write(Command::LineWidth as i32);
        self.output_buffer.write_float(width);
    }

    fn line_to(&mut self, x: f64, y: f64) {
        self.output_buffer.write(Command::LineTo as i32);
        self.output_buffer.write_float(x);
        self.output_buffer.write_float(y);
    }

    fn move_to(&mut self, x: f64, y: f64) {
        self.output_buffer.write(Command::MoveTo as i32);
        self.output_buffer.write_float(x);
        self.output_buffer.write_float(y);
    }

    fn fill_style_rgb(&mut self, r: i32, g: i32, b: i32) {
        self.output_buffer.write(Command::FillStyleRGB as i32);
        self.output_buffer.write(r);
        self.output_buffer.write(g);
        self.output_buffer.write(b);
    }

    fn fill_style_rgba(&mut self, r: i32, g: i32, b: i32, a: f64) {
        self.output_buffer.write(Command::FillStyleRGBA as i32);
        self.output_buffer.write(r);
        self.output_buffer.write(g);
        self.output_buffer.write(b);
        self.output_buffer.write_float(a);
    }

    fn shadow_color(&mut self, r: i32, g: i32, b: i32) {
        self.output_buffer.write(Command::ShadowColor as i32);
        self.output_buffer.write(r);
        self.output_buffer.write(g);
        self.output_buffer.write(b);
    }

    fn shadow_blur(&mut self, level: i32) {
        self.output_buffer.write(Command::ShadowBlur as i32);
        self.output_buffer.write(level);
    }

    fn arc(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) {
        self.output_buffer.write(Command::Arc as i32);
        self.output_buffer.write_float(x);
        self.output_buffer.write_float(y);
        self.output_buffer.write_float(radius);
        self.output_buffer.write_float(start_angle);
        self.output_buffer.write_float(end_angle);
    }

    fn end_frame(&mut self) {
        self.output_buffer.write(Command::EndFrame as i32);
        self.output_buffer.flush();
        self.output_buffer.perhaps_wait();
    }

    fn meta_delay(&mut self, delay_ms: u16) {
        self.output_buffer.write(Command::Delay as i32);
        self.output_buffer.write(delay_ms as i32);
        // FIXME: flush? end_frame?
        self.output_buffer.flush();
    }

    fn meta_switch_layer(&mut self, to_layer: u16) {
        self.output_buffer.write(Command::SwitchLayer as i32);
        self.output_buffer.write(to_layer as i32);
    }

    fn set_aspect_ratio(&mut self, width: i32, height: i32) {
        self.output_buffer.write(Command::SetAspectRatio as i32);
        self.aspect_ratio = width as f64 / height as f64;
        self.output_buffer.write_float(self.aspect_ratio);
        self.output_buffer.flush();
    }

    fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    fn await_forever(&mut self) {
        self.output_buffer.write(Command::Done as i32);
        self.output_buffer.flush();
        self.output_buffer.wait_forever();
    }

    fn log(&mut self, text: &str) {
        self.output_buffer.log(text);
    }
}

impl Drop for ToBufferDrawer {
    fn drop(&mut self) {
        self.await_forever();
    }
}
