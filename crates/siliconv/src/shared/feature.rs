/// A bitfield of what features a given replay supports.
#[derive(Clone)]
pub struct ReplayFeature {
    value: u32,
}

#[rustfmt::skip]
impl ReplayFeature {
    pub const FPS_CHANGE: u32        = 1 << 0;

    pub const DEATH: u32             = 1 << 1;
    pub const RESTART: u32           = 1 << 2;
    pub const RESTART_FULL: u32      = 1 << 3;
    pub const SEED_UPON_RESTART: u32 = 1 << 4;

    pub const BUG_CHECKPOINT: u32    = 1 << 5;

    pub const FRAME_FIXES: u32       = 1 << 6;

    pub const SKIP_INPUT: u32        = 1 << 7;

    
    pub const fn check(&self, feature: u32) -> bool {
        return (self.value & feature) != 0
    }

    pub fn set(&mut self, feature: u32, value: bool) {
        self.value &= !feature; // clear the value for that feature
        if value {
            self.value |= feature;
        }
    }

    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}
