mod nsga3;
mod problem;

use std::{cell::RefCell, collections::LinkedList, rc::Rc};

// use nsga3::Nsga3;
use problem::{dtzl1::DTZL1, dtzl2::DTZL2, dtzl3::DTZL3, dtzl6::DTZL6, dtzl7::DTZL7, Point};
use nsga3::{Nsga3, non_dominated_sort};


fn main() {
    // let mut nsga = Nsga3::new();
    // nsga.iterate();

    // println!("------------------------");
    // let problem =Rc::new(RefCell::new(DTZL1::new(9,5))) ;
    // let p2: Point<DTZL1> = Point::new_from(vec![0.10,0.7, 0.7,0.2,0.1, 0.5,0.,0.9, 0.6], Rc::clone(&problem));
    // println!("{:?}",p2);
    // println!("------------------------");
    // let problem =Rc::new(RefCell::new(DTZL2::new(9,5))) ;
    // let p2: Point<DTZL2> = Point::new_from(vec![0.5;9], Rc::clone(&problem));
    // println!("{:?}",p2);
    // println!("------------------------");
    // let problem =Rc::new(RefCell::new(DTZL3::new(9,5))) ;
    // let p2: Point<DTZL3> = Point::new_from(vec![0.5;9], Rc::clone(&problem));
    // println!("{:?}",p2);
    // println!("------------------------");
    // let problem =Rc::new(RefCell::new(DTZL6::new(9,5))) ;
    // let p2: Point<DTZL6> = Point::new_from(vec![0.5;9], Rc::clone(&problem));
    // println!("{:?}",p2);
    // println!("------------------------");
    // let problem =Rc::new(RefCell::new(DTZL7::new(9,5))) ;
    // let p2: Point<DTZL7> = Point::new_from(vec![0.5;9], Rc::clone(&problem));
    // println!("{:?}",p2);

    
    println!("------------------------");
    // let p3: Point<DTZL1> = Point::new_from(vec![0.1,0.2,0.3], Rc::clone(&problem));
    // println!("{:?}",p3);


    // println!("{:?}",p2.fitness.into_iter().reduce(|acc, v| acc + v).unwrap());

    // let p1: Point<DTZL1> = Point::new_from(vec![0.;3], Rc::clone(&problem));
    // println!("{:?}",p1);
    // let p: Point<DTZL1> = Point::new_from(vec![1.;9], Rc::clone(&problem));
    // println!("{:?}",p);

    let problem =Rc::new(RefCell::new(DTZL1::new(3,2))) ;
    let points: LinkedList<Point<DTZL1>> = (0..10).map(|_| Point::new(Rc::clone(&problem))).collect();

    for p in points.iter() {
        println!("{:?}",p);
    }

    println!("\n\n");

    let fronts = non_dominated_sort(points);

    // println!("{:?}",fronts);
    let mut a = 0;
    for i in 0..fronts.len() {
        a += fronts[i].len();
        for p in fronts[i].iter(){
            println!("{i} {:?}",p);
        }
    }

    println!("{a}");


    // for p in points {

    //     println!("------------------------");
    // }



}
