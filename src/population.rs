use rand::Rng;

// individual stuff
// todo: move this to a separate file later
pub trait Individual {
    fn new(dim: usize) -> Self where Self: Sized;
    fn initialize(&mut self, upper_bound: f64, lower_bound: f64);
    fn fitness(&self) -> f64;
    fn objective_function(&self) -> String;
    fn get_dim(&self) -> usize;
    fn get_x(&self) -> &Vec<f64>;
}

pub struct RastriginIndividual {
    pub dim: usize,
    pub x: Vec<f64>,
}

impl Individual for RastriginIndividual {
    fn new(dim: usize) -> Self {
        RastriginIndividual {
            dim,
            x: vec![0.0; dim],
        }
    }

    fn initialize(&mut self, lower_bound: f64, upper_bound: f64) {
        let mut rng = rand::thread_rng();
        for i in 0..self.dim {
            self.x[i] = rng.gen_range(lower_bound..upper_bound);
        }
    }

    fn fitness(&self) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.dim {
            sum += self.x[i].powf(2.0) - 10.0 * (2.0 * std::f64::consts::PI * self.x[i]).cos();
        }
        10.0 * self.dim as f64 + sum
    }

    fn objective_function(&self) -> String {
        String::from("Rastrigin")
    }

    fn get_dim(&self) -> usize {
        self.dim
    }

    fn get_x(&self) -> &Vec<f64> {
        &self.x
    }
}


// population stuff
pub struct Population {
    pub size: usize,
    pub crossover_rate: f64,
    pub mutation_rate: f64,
    pub elitism: bool,
    pub stop_criteria: f64,
    pub individuals: Vec<Box<dyn Individual>>,
}

impl Population {
    pub fn new<T: Individual + 'static>(size: usize, crossover_rate: f64, mutation_rate: f64, elitism: bool, stop_criteria: f64, dim: usize, individual_creator: fn(usize) -> T) -> Self {
        let mut individuals = Vec::new();
        for _ in 0..size {
            individuals.push(Box::new(individual_creator(dim)) as Box<dyn Individual>);
        }
        Population {
            size,
            crossover_rate,
            mutation_rate,
            elitism,
            stop_criteria,
            individuals,
        }
    }

    pub fn initialize(&mut self, lower_bound: f64, upper_bound: f64) {
        for individual in &mut self.individuals {
            individual.initialize(lower_bound, upper_bound);
        }
    }
}
