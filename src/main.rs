use std::collections::LinkedList;
use nalgebra::{DMatrix};

const NB_OBJ: usize = 5;

struct Nsga3<T>
where T: Eq + PartialEq + Clone + Copy
{
    parent_pop: LinkedList<T>,
    ref_points: Vec<Vec<f64>>,
    pop_size: usize,
    ideal_point: Vec<f64>
}

trait NsgaCompatible {
    fn fitness(&self) -> Vec<f64>;
    fn set_norm_fitness(&mut self, new_value: f64, idx: usize);
}

impl<T> Nsga3<T>
where T: Eq + PartialEq + Clone + Copy + NsgaCompatible
{
    fn new() -> Nsga3<T> {
        Nsga3 {
            parent_pop: LinkedList::new(),
            ref_points: Vec::new(),
            pop_size: 100,
            ideal_point: Vec::new()
        }
    }

    fn iterate(&mut self) {
        let mut saturated: LinkedList<T> = LinkedList::new();
        let mut i = 0;
        let mut everyone: LinkedList<T> = LinkedList::new();
        self.get_offspring(&mut everyone);
        everyone.append(&mut self.parent_pop);
        let mut fronts: Vec<LinkedList<T>> = Vec::new();
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

    fn get_offspring(&self, offspring: &mut LinkedList<T>) {
        todo!() // crossover + mutation from self.parent_pop
    }

    fn non_dominated_sort(&self, fronts: &mut Vec<LinkedList<T>>, pop: LinkedList<T>) {
        todo!()
    }

    fn normalise(&mut self, saturated: &mut LinkedList<T>) {
        let mut extreme_points: Vec<T> = Vec::new();
        let mut min_abs: Vec<f64> = Vec::new();
        let mut nb_obj: usize = 0;
        for &ele in saturated.iter() {
            let ele_fitness: Vec<f64> = ele.fitness();
            nb_obj = ele_fitness.len();
            let mut w: Vec<f64> = Vec::new();
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
                    extreme_points.push(ele);
                } else if min_abs[j] > abs {
                    min_abs[j] = abs;
                    extreme_points[j] = ele;
                }
            }
        }

        // calculate plan equation
        let n = nb_obj;
        let mut a = DMatrix::<f64>::from_element(n, n, 0.0);
        let mut b = DMatrix::<f64>::from_element(n, 1, 0.0);

        for (i, point) in extreme_points.iter().enumerate() {
            for j in 0..n {
                a[(i, j)] = point.fitness()[j];
            }
            b[i] = 1.0;
        }

        let coefficients = a.lu().solve(&b).unwrap();
        let mut a_list: Vec<f64> = Vec::new();
        for j in 0..nb_obj {
            a_list.push(-1./coefficients[(1, j)]);
        }
        


        // Normalise the fitness of every point
        for &mut mut ele in saturated.iter_mut() {
            for j in 0..nb_obj {
                ele.set_norm_fitness((ele.fitness()[j] - self.ideal_point[j]) / (a_list[j] - self.ideal_point[j]), j);
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

fn main() {
    println!("Hello, world!");
    let mut nsga = Nsga3::new();
    nsga.iterate();
}
