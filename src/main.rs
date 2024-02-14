mod complex;
mod genvec;
mod solver;
mod packet;
mod potential;
mod plotter;

use complex::*;
use genvec::*;
use solver::*;
use potential::*;
use potential::Shape::*;

fn main() {
    //1. Simulation Config
    let config = Config{
        dx: 0.25,
        dt: 0.001,
        boundary_condition_x: BoundaryCondition::Periodic,
        boundary_condition_y: BoundaryCondition::Periodic,
        num_steps: 5000,
        save_freq: 100,
    };

    //2. Potential setup
    let mut well = InfiniteWell::new();

    //Double slit:
    //well.add_shape(AABB{min: (1.0, -100.0), max: (2.0,  -3.0)})
    //    .add_shape(AABB{min: (1.0,   -2.0), max: (2.0,   2.0)})
    //    .add_shape(AABB{min: (1.0,    3.0), max: (2.0, 100.0)});

    //Grid:
    for i in 0..10 {
        for j in 0.. 10 {
            let x = 0.0 + 2.0 * (i as f64);
            let y = -10.0 + 2.0 * (j as f64);

            well.add_shape(Circle{pos: (x, y), rad: 0.5});
        }
    }

    let potential = |x: f64, y: f64| -> PotentialValue{
        well.get_value(x, y)
    };

    //3. Initial data
    const LATTICE_SIZE: usize = 100;

    let def = |i: usize, j: usize| -> Complex64 {

        let x = config.dx * ((i as f64) - 0.5 * (LATTICE_SIZE as f64));
        let y = config.dx * ((j as f64) - 0.5 * (LATTICE_SIZE as f64));

        packet::gaussian_packet(x+4.0, y, 2.0, (2.0, 0.0))
    };

    let init = generate_vec2d(LATTICE_SIZE, LATTICE_SIZE, def);

    //4. Visualization setup
    let draw_plots = |step: usize, data: &Vec<Vec<Complex64>>|{
        const IMG_PATH_PREFIX: &str = "images/";
        let fig_name = IMG_PATH_PREFIX.to_owned() + &step.to_string() + ".png";

        let config = plotter::PlotConfig{
            filepath: &fig_name,
            width: 512,
            height: 512,
            interpolation: plotter::Interpolation::Linear,
        };

        plotter::plot(&data, &config);
    };

    //5. Solver call
    evolve(&config, &init, potential, draw_plots);
}