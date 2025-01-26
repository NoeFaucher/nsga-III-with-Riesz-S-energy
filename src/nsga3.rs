use std::collections::LinkedList;
use nalgebra::{DMatrix};

use crate::problem::{Point, Problem};

const NB_OBJ: usize = 5;

pub struct Nsga3<T>
where T: Problem + Clone
{
    parent_pop: LinkedList<Point<T>>,
    ref_points: Vec<Vec<f64>>,
    pop_size: usize,
    ideal_point: Vec<f64>
}

impl<T> Nsga3<T>
where T: Problem + Clone
{
    fn new() -> Nsga3<T> {
        Nsga3 {
            parent_pop: LinkedList::new(),
            ref_points: Vec::new(),
            pop_size: 100,
            ideal_point: Vec::new()
        }
    }

    /* ALgorithm 1 in NSGA-III paper
    */
    fn iterate(&mut self) {

        let mut saturated: LinkedList<Point<T>> = LinkedList::new();
        let mut i = 0;
        let mut everyone: LinkedList<Point<T>> = LinkedList::new();
        let mut fronts: Vec<LinkedList<Point<T>>> = Vec::new();

        self.get_offspring(&mut everyone);
        everyone.append(&mut self.parent_pop);

        self.non_dominated_sort(&mut fronts, everyone);

        while saturated.len() < self.pop_size && i < fronts.len() {
            saturated.append(&mut fronts[i]);
            i+=1;
        }

        if saturated.len() == self.pop_size || i == fronts.len() { 
            self.parent_pop = saturated.clone();
            return;
        } else {
            for j in 0..i {
                self.parent_pop.append(&mut fronts[j]);
            }

            self.normalise(&mut saturated);
            self.associate();
            self.niching();
        }
    }

    fn get_offspring(&self, offspring: &mut LinkedList<Point<T>>) {
        todo!() // crossover + mutation from self.parent_pop
    }

    fn non_dominated_sort(&self, fronts: &mut Vec<LinkedList<Point<T>>>, pop: LinkedList<Point<T>>) {
        todo!()
    }

    fn normalise(&mut self, saturated: &mut LinkedList<Point<T>>) {
        let mut extreme_points: Vec<Point<T>> = Vec::new();
        let mut min_abs: Vec<f64> = Vec::new();
        let mut nb_obj: usize = 0;

        for ele in saturated.clone().iter() {
            let ele_fitness: Vec<f64> = ele.fitness.clone();
            let mut w: Vec<f64> = vec![];

            nb_obj = ele_fitness.len();
            
            // initialise w
            for _ in 0..nb_obj {
                w.push(0.000001);
            }

            for j in 0..nb_obj {
                // compute ideal point
                if ele_fitness[j] < self.ideal_point[j] {
                    self.ideal_point[j] = ele_fitness[j];
                }
                
                // compute abs
                w[j] = 1.;
                let mut abs = ele_fitness[0] / w[0];
                for i in 1..nb_obj {
                    if ele_fitness[i] / w[i] > abs {
                        abs = ele_fitness[i] / w[i]
                    }
                }
                w[j] = 0.000001;

                // check for min abs
                if min_abs.len() <= j {
                    min_abs.push(abs);
                    extreme_points.push(ele.clone());
                } else if min_abs[j] > abs {
                    min_abs[j] = abs;
                    extreme_points[j] = ele.clone();
                }
            }
        }

        // calculate plan equation
        let n = nb_obj;
        let mut a = DMatrix::<f64>::from_element(n, n, 0.0);
        let mut b = DMatrix::<f64>::from_element(n, 1, 0.0);

        for (i, point) in extreme_points.iter().enumerate() {
            for j in 0..n {
                a[(i, j)] = point.fitness[j];
            }
            b[i] = 1.0;
        }

        let coefficients = a.lu().solve(&b).unwrap();
        let mut a_list: Vec<f64> = vec![];

        for j in 0..nb_obj {
            a_list.push(-1./coefficients[(1, j)]);
        }
        

        // Normalise the fitness of every point
        for ele in saturated.iter_mut() {
            for j in 0..nb_obj {
                // ele.set_norm_fitness((ele.fitness[j] - self.ideal_point[j]) / (a_list[j] - self.ideal_point[j]), j);

                // on a besoin de la fitness normale apres ou on peut consider que la fitness normaliser c'est la nouvelle fitness ??
            }
        }
    }

    fn associate(&self) {
        todo!()
    }

    fn niching(&self) {
        todo!()
    }
}