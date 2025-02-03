use nalgebra::min;
use rand::random;

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

    pub fn reduction(dimension: i32, nb_points: i32, nb_rand: i32) -> Vec<Vec<f64>> {
        let mut res: Vec<Vec<f64>> = Vec::new();
        let mut nb_part = 0;
        while RefPointGenerator::nb_point_das_dennis(dimension, nb_part) < nb_points {
            nb_part += 1;
        }
        nb_part -= 1;
        res = RefPointGenerator::das_dennis(nb_part, dimension);
        let original_dd_len = res.len();
        for i in 0..original_dd_len {
            let mut product = 1.;
            for coord in res[original_dd_len-i-1].clone() {
                product *= coord;
            }
            if product > 0. {
                res.swap_remove(i);
            }
        }

        //only border elements of das_dennis in res now
        let mut randoms: Vec<Vec<f64>> = Vec::new();
        for _ in 0..nb_rand {
            randoms.push(RefPointGenerator::random_point(dimension).clone());
        }

        // add to res the most isolated points
        let mut left_to_add = nb_points as usize - res.len();
        while left_to_add > 0 {
            let mut min_dist: Vec<f64> = Vec::new();
            for j in 0..randoms.len() {
                min_dist.push(1000000.);
                for chosen in res.iter() {
                    let mut dist = 0.;
                    for i in 0..dimension {
                        dist += (randoms[j][i] - chosen[i]) * (randoms[j][i] - chosen[i]); // how the f am i supposed to iterate through f-ing vectors!!!
                    }
                    if dist < min_dist[j] {
                        min_dist[j] = dist;
                    }
                }
            }
            // get the max min_dist
            let mut max = min_dist[0];
            let mut idx = 0;
            for i in 1..min_dist.len() {
                if min_dist[i] > max {
                    max = min_dist[i];
                    idx = i;
                }
            }

            res.push(randoms[idx].clone());
            left_to_add -= 1;
        }

        // selected every initial cluster centers, now k-mean



        return res;
    }

    pub fn random_point(dimension: i32) -> Vec<f64> { // random-ish, but I don't know how to uniformly distribute on an n-dimensional plane
        let mut res = vec![0.;dimension as usize];
        let mut remainder = 1.;
        let mut idx = 0;
        let mut a = rand::random::<f64>() / (dimension) as f64;
        while a < remainder {
            res[idx] += a;
            idx = (idx+1)%dimension as usize;
            remainder -= a;
            a = rand::random::<f64>() / (dimension) as f64;
        }
        res[idx] += remainder;
        return res;
    }

    fn nb_point_das_dennis(dimension: i32, partitions: i32) -> i32 {
        return RefPointGenerator::factorial(dimension + partitions-1) / (RefPointGenerator::factorial(partitions) * RefPointGenerator::factorial(dimension-1));
    }

    fn factorial(n: i32) -> i32 {
        let mut res = 1;
        for i in 1..n {
            res *= i;
        }
        return res;
    }

    pub fn riesz(dimension: i32, nb_points: i32, nb_rand: i32) -> Vec<Vec<f64>> {
        let res = RefPointGenerator::reduction(dimension, nb_points, nb_rand);
        
        return res;
    }
}