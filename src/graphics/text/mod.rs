use graphics::Drawable;

pub mod font;
use self::font::Font;

use std::rc::Rc;
use std::cell::RefCell;

use gl_context::{Texture, Buffer, BufferType, shaders::OurShader};

use graphics::*;

pub struct Text<'a> {
    text: String,
    font: Rc<RefCell<Font<'a>>>,
    vertices: Buffer<f32>,
    indices: Buffer<u16>,
}

impl<'a> Drawable for Text<'a> {
    fn get_texture(&self) -> Option<Rc<Texture>> {
        let texture = self.font.borrow().get_texture();
        Some(texture)
        //None
    }

    fn get_shader(&self) -> Option<&OurShader> {
        Some(self.font.borrow().get_shader())
    }

    fn draw(&self) {
        render_target::draw_indices(&self.vertices, &self.indices, RenderStates::from(self));
    }
}

impl<'a> Text<'a> {
    pub fn new(text: String, font: Rc<RefCell<Font<'a>>>) -> Self {

        let vertices: Buffer<f32> = Buffer::new(BufferType::Array);
        let indices: Buffer<u16> = Buffer::new(BufferType::IndexArray);

        let mut t = Text {
            text,
            font: font.clone(),
            vertices,
            indices,
        };

        t.font.borrow_mut().update_texture(&t.text, &mut t.vertices, &mut t.indices);

        t
    }
}