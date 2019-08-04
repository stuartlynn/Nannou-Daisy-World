use super::daisy_field::DaisyField;
use super::temperature_field::TempField;
// mod TemperatureField;


pub struct Simulation{
    pub dasies: DaisyField,
    pub temperature: TempField,
    pub solar_rad: f32,
    pub t: u32
}


impl Simulation{
    pub fn new(width: usize, height:usize)->Simulation{
        let dasies = DaisyField::new(width,height);
        let temperature = TempField::new(width,height,255.0);

        Simulation{
            dasies: dasies,
            temperature: temperature,
            solar_rad : 2.0,
            t:0
        }
    }

    pub fn update(&mut self, steps: usize){
        for _ in 0..200{
            self.dasies.update(&self.temperature);
            self.temperature.update(&self.dasies);
        }
        self.t += 1;
    }

}
