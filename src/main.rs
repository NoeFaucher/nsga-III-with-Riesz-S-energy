use std::collections::LinkedList;

const DIM: usize = 5;

struct Nsga3 {
    parent_pop: LinkedList<[f64; DIM]>,
    ref_points: LinkedList<[f64; DIM]>,
    n: usize,
}

impl Nsga3 {
    fn new() -> Nsga3 {
        Nsga3 {
            parent_pop: LinkedList::new(),
            ref_points: LinkedList::new(),
            n: 100
        }
    }

    fn iterate(&mut self) {
        let mut saturated = LinkedList::new();
        let mut i = 0;
        let mut everyone: LinkedList<[f64; DIM]> = LinkedList::new();
        self.get_offspring(&mut everyone);
        everyone.append(&mut self.parent_pop);
        let mut fronts: Vec<LinkedList<[f64; DIM]>> = Vec::new();
        self.non_dominated_sort(&mut fronts, everyone);
        while saturated.len() < self.n && i < fronts.len() {
            saturated.append(&mut fronts[i]);
            i+=1;
        }
        if saturated.len() == self.n || i == fronts.len() { 
            self.parent_pop = saturated.clone();
            return;
        } else {
            for front in fronts.iter_mut() {
                self.parent_pop.append(front);
            }
            
            self.normalise();
            self.associate();
            self.niching();
        }
    }

    fn get_offspring(&self, everyone: &mut LinkedList<[f64; DIM]>) {
        todo!()
    }

    fn non_dominated_sort(&self, fronts: &mut Vec<LinkedList<[f64; DIM]>>, pop: LinkedList<[f64; DIM]>) {
        todo!()
    }

    fn normalise(&self) {
        todo!()
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
