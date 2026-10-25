use crate::grid::Grid;

pub struct Env {
    organics: Grid<bool>,
    tick: u64,
}
