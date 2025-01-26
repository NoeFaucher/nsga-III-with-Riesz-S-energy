use std::f64::consts::PI;

use rand::Rng;

use super::Problem;


#[derive(Debug, Clone)]
pub struct DTZL7 {
    dim_point: usize,
    dim_objective: usize,
}

impl DTZL7 {
    pub fn new(dim_point:usize,dim_objective: usize) -> Self {

        if dim_point <= dim_objective {
            panic!("For DTZL7: you must have dim_point > dim_objective ")
        }

        Self {
            dim_objective,
            dim_point,
        }
    }

    fn g_func(&self, coord: &Vec<f64>) -> f64 {
        let sum = coord.clone()
                            .into_iter()
                            .skip(self.dim_objective - 1 )
                            .fold(0.,|acc ,v| acc + v );
        return 1. + sum * 9. / (self.dim_point - self.dim_objective +1) as f64;
    }

    fn h_func(&self, coord: &Vec<f64>, g: f64) -> f64 {
        let sum = coord.clone()
                            .into_iter()
                            .take(self.dim_objective - 1)
                            .fold(0.,|acc ,v| acc + v * (1. + (3. * PI * v).sin()) / (1. + g) );

        return self.dim_objective as f64  - sum;
    }
    

}

impl Problem for DTZL7 {
    fn fitness(&self, coord: &Vec<f64>) -> Vec<f64> {
        let g = self.g_func(coord);

        // compute f starting from f_{M-1} -> f_{M-2} -> ... -> f_0
        let mut res: Vec<f64> = coord.clone().into_iter().take(self.dim_objective).collect();

        res[self.dim_objective - 1] = (1.+g) * self.h_func(coord, g);

        return res;
    }
    
    fn generate_random_coord(&self) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        return (0..self.dim_point).map(|_| rng.gen_range(0.0..=1.)).collect();
    }
    
    fn is_coord_allow(&self,coord: &Vec<f64>) -> bool {
        return coord.len() == self.dim_point && coord.iter().all(|&v| {v >= 0. && v <= 1.});
    }
}
    