// Growth for any part of a tree
// Purpose of this is mostly animation
#[derive(Debug, Clone)]
pub struct Growth {
    // Unit of growth
    factor: u16,
    amount: u16,
    // Delay, works like amount debt
    dormancy: u16,
}

impl Growth {
    pub const MAX: u16 = u16::MAX;

    pub fn new(factor: u16, dormancy: u16) -> Self {
        Self {
            factor,
            amount: 0,
            dormancy,
        }
    }

    pub fn new_segmented(seg_c: u16, dormancy: u16) -> Self {
        Self::new(Self::MAX / seg_c, dormancy)
    }

    pub const fn get_segment_size(seg_c: u16) -> u16 {
        Self::MAX / seg_c
    }

    // If it is exist, pay the debt (dormany) and increase amount
    pub fn grow(&mut self, mut amount: u16) {
        let dormancy_interest = if amount > self.dormancy {
            self.dormancy
        } else {
            amount
        };

        amount -= dormancy_interest;

        self.dormancy -= dormancy_interest;
        self.amount = self.amount.saturating_add(amount);
    }

    pub fn grow_factors(&mut self, count: u16) {
        self.grow(self.factor * count);
    }

    pub fn grow_segments(&mut self, seg_c: u16, count: u16) {
        self.grow(Self::get_segment_size(seg_c) * count);
    }

    pub fn set_dormancy(&mut self, dormancy: u16) {
        self.dormancy = dormancy;
    }

    pub fn get_amount(&self) -> u16 {
        self.amount
    }

    pub fn get_amount_factor(&self) -> u16 {
        Self::MAX / self.factor
    }

    pub fn get_dormancy(&self) -> u16 {
        self.dormancy
    }

    pub fn get_grown_factor_count(&self) -> u16 {
        self.amount / self.factor
    }

    pub fn get_grown_segment_count(&self, seg_c: u16) -> u16 {
        self.amount / Self::get_segment_size(seg_c)
    }

    pub fn is_fully_grown(&self) -> bool {
        self.amount == Self::MAX
    }
}

pub trait Growable {
    fn get_growth<'a>(&'a self) -> &'a Growth;
    fn get_growth_mut<'a>(&'a mut self) -> &'a mut Growth;

    // Not realated to Growth.grow,
    // this meant be structural growing,
    // like growing children after the part 
    //
    // Each of the tree parts should implement this its own
    fn grow(&mut self, factors: u16) {
        self.get_growth_mut().grow_factors(factors);
    }

    // Same like "grow", this is also meant to be structural
    fn is_fully_grown(&self) -> bool {
        self.get_growth().is_fully_grown()
    }
}

impl<T: Growable + ?Sized> Growable for &mut T {
    fn get_growth<'a>(&'a self) -> &'a Growth {
        (**self).get_growth()
    }

    fn get_growth_mut<'a>(&'a mut self) -> &'a mut Growth {
        (**self).get_growth_mut()
    }
}
