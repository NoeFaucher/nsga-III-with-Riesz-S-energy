use std::{cell::RefCell, rc::Rc};

pub(crate) mod dtzl1;
pub(crate) mod dtzl2;
pub(crate) mod dtzl3;
pub(crate) mod dtzl6;
pub(crate) mod dtzl7;


pub trait Problem {
    fn fitness(&self, coord: &Vec<f64>) -> Vec<f64>;

    fn generate_random_coord(&self) -> Vec<f64>;

    fn is_coord_allow(&self,coord: &Vec<f64>) -> bool;
}

#[derive(Debug)]
pub struct Point<T> 
where T: Problem
{
    pub coord: Vec<f64>,
    pub fitness: Vec<f64>,

    problem: Rc<RefCell<T>>
}


impl<T> Point<T>
where T: Problem
{
    pub fn new(problem: Rc<RefCell<T>>) -> Self {
        let coord = problem.borrow().generate_random_coord();
        let fitness: Vec<f64>=  problem.borrow().fitness(&coord);

        Self {
            coord,
            fitness,
            problem,
        }
    }

    pub fn new_from(coord: Vec<f64>, problem: Rc<RefCell<T>>) -> Self {

        // to disable when running
        if !problem.borrow().is_coord_allow(&coord) {
            panic!("Point::new_from : Coord of point not allow may be out of bounds of the dimension may mismatch");
        }

        let fitness=  problem.borrow().fitness(&coord);

        Self {
            coord,
            fitness,
            problem,
        }

    }
}
