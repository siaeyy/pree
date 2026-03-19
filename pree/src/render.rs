use std::cell::RefCell;

use rand::{RngExt, rngs::ThreadRng};
use speedy2d::{Graphics2D, color::Color, dimen::Vec2};

use crate::tree::TreeTheme;

// For creating color variant between two base color
#[derive(Debug, Clone)]
pub struct ColorVariant {
    r: f32,
    g: f32,
    b: f32,
}

impl ColorVariant {
    pub fn new() -> Self {
        Self::new_with_generator(&mut rand::rng())
    }

    pub fn new_with_generator(rng: &mut ThreadRng) -> Self {
        Self {
            r: rng.random(),
            g: rng.random(),
            b: rng.random(),
        }
    }

    // Get a color between two base color by variant values
    pub fn get_color(&self, start_color: &Color, end_color: &Color) -> Color {
        let (r0, r1) = (start_color.r(), end_color.r());
        let (g0, g1) = (start_color.g(), end_color.g());
        let (b0, b1) = (start_color.b(), end_color.b());

        let r_gap = r1 - r0;
        let g_gap = g1 - g0;
        let b_gap = b1 - b0;

        let r = r0 + (r_gap * self.r);
        let g = g0 + (g_gap * self.g);
        let b = b0 + (b_gap * self.b);

        Color::from_rgb(r, g, b)
    }
}

// Stuffs that needed for rendering a tree
pub struct RenderContext<'a> {
    pub gfx: RefCell<&'a mut Graphics2D>,
    pub theme: &'a TreeTheme,
}

pub trait Render {
    fn render<'a>(&self, ctx: &RenderContext<'a>);
}

#[derive(Clone, Copy)]
pub struct Viewport {
    width: u32,
    height: u32,
}

impl Into<(u32, u32)> for Viewport {
    fn into(self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl From<(u32, u32)> for Viewport {
    fn from((width, height): (u32, u32)) -> Self {
        Self { width, height }
    }
}

// speedy2d please implement Bézier curve
#[inline(always)]
pub fn calc_quadratic_point(
    position: Vec2,
    length: f32,
    angle: f32,
    curve: f32,
    seg_c: usize,
    seg_i: usize,
) -> (f32, f32) {
    let x0 = position.x;
    let y0 = position.y;

    let x1 = x0 + length * angle.cos();
    let y1 = y0 + length * angle.sin();

    let cx = x0 + length * 0.5 * (angle + curve).cos();
    let cy = y0 + length * 0.5 * (angle + curve).sin();

    let i = seg_i as f32 / seg_c as f32;

    let ix = (1.0 - i) * (1.0 - i) * x0 + 2.0 * (1.0 - i) * i * cx + i * i * x1;
    let iy = (1.0 - i) * (1.0 - i) * y0 + 2.0 * (1.0 - i) * i * cy + i * i * y1;

    (ix, iy)
}
