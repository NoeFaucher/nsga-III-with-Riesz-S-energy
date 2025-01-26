use std::{f64::consts::PI, vec};

use rand::Rng;

use super::Problem;


#[derive(Debug, Clone)]
pub struct DTZL3 {
    dim_point: usize,
    dim_objective: usize,
}

impl DTZL3 {
    pub fn new(dim_point:usize,dim_objective: usize) -> Self {

        if dim_point <= dim_objective {
            panic!("For DTZL3: you must have dim_point > dim_objective ")
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
                            .fold(0.,|acc ,v| acc + (v-0.5).powf(2.) - ((20. * PI*(v-0.5)).cos()));
        return 100. * ((self.dim_point - self.dim_objective +1) as f64 + sum )
    }
}

impl Problem for DTZL3 {
    fn fitness(&self, coord: &Vec<f64>) -> Vec<f64> {
        let g = self.g_func(coord);

        let mut res = vec![1.+g;self.dim_objective];

        // compute f starting from f_{M-1} -> f_{M-2} -> ... -> f_0
        res = res.clone().into_iter().enumerate().map(|(i, v)| {
            let mut end = 1.;   

            for j in 0..i {
                end *= (coord[j]*PI/2.).cos();
            }

            if i < self.dim_objective -1 {
                end *= (coord[i]*PI/2.).sin();
            }            
            return v * end ;
        }).rev().collect();

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
    