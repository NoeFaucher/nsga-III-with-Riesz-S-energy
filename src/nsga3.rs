use std::collections::LinkedList;
use nalgebra::{DMatrix};
use rand::random;

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
    pub fn new() -> Nsga3<T> {
        Nsga3 {
            parent_pop: LinkedList::new(),
            ref_points: Vec::new(),
            pop_size: 100,
            ideal_point: vec![0.; 5]
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

        fronts = non_dominated_sort(everyone);

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

            let missing = self.parent_pop.len() - self.pop_size;

            self.normalise(&mut saturated);
            let mut associated: Vec<usize> = Vec::new();
            let mut distances: Vec<f64> = Vec::new();
            self.associate(&mut associated, &mut distances, &saturated);

            let mut associated_last_front: Vec<usize> = Vec::new();
            let mut distances_last_front: Vec<f64> = Vec::new();
            self.associate_last_front(&mut associated_last_front, &mut distances_last_front, &fronts[i]);

            self.niching(&associated, &distances, &associated_last_front, &distances_last_front, missing, &fronts[i], &saturated);
        }
    }

    fn get_offspring(&self, offspring: &mut LinkedList<Point<T>>) {
        todo!() // crossover + mutation from self.parent_pop
    }

    fn non_dominated_sort(&self, fronts: &mut Vec<LinkedList<Point<T>>>, pop: LinkedList<Point<T>>) {
        todo!()
    }

    pub fn normalise(&mut self, saturated: &mut LinkedList<Point<T>>) {
        let mut rng = rand::thread_rng();
        let mut extreme_points: Vec<Point<T>> = Vec::new();
        let mut min_abs: Vec<f64> = Vec::new();
        let mut nb_obj: usize = 0;

        for ele in saturated.clone().iter() {
            println!("{:?}", ele.fitness);
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
            println!("{:?}", point.fitness);
            for j in 0..n {
                a[(i, j)] = point.fitness[j];
                if i == j {
                    a[(i, j)] += 0.000001;
                }
            }
            b[(i, 0)] = 1.0;
        }
        println!("{:?}", a.determinant());
        println!("{:?}", a);
        let coefficients = a.lu().solve(&b).unwrap();
        let mut a_list: Vec<f64> = vec![];

        for j in 0..nb_obj {
            a_list.push(b[(j, 0)]/(coefficients[(j, 0)]));
        }
        

        // Normalise the fitness of every point
        for ele in saturated.iter_mut() {
            for j in 0..nb_obj {
                ele.norm_fitness.push((ele.fitness[j] - self.ideal_point[j]) / (a_list[j] - self.ideal_point[j]));
            }
        }
    }

    fn associate(&self, associated: &mut Vec<usize>, distances: &mut Vec<f64>, saturated: &LinkedList<Point<T>>) {
        for (k, ref_point) in self.ref_points.iter().enumerate() {
            for (i, point) in saturated.iter().enumerate() {
                let mut scalar = 0.;
                let mut norm_sq = 0.;
                for j in 0..ref_point.len() {
                    scalar += ref_point[j] * point.fitness[j];
                    norm_sq += ref_point[j] * ref_point[j];
                }
                
                let mut dist_sq = 0.;
                for j in 0..ref_point.len() {
                    dist_sq += point.fitness[j] - scalar / norm_sq * ref_point[j] * point.fitness[j] - scalar / norm_sq * ref_point[j];
                }
                if associated.len() < saturated.len() {
                    associated.push(k);
                    distances.push(dist_sq);
                } else if dist_sq < distances[i] {
                    associated[i] = k;
                    distances[i] = dist_sq;
                }
            }
        }
    }

    fn associate_last_front(&self, associated_last_front: &mut Vec<usize>, distances_last_front: &mut Vec<f64>, last_front: &LinkedList<Point<T>>) {
        for (k, ref_point) in self.ref_points.iter().enumerate() {
            for (i, point) in last_front.iter().enumerate() {
                let mut scalar = 0.;
                let mut norm_sq = 0.;
                for j in 0..ref_point.len() {
                    scalar += ref_point[j] * point.fitness[j];
                    norm_sq += ref_point[j] * ref_point[j];
                }
                
                let mut dist_sq = 0.;
                for j in 0..ref_point.len() {
                    dist_sq += point.fitness[j] - scalar / norm_sq * ref_point[j] * point.fitness[j] - scalar / norm_sq * ref_point[j];
                }
                if associated_last_front.len() < last_front.len() {
                    associated_last_front.push(k);
                    distances_last_front.push(dist_sq);
                } else if dist_sq < distances_last_front[i] {
                    associated_last_front[i] = k;
                    distances_last_front[i] = dist_sq;
                }
            }
        }
    }

    fn niching(&mut self, associated: &Vec<usize>, distances: &Vec<f64>, associated_last_front: &Vec<usize>, distances_last_front: &Vec<f64>, missing: usize, last_front: &LinkedList<Point<T>>, saturated: &LinkedList<Point<T>>) {
        let mut k = 0;
        let mut niche_count: Vec<(usize, usize)> = vec![(0, 0); self.ref_points.len()];
        for i in associated {
            niche_count[*i] = match niche_count[*i] {
                | (_, y) => (*i, y+1)
            };
        }
        niche_count.sort_by(|(_, x), (_, y) | y.cmp(x)); // from biggest to smallest
        while k < missing {
            let (ref_point_idx, associated_count) = niche_count[niche_count.len()-1];
            let mut candidates: Vec<&Point<T>> = Vec::new();
            for (i, point) in last_front.iter().enumerate() {
                if associated_last_front[i] == ref_point_idx {
                    candidates.push(point);
                }
            }
            if candidates.len() == 0 {
                niche_count.pop();
                continue;
            }
            if associated_count > 0 {
                self.parent_pop.push_back(*candidates[(rand::random::<f64>() * candidates.len() as f64) as usize]);
            } else {

            }
            let last_idx = niche_count.len()-1;
            niche_count[last_idx] = match niche_count[niche_count.len()-1] {
                | (x, y) => (x, y+1)
            };
            k += 1;
        }
    }
}


pub fn non_dominated_sort<T>(pop: LinkedList<Point<T>>) -> Vec<LinkedList<Point<T>>>
where T: Problem + Clone
{
    let mut s: Vec<Vec<Point<T>>>= vec![vec![];pop.len()]; // list of point dominated by the point of the index in the pop
    let mut s_index: Vec<Vec<usize>>= vec![vec![];pop.len()]; // list of index of point dominated by the point of the index in the pop
    let mut f: Vec<LinkedList<Point<T>>> = vec![];  // list of fronts
    let mut f_index: Vec<LinkedList<usize>> = vec![]; // list of the index of point in the fronts
    
    let mut d_count = vec![0;pop.len()]; // counter of the number of time the point with index i in pop is dominated

    // pass through all point and compare them between each other
    for (i,p1) in pop.clone().into_iter().enumerate() {
        for (j,p2) in pop.clone().into_iter().enumerate() {
            if i == j {
                // do nothing
                continue;
            }
            // check domination
            match p1.domination(&p2) {
                crate::problem::Domination::Dominates => {
                    // keep all the point dominated by p1
                    s[i].push(p2);
                    s_index[i].push(j);
                },
                crate::problem::Domination::Equivalent => (),
                crate::problem::Domination::Dominated => {
                    // count the number of time p1 is dominated
                    d_count[i] += 1
                },
            }
        }
        
        // if the point i in pop (p1) is dominated by no one
        if d_count[i] == 0 {
            if f.len() == 0 {
                f.push(LinkedList::new());
                f_index.push(LinkedList::new());
            }
            // add the point to the first front
            f[0].push_back(p1.clone());
            f_index[0].push_back(i);
        }
    }
    
    let mut fi = 0;
    loop {
        let mut q: LinkedList<Point<T>> = LinkedList::new();
        let mut q_index: LinkedList<usize> = LinkedList::new();

        // go through each point of the previous front
        for i in f_index[fi].clone().into_iter() {
            // go through all the point dominated by a point in the previous point
            for (p2, j) in s[i].clone().into_iter().zip(s_index[i].clone().into_iter()) {
                // decrement the count once it reach 0 the point belong to the next front
                // it mean that no more point dominated it in all the point that are not yet in a front 
                d_count[j] -= 1; 
                if d_count[j] == 0 {
                    // add the point in the next front
                    q.push_back(p2);
                    q_index.push_back(j);
                }
            }
        }

        fi += 1;
        // if no point was added in q (next front) then we stop (all the points were taken in account)
        if q.is_empty() {
            break;
        }

        // add the next front (q) in the list of fronts
        f.push(q.clone());
        f_index.push(q_index.clone());
    }

    return f;
}