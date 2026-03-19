use speedy2d::dimen::Vec2;

use crate::growth::{Growable, Growth};
use crate::leaf::{Leaf, LeafBunchProducer};
use crate::render::{Render, RenderContext, calc_quadratic_point};

// Trait that holds methods for procedural branch generation
pub trait BranchProducer {
    // For creating leaves
    fn get_leaf_bunch_producer(&self) -> &impl LeafBunchProducer;
    // Curve of a branch
    fn get_curve(&self) -> f32;
    // Initialized growth for a branch
    fn get_growth(&self, length: f32) -> Growth;
    // Segment count for a branch, mainly used by rendering
    fn get_segment_count(&self, length: f32) -> u16;
    // Length of the main branch child
    fn get_main_length(&self, length: f32) -> f32;
    // Angle of the main branch child
    fn get_main_angle(&self, angle: f32) -> f32;
    // Thickness of the main branch child
    fn get_main_thickness(&self, thickness: f32) -> f32;
    // Depth of the main branch child
    fn get_main_depth(&self, depth: u8) -> u8;

    // ===== Same but for other child branches ======
    fn get_child_length(&self, length: f32) -> f32;
    fn get_child_angle(&self, angle: f32) -> f32;
    fn get_child_thickness(&self, thickness: f32) -> f32;
    fn get_child_depth(&self, depth: u8) -> u8;
    // ==============================================

    // Does what its name says
    fn get_child_branch_count(&self) -> usize;
}

#[derive(Debug, Clone)]
pub enum BranchChildren {
    Leaves(Vec<Leaf>),
    Branchs(Vec<Branch>),

    // Unused, can be usable in future
    Mixed(Vec<Leaf>, Vec<Branch>),
}

impl Growable for BranchChildren {
    fn get_growth<'a>(&'a self) -> &'a Growth {
        unimplemented!()
    }

    fn get_growth_mut<'a>(&'a mut self) -> &'a mut Growth {
        unimplemented!()
    }

    fn grow(&mut self, factors: u16) {
        match self {
            Self::Leaves(leaves) => leaves.grow(factors),
            Self::Branchs(branchs) => branchs.grow(factors),
            Self::Mixed(leaves, branchs) => {
                branchs.grow(factors);
                leaves.grow(factors);
            }
        }
    }

    fn is_fully_grown(&self) -> bool {
        match self {
            Self::Leaves(leafs) => leafs.is_fully_grown(),
            Self::Branchs(branchs) => branchs.is_fully_grown(),
            Self::Mixed(leafs, branchs) => leafs.is_fully_grown() && branchs.is_fully_grown(),
        }
    }
}

impl Render for BranchChildren {
    fn render<'a>(&self, ctx: &RenderContext<'a>) {
        match self {
            Self::Leaves(leaves) => leaves.render(ctx),
            Self::Branchs(branchs) => branchs.render(ctx),
            Self::Mixed(leaves, branchs) => {
                branchs.render(ctx);
                leaves.render(ctx);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Branch {
    position: Vec2,
    length: f32,
    angle: f32,
    thickness: f32,
    curve: f32,
    children: BranchChildren,
    growth: Growth,
}

impl Branch {
    fn new_child_branch(
        position: Vec2,
        length: f32,
        angle: f32,
        thickness: f32,
        depth: u8,
        producer: &impl BranchProducer,
    ) -> Self {
        let mut child = Self::new_nested(position, length, angle, thickness, depth, producer);

        // Like animation delay
        child.growth.set_dormancy(Growth::MAX);

        child
    }

    fn new_child_branches(
        position: Vec2,
        length: f32,
        angle: f32,
        thickness: f32,
        depth: u8,
        producer: &impl BranchProducer,
    ) -> Vec<Branch> {
        let child_count = producer.get_child_branch_count();
        // Plus one for main child branch
        let mut branchs = Vec::with_capacity(child_count + 1);

        for _ in 0..child_count {
            let child_length = producer.get_child_length(length);
            let child_angle = producer.get_child_angle(angle);

            let branch = Self::new_child_branch(
                position,
                child_length,
                child_angle,
                thickness,
                depth,
                producer,
            );

            branchs.push(branch);
        }

        branchs
    }

    fn new_children(
        position: Vec2,
        length: f32,
        angle: f32,
        thickness: f32,
        depth: u8,
        producer: &impl BranchProducer,
    ) -> BranchChildren {
        let end_position = Vec2::new(
            position.x + length * angle.cos(),
            position.y + length * angle.sin(),
        );

        let main_thickness = producer.get_main_thickness(thickness);
        let child_thickness = producer.get_child_thickness(thickness);

        let main_depth = producer.get_main_depth(depth);
        let child_depth = producer.get_child_depth(depth);

        let create_main_branch = || {
            let main_length = producer.get_main_length(length);
            let main_angle = producer.get_main_angle(angle);

            Self::new_nested(
                end_position,
                main_length,
                main_angle,
                main_thickness,
                main_depth,
                producer,
            )
        };

        // Continue the branch over children
        if main_depth > 1 {
            let mut child_branchs = Self::new_child_branches(
                end_position,
                length,
                angle,
                child_thickness,
                child_depth,
                producer,
            );

            child_branchs.push(create_main_branch());
            BranchChildren::Branchs(child_branchs)

        // End of the branch, place leaves
        } else if main_depth < 1 {
            BranchChildren::Leaves(Leaf::new_bunch(
                end_position,
                producer.get_leaf_bunch_producer(),
            ))
        // Before end continue as one branch
        } else {
            BranchChildren::Branchs(vec![create_main_branch()])
        }
    }

    pub fn new(
        position: Vec2,
        length: f32,
        angle: f32,
        thickness: f32,
        curve: f32,
        children: BranchChildren,
        growth: Growth,
    ) -> Self {
        Self {
            position,
            length,
            angle,
            thickness,
            curve,
            children,
            growth,
        }
    }

    pub fn new_nested(
        position: Vec2,
        length: f32,
        angle: f32,
        thickness: f32,
        depth: u8,
        producer: &impl BranchProducer,
    ) -> Self {
        let curve = producer.get_curve();
        let growth = producer.get_growth(length);

        let children = if depth <= 0 {
            // Dont dig
            BranchChildren::Leaves(vec![])
        } else {
            Self::new_children(position, length, angle, thickness, depth, producer)
        };

        Self {
            position,
            length,
            angle,
            thickness,
            curve,
            children,
            growth,
        }
    }
}

impl Growable for Branch {
    fn get_growth<'a>(&'a self) -> &'a Growth {
        &self.growth
    }

    fn get_growth_mut<'a>(&'a mut self) -> &'a mut Growth {
        &mut self.growth
    }

    fn grow(&mut self, factors: u16) {
        let growth = self.get_growth_mut();

        if growth.is_fully_grown() {
            self.children.grow(factors);
        } else {
            growth.grow_factors(factors);
        }
    }

    fn is_fully_grown(&self) -> bool {
        self.children.is_fully_grown()
    }
}

impl Growable for [Branch] {
    fn get_growth<'a>(&'a self) -> &'a Growth {
        unimplemented!()
    }

    fn get_growth_mut<'a>(&'a mut self) -> &'a mut Growth {
        unimplemented!()
    }

    fn grow(&mut self, factors: u16) {
        for branch in self {
            branch.grow(factors);
        }
    }

    fn is_fully_grown(&self) -> bool {
        self.iter().all(|branch| branch.is_fully_grown())
    }
}

impl Render for Branch {
    fn render<'a>(&self, ctx: &RenderContext<'a>) {
        let x = self.position.x;
        let y = self.position.y;

        let color = ctx.theme.get_branch_color();
        let growth = &self.growth;

        let mut start = (x, y);

        let mut gfx = ctx.gfx.borrow_mut();

        // Since the growth factor is set to segment size,
        // amount factor means total segment count
        let seg_c = growth.get_amount_factor();
        let grown_seg_c = growth.get_grown_segment_count(seg_c);

        let calc_quadratic_point = |seg_i: usize| {
            calc_quadratic_point(
                self.position,
                self.length,
                self.angle,
                self.curve,
                seg_c as usize,
                seg_i,
            )
        };

        for i in 0..grown_seg_c + 1 {
            let end = calc_quadratic_point(i.into());
            gfx.draw_line(start, end, self.thickness, color);
            start = end;
        }

        // Smooth finish
        if grown_seg_c > 0 {
            gfx.draw_circle(start, self.thickness / 2.0, color);
        }

        drop(gfx);

        self.children.render(ctx);
    }
}

impl Render for [Branch] {
    fn render<'a>(&self, ctx: &RenderContext<'a>) {
        for branch in self {
            branch.render(ctx);
        }
    }
}
