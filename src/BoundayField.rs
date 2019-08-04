extern crate ndarray;
use ndarray::prelude::*;

use ndarray::iter::IndexedIter;

pub #[derive(Debug)]

enum BoundaryType {
    VALUE,
    PERIODIC,
}

enum Axis{
    x,
    y
}

trait PeriodicBoundary {
    fn p_at(&self, x: i32, y:u32) -> f32;
}

impl PeriodicBoundary for Array2{
    fn p_at(&self,x:i32, y:i32)->f32{
        let x_wrap = x;
        let y_wrap = y;
        if(x >= self.raw_dim[0]){
            let x_wrap =0;
        }
        if(x<0){
            let x_wrap= self.raw_dim[0]-1;
        }
        if(y >= self.raw_dim[1]){
            let y_wrap =0;
        }
        if(y<0){
            let y_wrap= self.raw_dim[1]-1;
        }
        return self[[x,y]];
    }
}
