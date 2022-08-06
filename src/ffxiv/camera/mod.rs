// Module

mod worldmatrix;
pub use worldmatrix::{WorldMatrix, WorldMatrixFn};

// Camera

#[struct_layout::explicit(size = 0x2b0, align = 4)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Camera {}

impl Camera {}