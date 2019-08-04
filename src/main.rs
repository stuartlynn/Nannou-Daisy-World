extern crate rand;

extern crate daisyworld;
use nannou::prelude::*;
use daisyworld::{Simulation,DaisyField,TempField};

struct Model{
    simulation: Simulation,
    width: usize,
    height: usize,
}

impl Model{
    fn new(width: usize, height: usize) -> Model{
        Model{
            simulation: Simulation::new(width,height),
            width,
            height,
        }
    }
}

fn model(_app:&App)->Model{
    Model::new(100,100)
}

fn update(_app:&App, model: &mut Model, _update:Update){
   model.simulation.update(1);
}

fn view(app: &App, model: &Model, frame: &Frame) {

    app.main_window().set_inner_size_points(720.0*2.0, 720.0);
    let draw = app.draw();

    frame.clear(BLACK);
    let temp_field = &model.simulation.temperature;
    let daisy_field = &model.simulation.dasies;

    

    let x_box_size  = 720.0/(model.width as f32);
    let y_box_size  = 720.0/(model.height as f32);
    let mut max_val = 0.0;
    let mut min_val = 0.0;

    let mut total = 0.0;
    let tempStart = 198.0;
    let tempEnd   = 202.0;


    for i in 0..200{
        
        draw.rect()
            .w_h(x_box_size,y_box_size)
            .xy(vec2(- 720.0*2.0/2.0 - 20.0, (i as f32)*y_box_size - 720.0/2.0))
            .hsv((i as f32)/200.0, 1.0,0.5);
    }

    for ((x,y), val) in temp_field.iterate(){
        if *val > max_val{
            max_val = *val;   
        }

        if *val < min_val{
            min_val = *val;
        }

        total += val;
        draw.rect()
        .w_h(x_box_size, y_box_size)
        .xy(vec2((x as f32)*x_box_size - 720.0*2.0/2.0, (y as f32)*y_box_size - 720.0/2.0))
        .hsv((*val - 180.0) /(220.0 - 180.0), 1.0, 0.5);

        let daisy_val = daisy_field.field[[x,y]];
        if(daisy_val >= 0.0){
            draw.rect()
            .w_h(x_box_size, y_box_size)
            .xy(vec2((x as f32)*x_box_size , (y as f32)*y_box_size - 720.0/2.0))
            .hsl(0.0, 0.0, daisy_field.field[[x,y]]);
        }
        else{
            draw.rect()
            .w_h(x_box_size, y_box_size)
            .xy(vec2((x as f32)*x_box_size , (y as f32)*y_box_size - 720.0/2.0))
            .color(RED);
        }
    }

    println!("Tick {} min: {} max: {} total: {} births: {} deaths: {}",
            model.simulation.t,
            min_val, 
            max_val, 
            total, 
            daisy_field.births, 
            daisy_field.deaths 
            );

    draw.to_frame(app, &frame);
}

fn main() {
    let mut field = TempField::new(100,100, 5.0);
    let dasies  = DaisyField::new(100,100);

    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
    

    println!("Hello, world!");

}
