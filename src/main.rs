mod population;
use population::{Population, RastriginIndividual, Individual};

fn main() {
    let mut population = Population::new(10, 0.8, 0.1, 0.001, 5, RastriginIndividual::new);

    population.initialize(-5.12, 5.12);

    let pop_chars = population.characteristics();
    println!("{}", pop_chars);

    // print our Xs
    for i in 0..population.size {
        println!("{:?}", population.individuals[i].get_x());
    }
}
