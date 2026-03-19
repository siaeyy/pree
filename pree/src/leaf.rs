use speedy2d::dimen::Vec2;

use crate::growth::{Growable, Growth};
use crate::render::{ColorVariant, Render, RenderContext};

// Trait that holds methods for procedural leaf generation
pub trait LeafProducer {
    fn get_color_variant(&self) -> ColorVariant;
    fn get_growth(&self, radius: f32) -> Growth;
    fn get_radius(&self) -> f32;
}

// Trait that holds methods for procedural a bunch leaf generation
pub trait LeafBunchProducer {
    fn get_leaf_producer(&self) -> &impl LeafProducer;
    // How mant leaves are there
    fn get_count(&self) -> usize;
    // Angle of the leaf by the center
    fn get_scatter_angle(&self) -> f32;
    // How much far is the leaf from the center
    fn get_scatter_radius(&self) -> f32;
}

#[derive(Debug, Clone)]
pub struct Leaf {
    position: Vec2,
    radius: f32,
    color_variant: ColorVariant,
    growth: Growth,
}

impl Leaf {
    pub fn new(position: Vec2, radius: f32, color_variant: ColorVariant, growth: Growth) -> Self {
        Self {
            position,
            radius,
            color_variant,
            growth,
        }
    }

    pub fn new_with_producer(position: Vec2, producer: &impl LeafProducer) -> Self {
        let radius = producer.get_radius();
        let color_variant = producer.get_color_variant();
        let growth = producer.get_growth(radius);

        Self::new(position, radius, color_variant, growth)
    }

    pub fn new_bunch(base_position: Vec2, producer: &impl LeafBunchProducer) -> Vec<Self> {
        let leaf_producer = producer.get_leaf_producer();

        let base_x = base_position.x;
        let base_y = base_position.y;

        let leaf_count = producer.get_count();
        let mut leaves = Vec::with_capacity(leaf_count);

        for _ in 0..leaf_count {
            let scatter_angle = producer.get_scatter_angle();
            let scatter_radius = producer.get_scatter_radius();

            let x = base_x + scatter_angle.cos() * scatter_radius;
            let y = base_y + scatter_angle.sin() * scatter_radius;

            let position = Vec2::new(x, y);

            let leaf = Self::new_with_producer(position, leaf_producer);

            leaves.push(leaf);
        }

        leaves
    }
}

impl Growable for Leaf {
    fn get_growth<'a>(&'a self) -> &'a Growth {
        &self.growth
    }

    fn get_growth_mut<'a>(&'a mut self) -> &'a mut Growth {
        &mut self.growth
    }
}

impl Growable for [Leaf] {
    fn get_growth<'a>(&'a self) -> &'a Growth {
        unimplemented!()
    }

    fn get_growth_mut<'a>(&'a mut self) -> &'a mut Growth {
        unimplemented!()
    }

    fn grow(&mut self, factors: u16) {
        for leaf in self {
            leaf.grow(factors);
        }
    }

    fn is_fully_grown(&self) -> bool {
        self.iter().all(|branch| branch.is_fully_grown())
    }
}

impl Render for Leaf {
    fn render<'a>(&self, ctx: &RenderContext<'a>) {
        let growth_amount = self.growth.get_amount();
        let growth_threshold = Growth::MAX - self.radius as u16;

        if growth_amount <= growth_threshold {
            return;
        }

        let mut gfx = ctx.gfx.borrow_mut();

        let (start_color, end_color) = ctx.theme.get_leaf_color_bases();

        let color = self.color_variant.get_color(&start_color, &end_color);

        // For animation, so leaves do not spawn instantly
        let radius = if growth_amount == growth_threshold {
            self.radius
        } else {
            (growth_amount - growth_threshold) as f32
        };

        gfx.draw_circle(self.position, radius, color);
    }
}

impl Render for [Leaf] {
    fn render<'a>(&self, ctx: &RenderContext<'a>) {
        for leaf in self {
            leaf.render(ctx);
        }
    }
}
