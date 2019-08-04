extern crate ndarray;
use ndarray::prelude::*;

enum BoundaryCondition{
    PERIODIC,
    FIXED
}

pub struct BoundaryField{
    pub field: Array2<f32>,
    pub btype: BoundaryCondition
}

impl BoundaryField{

    pub fn new(width: usize, height: usize, boundary_type: BoundaryCondition) -> Self{
        Self{
            field :Array2::<f32>::zeros((width,height)),
            btype:boundary_type
        }
    }
}