use crate::problem::{Point, Problem};
use std::collections::LinkedList;

pub struct RefPointGenerator
{}

impl RefPointGenerator
{
    pub fn das_dennis(nb_partitions: i32, nb_dimensions: i32) -> Vec<Vec<f64>>
    // returns the coordinates in the objective space
    {
        let mut res: Vec<Vec<f64>> = Vec::new();
        RefPointGenerator::rec_das_dennis(&mut res, &Vec::new(), nb_partitions, nb_dimensions, nb_partitions);
        return res;
    }

    fn rec_das_dennis(acc_acc: &mut Vec<Vec<f64>>, acc: &Vec<f64>, nb_partitions: i32, nb_dimensions: i32, left_partitions: i32) {
        if acc.len() == nb_dimensions as usize {
            acc_acc.push(acc.clone());
            return;
        }

        if acc.len() == (nb_dimensions-1) as usize {
            let mut new_acc = acc.clone();
            new_acc.push((left_partitions as f64)/(nb_partitions as f64));
            RefPointGenerator::rec_das_dennis(acc_acc, &new_acc, nb_partitions, nb_dimensions, 0);
            return;
        }

        for i in 0..left_partitions+1 {
            let mut new_acc = acc.clone();
            new_acc.push((i as f64)/(nb_partitions as f64));
            RefPointGenerator::rec_das_dennis(acc_acc, &new_acc, nb_partitions, nb_dimensions, left_partitions-i);
        }
    }
}