extern crate ndarray;

use super::temperature_field::TempField;
use super::utils::cont_boundary;

use ndarray::prelude::*;
use rand::Rng;
use rand::distributions::{Distribution,Uniform, Normal};

pub struct DaisyField{
    pub field : Array2<f32>,
    width: usize,
    height: usize,
    death_rate: f32,
    mutation_rate: f32,
    pub births: u16,
    pub deaths: u16
}


impl DaisyField{
    
    pub fn new(width: usize, height: usize)->DaisyField{
        let mut rand =  rand::thread_rng();


        let mut field  = Array2::<f32>::zeros((width,height));
        for x in 0..width {
            for y in 0..height {
                let n = rand.gen::<f32>();
                field[[x,y]] = n;    
            }
        }

        DaisyField{
            width,
            height,
            field: field,
            death_rate : 0.01,
            mutation_rate: 0.01,
            deaths:0,
            births:0
        }
    }

    pub fn at(&self,x:usize, y:usize) -> f32{
        self.field[[x,y]]
    }

    

    pub fn update(&mut self, temp_field: &TempField){
        self.births = 0;
        self.deaths = 0;

        self.birth_phase(&temp_field.field);
        self.death_phase();
    }

    fn birth_function(&self,temp:f32)->f32{
        let normal = Normal::new(temp as f64 -295.5, 1.0);
        let v = normal.sample(&mut rand::thread_rng());
        return v as f32;
    }
    
    fn birth_phase(&mut self, temp: &Array2<f32>){
        let mut rand =  rand::thread_rng();
        
        let mut new_field = Array2::<f32>::zeros(self.field.raw_dim());
        for ((x,y), val) in self.field.indexed_iter(){
            if *val < 0.0 {
                //  let quad = randU.sample(&mut rand);
                //  let parent = match quad{
                //      0 => (cont_boundary(x,self.width, 1),y),
                //      1 => (cont_boundary(x,self.width, -1),y),
                //      2 => (x,cont_boundary(y,self.height,1)),
                //      3 => (x,cont_boundary(y,self.height,-1)),
                //      _ => panic!("Failed to match pattern")
                //  };

                 if rand.gen::<f32>() < self.birth_function(temp[[x,y]]){
                    let mut parents = Vec::new();
                     
                    for dx in -1..2 {
                        for dy in -1..2{
                            let v = self.field[[cont_boundary(x, self.width, dx),cont_boundary(y, self.height, dy)]];
                            if v > 0.0{
                                parents.push(v)
                            }
                        }
                    }

                    if parents.len() >0{
                        self.births += 1;
                        let randU  = Uniform::from(0..parents.len());
                        let parentSelect = randU.sample(&mut rand);
                        let parent_albedo  = parents[parentSelect];
                        let mut result = parent_albedo - (rand.gen::<f32>()-0.5)*2.0*self.mutation_rate;
                        new_field[[x,y]] = result;
                    }
                    else{
                        new_field[[x,y]] = -1.0;
                    }
                 }
                 else{
                    new_field[[x,y]] = -1.0; 
                 }
                    
            }
            else{
                new_field[[x,y]] = *val; 
            }
            
        }
        self.field.assign(&new_field);
    }

    fn death_phase(&mut self){
        let mut rand =  rand::thread_rng();
        let mut new_field = Array2::<f32>::zeros(self.field.raw_dim());

        for ((x,y), val) in self.field.indexed_iter(){
            if  *val > 0.0 && rand.gen::<f32>() < self.death_rate{
               new_field[[x,y]] = -1.0;      
               self.deaths += 1;
            }
            else{
                new_field[[x,y]] = *val;
            }
        }
        self.field.assign(&new_field);
    }
}
