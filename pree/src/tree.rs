use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    sync::Mutex,
};

use rand::seq::SliceRandom;
use speedy2d::{GLRenderer, color::Color, dimen::Vec2};

use crate::branch::{Branch, BranchProducer};
use crate::growth::{Growable, Growth};
use crate::render::{Render, RenderContext, Viewport};

// Trait that holds methods for procedural tree generation
pub trait TreeProducer {
    fn get_branch_producer(&self) -> &impl BranchProducer;
    // You got these
    fn get_root_branch_count(&self) -> usize;
    fn get_root_branch_angle(&self, angle: f32, branch_count: usize, branch_index: usize) -> f32;
}

#[derive(Debug, Clone)]
pub struct Tree {
    root: Vec<Branch>,
}

impl Tree {
    pub fn new(
        position: Vec2,
        length: f32,
        angle: f32,
        thickness: f32,
        depth: u8,
        producer: &impl TreeProducer,
    ) -> Self {
        let mut rng = rand::rng();

        let branch_count = producer.get_root_branch_count();
        let branch_producer = producer.get_branch_producer();

        let mut branch_indexes: Vec<usize> = (0..branch_count).collect();
        // Better animation
        branch_indexes.shuffle(&mut rng);

        let mut branchs = Vec::with_capacity(branch_count);

        for i in branch_indexes {
            let angle = producer.get_root_branch_angle(angle, branch_count, i);

            let branch =
                Branch::new_nested(position, length, angle, thickness, depth, branch_producer);

            branchs.push(branch);
        }

        Self { root: branchs }
    }
}

impl Growable for Tree {
    fn get_growth<'a>(&'a self) -> &'a Growth {
        unimplemented!()
    }

    fn get_growth_mut<'a>(&'a mut self) -> &'a mut Growth {
        unimplemented!()
    }

    fn grow(&mut self, factors: u16) {
        for branch in self.root.iter_mut() {
            branch.grow(factors);
        }
    }

    fn is_fully_grown(&self) -> bool {
        self.root.iter().all(|branch| branch.is_fully_grown())
    }
}

impl Render for Tree {
    fn render<'a>(&self, ctx: &RenderContext<'a>) {
        self.root.render(ctx);
    }
}

// In the future this apporach should be converted to trait,
// because with traits, it will be easier for implementing themes 
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeTheme {
    Pink,
    Green,
    Orange,
}

impl TreeTheme {
    // speedy2d please implement const from_int_rgb
    const fn color_from_int_rgb(rgb: (u8, u8, u8)) -> Color {
        Color::from_rgb(
            rgb.0 as f32 / 255.0,
            rgb.1 as f32 / 255.0,
            rgb.2 as f32 / 255.0,
        )
    }

    pub fn get_leaf_color_bases(&self) -> (Color, Color) {
        type RGB = (u8, u8, u8);

        let rgbs = match self {
            TreeTheme::Pink => {
                const START_COLOR: RGB = (220, 100, 120);
                const END_COLOR: RGB = (240, 150, 160);

                (START_COLOR, END_COLOR)
            }
            TreeTheme::Green => {
                const START_COLOR: RGB = (20, 120, 20);
                const END_COLOR: RGB = (40, 200, 20);

                (START_COLOR, END_COLOR)
            }
            TreeTheme::Orange => {
                const START_COLOR: RGB = (20, 120, 20);
                const END_COLOR: RGB = (40, 200, 20);

                (START_COLOR, END_COLOR)
            }
        };

        (
            Self::color_from_int_rgb(rgbs.0),
            Self::color_from_int_rgb(rgbs.1),
        )
    }

    pub fn get_branch_color(&self) -> Color {
        Self::color_from_int_rgb((74, 53, 37))
    }
}

// For JS string -> Rust enum
#[derive(Debug)]
pub struct TreeThemeErr;

impl std::fmt::Display for TreeThemeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid tree theme value!")
    }
}

impl std::error::Error for TreeThemeErr {}

// For JS string -> Rust enum
impl TryFrom<&str> for TreeTheme {
    type Error = TreeThemeErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "pink" => Ok(Self::Pink),
            "green" => Ok(Self::Green),
            "orange" => Ok(Self::Orange),
            _ => Err(TreeThemeErr),
        }
    }
}

pub struct TreeContext {
    tree: Tree,
    // For following and syncing the canvas but not implemented yet 
    viewport: Viewport,
    renderer: GLRenderer,
    // This is seperated from the tree because this is better, hmm
    pub theme: TreeTheme,
}

impl TreeContext {
    pub fn new(renderer: GLRenderer, viewport: Viewport, tree: Tree, theme: TreeTheme) -> Self {
        Self {
            viewport,
            tree,
            theme,
            renderer,
        }
    }

    pub fn grow(&mut self) {
        self.tree.grow(1);
    }

    pub fn render(&mut self) {
        let renderer = &mut self.renderer;
        let theme = &self.theme;

        renderer.draw_frame(|gfx| {
            let ctx = RenderContext {
                gfx: RefCell::new(gfx),
                theme,
            };

            self.tree.render(&ctx);
        });
    }

    pub fn get_tree(&self) -> &Tree {
        &self.tree
    }
}

pub type TreeContextHandle = usize;

// For guaranting that there will be no problems to Rust,
// well, in WASM, cant be, right?
pub struct SharedTreeContext(Mutex<TreeContext>);

unsafe impl Send for SharedTreeContext {}
unsafe impl Sync for SharedTreeContext {}

impl SharedTreeContext {
    pub fn new(value: TreeContext) -> Self {
        Self(Mutex::new(value))
    }
}

impl Deref for SharedTreeContext {
    type Target = Mutex<TreeContext>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SharedTreeContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<TreeContext> for SharedTreeContext {
    fn from(value: TreeContext) -> Self {
        Self::new(value)
    }
}
