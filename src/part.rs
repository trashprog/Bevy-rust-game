use std::time::Instant;
use bevy::prelude::*;


#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Part{
    pub part_tier : PartTier,
    pub size : Vec2,
    pub instant : Instant
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PartTier {
    Blue,
    Red,
    Green
}
