use super::frame::Frame;

#[allow(dead_code)]
pub trait Render {
    fn render(&self, frame: &Frame);
}
