// A pretty procedure configration example

use std::{cell::RefCell, f32::consts::PI};

use pree::branch::BranchProducer;
use pree::growth::Growth;
use pree::leaf::{LeafBunchProducer, LeafProducer};
use pree::render::ColorVariant;
use pree::tree::TreeProducer;

use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;
use rand::{RngExt, rngs::ThreadRng};

#[derive(Default)]
pub struct ExampleLeafProducer {
    rng: RefCell<ThreadRng>,
}

impl ExampleLeafProducer {
    pub fn new() -> Self {
        Default::default()
    }
}

impl LeafProducer for ExampleLeafProducer {
    fn get_color_variant(&self) -> ColorVariant {
        ColorVariant::new_with_generator(&mut self.rng.borrow_mut())
    }

    fn get_growth(&self, radius: f32) -> Growth {
        let mut rng = self.rng.borrow_mut();

        let mut growth = Growth::new(1, 0);
        let delay = rng.random_range(radius as u16..10);

        growth.grow(Growth::MAX - delay);

        growth
    }

    fn get_radius(&self) -> f32 {
        let mut rng = self.rng.borrow_mut();
        4.0 + rng.random::<f32>() * 3.0
    }
}

#[derive(Default)]
pub struct ExampleLeafBunchProducer {
    leaf_procuder: ExampleLeafProducer,
}

impl ExampleLeafBunchProducer {
    pub fn new() -> Self {
        Default::default()
    }
}

impl LeafBunchProducer for ExampleLeafBunchProducer {
    fn get_leaf_producer(&self) -> &impl LeafProducer {
        &self.leaf_procuder
    }

    fn get_count(&self) -> usize {
        let mut rng = self.leaf_procuder.rng.borrow_mut();
        (6.0 + (rng.random::<f64>() * 6.0).floor()) as usize
    }

    fn get_scatter_angle(&self) -> f32 {
        let mut rng = self.leaf_procuder.rng.borrow_mut();
        rng.random::<f32>() * PI * 2.0
    }

    fn get_scatter_radius(&self) -> f32 {
        let mut rng = self.leaf_procuder.rng.borrow_mut();
        8.0 + rng.random::<f32>() * 10.0
    }
}

#[derive(Default)]
pub struct ExampleBranchProducer {
    rng: RefCell<ThreadRng>,
    leaf_bunch_producer: ExampleLeafBunchProducer,
}

impl ExampleBranchProducer {
    pub fn new() -> Self {
        Default::default()
    }
}

impl BranchProducer for ExampleBranchProducer {
    fn get_leaf_bunch_producer(&self) -> &impl LeafBunchProducer {
        &self.leaf_bunch_producer
    }

    fn get_curve(&self) -> f32 {
        let mut rng = self.rng.borrow_mut();
        (rng.random::<f32>() - 0.5) * 0.6
    }

    fn get_growth(&self, length: f32) -> Growth {
        Growth::new_segmented(self.get_segment_count(length), 0)
    }

    fn get_segment_count(&self, length: f32) -> u16 {
        const MIN_LENGTH: f32 = 50.0;
        const MAX_LENGTH: f32 = 120.0;

        const MIN_SEG_C: f32 = 10.0;
        const MAX_SEG_C: f32 = 30.0;

        let clamped_len = length.clamp(MIN_LENGTH, MAX_LENGTH);
        let ratio = (clamped_len - MIN_LENGTH) / (MAX_LENGTH - MIN_LENGTH);
        let segments = MIN_SEG_C + ratio * (MAX_SEG_C - MIN_SEG_C);

        segments.round() as u16
    }

    fn get_main_length(&self, length: f32) -> f32 {
        let mut rng = self.rng.borrow_mut();
        length * (0.8 + rng.random::<f32>() * 0.1)
    }

    fn get_main_angle(&self, angle: f32) -> f32 {
        let mut rng = self.rng.borrow_mut();
        angle + (rng.random::<f32>() - 0.5) * 0.6
    }

    fn get_main_thickness(&self, thickness: f32) -> f32 {
        thickness * 0.75
    }

    fn get_main_depth(&self, depth: u8) -> u8 {
        depth - 1
    }

    fn get_child_length(&self, length: f32) -> f32 {
        let mut rng = self.rng.borrow_mut();
        length * (0.8 + rng.random::<f32>() * 0.1) * 0.6
    }

    fn get_child_angle(&self, angle: f32) -> f32 {
        let mut rng = self.rng.borrow_mut();
        angle + (rng.random::<f32>() - 0.5) * 1.2
    }

    fn get_child_thickness(&self, thickness: f32) -> f32 {
        self.get_main_thickness(thickness) * 0.7
    }

    fn get_child_depth(&self, depth: u8) -> u8 {
        self.get_main_depth(depth) - 1
    }

    fn get_child_branch_count(&self) -> usize {
        let mut rng = self.rng.borrow_mut();
        (1.0 + (rng.random::<f32>() * 2.0).floor()) as usize
    }
}

#[derive(Default)]
pub struct ExampleTreeProducer {
    rng: RefCell<ThreadRng>,
    branch_producer: ExampleBranchProducer,
}

impl ExampleTreeProducer {
    pub fn new() -> Self {
        Default::default()
    }
}

impl TreeProducer for ExampleTreeProducer {
    fn get_branch_producer(&self) -> &impl BranchProducer {
        &self.branch_producer
    }

    fn get_root_branch_count(&self) -> usize {
        let mut rng = self.rng.borrow_mut();

        let branch_count_ratios = [(1, 2), (2, 2), (3, 1)];

        let weights: Vec<_> = branch_count_ratios.iter().map(|x| x.1).collect();
        let dist = WeightedIndex::new(&weights).unwrap();

        let index = dist.sample(&mut rng);

        let branch_count = branch_count_ratios[index].0;

        branch_count
    }

    fn get_root_branch_angle(&self, angle: f32, branch_count: usize, branch_index: usize) -> f32 {
        let mut rng = self.rng.borrow_mut();

        let spread = (branch_index as f32 - (branch_count as f32 - 1.0) / 2.0) * 0.4;
        let random_spread = spread + (rng.random::<f32>() * 0.2 - 0.1);

        angle + random_spread
    }
}
