use std::{collections::LinkedList, ptr, rc::Rc};
use nalgebra::{DMatrix};
use rand::{seq::{IteratorRandom, SliceRandom}, Rng};

use crate::problem::{self, Point, Problem};

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

        self.get_offspring();
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

            self.normalise(&mut saturated);
            self.associate();
            self.niching();
        }
    }

    // crossover and mutation from:
    // Kalyanmoy Deb, Karthik Sindhya, and Tatsuya Okabe. Self-adaptive simulated binary crossover for real-parameter optimization. 
    // In Proceedings of the 9th Annual Conference on Genetic and Evolutionary Computation, GECCO ‘07, 1187–1194. New York, NY, USA, 2007. ACM.
    fn get_offspring(&self ) -> LinkedList<Point<T>> {
        // crossover + mutation from self.parent_pop

        let mut rng = rand::thread_rng();

        let nb_offsprings: usize = 100;
        
        // crossover (SBX)
        let mut offsprings: LinkedList<Point<T>> = LinkedList::new();
        let parent: Vec<Point<T>> = self.parent_pop.clone().into_iter().collect();
        let problem = parent[0].get_problem(); 
        let (lower_b,upper_b) = problem.borrow().get_bounds();

        let eta = 2.; // distribution index


        fn calc_betaq(beta: f64, eta: f64, u: f64) -> f64 {
            let alpha = 2. - beta.powf(-(eta + 1.)) ;
            let betaq;

            if u <= (1./alpha) {
                betaq = (u * alpha).powf(1. / (eta + 1.));
            } else {
                betaq = (1. / (2. - u * alpha)).powf(1. / (eta + 1.));
            }
            return  betaq;
        }


        for _ in 0..nb_offsprings/2 {
            let parents: Vec<&Point<T>> = parent.choose_multiple(&mut rng,2).collect();

            let coord_size = parents[0].coord.len();
            let cross: Vec<bool> = (0..coord_size).map(|_| rng.gen_bool(0.3)).collect();


            let mut cc1: Vec<f64> = parents[0].coord.clone();
            let mut cc2: Vec<f64> = parents[1].coord.clone();

            for i in 0..coord_size {
                let u: f64 = rng.gen_range(0.0..=1.);

                let y1 = parents[0].coord[i].min(parents[1].coord[i]);
                let y2 = parents[0].coord[i].max(parents[1].coord[i]);

                let delta: f64 = y2 - y1;

                if cross[i] {
                    let beta: f64 = 1. + (2. * (y1 - lower_b) / delta);
                    let betaq = calc_betaq(beta, eta, u);
                    cc1[i] = 0.5 * ( (1. + betaq) * parents[0].coord[i] +  (1. - betaq) * parents[1].coord[i] );


                    let beta: f64 = 1. + (2. * (upper_b - y2) / delta);
                    let betaq = calc_betaq(beta, eta, u);
                    cc2[i] = 0.5 * ( (1. - betaq) * parents[0].coord[i] +  (1. + betaq) * parents[1].coord[i] );
                }
            }
            
            let c1: Point<T> = Point::new_from(cc1, Rc::clone(&problem));
            let c2: Point<T> = Point::new_from(cc2, Rc::clone(&problem));
            
            offsprings.push_back(c1);
            offsprings.push_back(c2);
        }


        return offsprings;
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

// from paper: 
// Deb K, Pratap A, Agarwal S, Meyarivan T. A fast and elitist multiobjective genetic algorithm: NSGA-IIDeb K, Pratap A, Agarwal S, Meyarivan T. A fast and elitist multiobjective genetic algorithm: NSGA-II[J].
// Ieee Transactions on Evolutionary Computation. 2002,6(2):182-97
pub fn non_dominated_sort<T>(pop: LinkedList<Point<T>>) -> Vec<LinkedList<Point<T>>>
where T: Problem + Clone
{   
    let mut s: Vec<Vec<Point<T>>>= vec![vec![];pop.len()]; // list of point dominated by the point of the index in the pop
    let mut s_index: Vec<Vec<usize>>= vec![vec![];pop.len()]; // list of index of pointdominated by the point of the index in the pop
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