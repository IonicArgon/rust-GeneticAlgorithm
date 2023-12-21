mod population;
use population::{Population, RastriginIndividual, Individual};

// other stuff
use requestty::{Question, Answers};

fn main() {
    // lets prompt for stuff first
    let q_pop_size = Question::int("pop_size")
        .message("Population size?")
        .default(100)
        .validate(|pop, _| {
            if pop < 1 {
                Err("Population size must be greater than 0".into())
            } else {
                Ok(())
            }
        })
        .build();

    let q_max_gen = Question::int("max_gen")
        .message("Max generations?")
        .default(100)
        .validate(|gen, _| {
            if gen < 2 {
                Err("Max generations must be at least 2".into())
            } else {
                Ok(())
            }
        })
        .build();

    let q_mutation_rate = Question::float("mutation_rate")
        .message("Mutation rate?")
        .default(0.01)
        .validate(|rate, _| {
            if rate < 0.0 || rate > 1.0 {
                Err("Mutation rate must be between 0 and 1".into())
            } else {
                Ok(())
            }
        })
        .build();

    let q_crossover_rate = Question::float("crossover_rate")
        .message("Crossover rate?")
        .default(0.9)
        .validate(|rate, _| {
            if rate < 0.0 || rate > 1.0 {
                Err("Crossover rate must be between 0 and 1".into())
            } else {
                Ok(())
            }
        })
        .build();

    let q_elitism = Question::select("elitism")
        .message("Elitism?")
        .choices(vec!["Yes", "No"])
        .default(0)
        .build();

    let q_stop_criteria = Question::float("stop_criteria")
        .message("Stop criteria?")
        .default(0.0)
        .validate(|criteria, _| {
            if criteria < 0.0 {
                Err("Stop criteria must be greater than or equal to 0".into())
            } else {
                Ok(())
            }
        })
        .build();

    let q_dim = Question::int("dim")
        .message("Dimension?")
        .default(2)
        .validate(|dim, _| {
            if dim < 1 {
                Err("Dimension must be greater than 0".into())
            } else {
                Ok(())
            }
        })
        .build();

    // todo: add question for choosing objective function

    let questions = vec![q_pop_size, q_max_gen, q_mutation_rate, q_crossover_rate, q_elitism, q_stop_criteria, q_dim];

    let answers = requestty::prompt(questions).unwrap();

    let pop_size = answers.get("pop_size").unwrap().as_int().unwrap() as usize;
    let max_gen = answers.get("max_gen").unwrap().as_int().unwrap() as usize;
    let mutation_rate = answers.get("mutation_rate").unwrap().as_float().unwrap();
    let crossover_rate = answers.get("crossover_rate").unwrap().as_float().unwrap();
    let elitism = answers.get("elitism").unwrap().as_list_item().unwrap().index;
    let elitism = elitism == 0;
    let stop_criteria = answers.get("stop_criteria").unwrap().as_float().unwrap();
    let dim = answers.get("dim").unwrap().as_int().unwrap() as usize;

    // print parameters
    println!("Selected parameters:");
    println!("{}", "-".repeat(20));
    println!("\t- Population size: {}", pop_size);
    println!("\t- Max generations: {}", max_gen);
    println!("\t- Mutation rate: {}", mutation_rate);
    println!("\t- Crossover rate: {}", crossover_rate);
    println!("\t- Elitism: {}", if elitism { "Yes" } else { "No" });

    // create population
    // ? default to Rastrigin function for now
    let mut pop = Population::new(pop_size, crossover_rate, mutation_rate, elitism, stop_criteria, dim, RastriginIndividual::new);

    // initialize population
    pop.initialize(-5.12, 5.12);


}
