mod nsga3;

use nsga3::Nsga3;




fn main() {
    let mut nsga = Nsga3::new();
    nsga.iterate();
}
