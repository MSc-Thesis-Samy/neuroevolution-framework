use neuroevolution::vneuron::*;
use neuroevolution::discrete_vnetwork::*;
use neuroevolution::benchmarks::*;
use neuroevolution::neuroevolution_algorithm::*;
use neuroevolution::constants::*;

fn main() {
    let half = Benchmark::new(Problem::Half);
    let quarter = Benchmark::new(Problem::Quarter);
    let two_quarters = Benchmark::new(Problem::TwoQuarters);

    let vneuron = VNeuron::new(2);
    let mut alg = Algorithm::ContinuousBNA(vneuron);
    alg.optimize(&half, N_ITERATIONS);
    println!("half fitness: {:.2}", half.evaluate(&alg));
    println!("Half: {}", alg);
    alg.optimize(&quarter, N_ITERATIONS);
    println!("quarter fitness: {:.2}", quarter.evaluate(&alg));
    println!("Half: {}", alg);
    alg.optimize(&two_quarters, N_ITERATIONS);
    println!("two_quarters fitness: {:.2}", two_quarters.evaluate(&alg));
    println!("Half: {}", alg);

    let dvnetwork = DiscreteVNetwork::new(RESOLUTION, 1, 2);
    let mut alg = Algorithm::DiscreteBNA(dvnetwork);
    alg.optimize(&half, N_ITERATIONS);
    println!("half fitness: {:.2}", half.evaluate(&alg));
    println!("Half: {}", alg);
    alg.optimize(&quarter, N_ITERATIONS);
    println!("quarter fitness: {:.2}", quarter.evaluate(&alg));
    println!("Half: {}", alg);
    alg.optimize(&two_quarters, N_ITERATIONS);
    println!("two_quarters fitness: {:.2}", two_quarters.evaluate(&alg));
    println!("Half: {}", alg);
}
