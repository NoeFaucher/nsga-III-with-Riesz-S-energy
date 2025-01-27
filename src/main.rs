mod nsga3;
mod problem;

use std::{cell::RefCell, collections::LinkedList, rc::Rc, vec};

use nsga3::Nsga3;
// use nsga3::Nsga3;
use problem::{dtzl1::DTZL1, dtzl2::DTZL2, dtzl3::DTZL3, dtzl6::DTZL6, dtzl7::DTZL7, Point};



fn main() {
    // let mut nsga = Nsga3::new();
    // nsga.iterate();

    // println!("------------------------");
    // let problem =Rc::new(RefCell::new(DTZL1::new(9,5))) ;
    
    
    // let p2: Point<DTZL1> = Point::new_from(vec![0.5;9], Rc::clone(&problem));
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

    // // let points: Vec<Point<DTZL1>> = (0..1).map(|_| Point::new(Rc::clone(&problem))).collect();
    
    // println!("------------------------");
    // // let p3: Point<DTZL1> = Point::new_from(vec![0.1,0.2,0.3], Rc::clone(&problem));
    // // println!("{:?}",p3);


    // println!("{:?}",p2.fitness.into_iter().reduce(|acc, v| acc + v).unwrap());

    // // let p1: Point<DTZL1> = Point::new_from(vec![0.;3], Rc::clone(&problem));
    // // println!("{:?}",p1);
    // // let p: Point<DTZL1> = Point::new_from(vec![1.;9], Rc::clone(&problem));
    // // println!("{:?}",p);


    // for p in points {

    //     println!("------------------------");
    // }

    let problem =Rc::new(RefCell::new(DTZL1::new(9,5))) ;

    let mut nsga3: Nsga3<DTZL1> = Nsga3::new();
    let mut list: LinkedList<Point<DTZL1>> = LinkedList::new();
    for i in 0..500 {
        list.push_back(Point::new(Rc::clone(&problem)));
    }
    nsga3.normalise(&mut list);
    for ele in  list {
        let mut sum: f64 = 0.;
        for i in 0..5 {
            sum += ele.norm_fitness[i];
        }
        println!("{:?}", sum);
    }

}
