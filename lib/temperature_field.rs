extern crate ndarray;
use ndarray::prelude::*;
use rand::Rng;
use rand::distributions::{Distribution,Uniform};
use ndarray::iter::IndexedIter;

use super::daisy_field::DaisyField;

pub struct TempField{
    pub field: Array2<f32>,
    width: usize,
    height: usize
}


impl TempField{
    pub fn new(width: usize, height: usize, inital_temp: f32) -> TempField{
        let mut field = Array::from_elem((width, height), inital_temp);
        
        for x in 0..width{
            for y in 0..height{
                
                field[[x,y]] = 295.5; /* 1.0*gaussian2D(x as f32, 
                                          y as f32,
                                          (width as f32)/2.0, 
                                          (height as f32)/2.0,
                                          10.0);*/
            }
        }

        TempField{
            width,
            height,
            field
        }
    }

    pub fn iterate(&self) -> IndexedIter<f32,ndarray::Dim<[usize; 2]>> {
        self.field.indexed_iter()
    }

    pub fn update(&mut self,daisy_field: &DaisyField){
        let mut div = Array2::<f32>::zeros(self.field.raw_dim());

        for ((x,y),val) in self.field.indexed_iter(){
            
            
            let C = 2500.0;
            let D  = 0.2;
            let DT = D*C;
            let sb = 5.67e-8;
            let time_step=0.02;
            let solar = 1.0 * 432.3 * 2.0;

            // let xm  = if x ==0  {0.0} else {self.field[[x-1,y]]};
            // let xp  = if x == self.width -1 {0.0} else{self.field[[x+1,y]]};
            // let ym  = if y ==0  {0.0} else {self.field[[x,y-1]]};
            // let yp  = if y == self.height -1 {0.0} else{self.field[[x,y+1]]};
            
            let xm  = if x ==0  {self.field[[self.width-1,y]]} else {self.field[[x-1,y]]};
            let xp  = if x == self.width -1 {self.field[[0,y]]} else{self.field[[x+1,y]]};
            let ym  = if y ==0  {self.field[[x,self.height-1]]} else {self.field[[x,y-1]]};
            let yp  = if y == self.height -1 {self.field[[x,0]]} else{self.field[[x,y+1]]};

            let dis =  DT*( xm + xp + ym + yp - 4.0*val) ;
            let rad = sb*val.powi(4);
            let ins = 
            if daisy_field.field[[x,y]]>=0.0{
                (1.0 - daisy_field.field[[x,y]])
            }
            else{
                0.5
            };

            // if x==50 && y== 50 {
            //     println!("Disapation {} radiance {} albido factor {} insolation {}", dis,rad,ins,ins*solar);
            // }
            div[[x,y]] = val +  (time_step/C)*(dis - rad + ins*solar);
        };
        self.field.assign(&div);
    }
}