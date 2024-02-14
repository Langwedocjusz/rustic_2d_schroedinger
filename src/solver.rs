//Finite difference solver of the 2d, time dependent Schroedinger equation
//Second order in space, first order in time

use std::time::Instant;
use crate::complex::*;
use crate::genvec::*;
use crate::potential::PotentialValue;
use crate::potential::PotentialValue::*;

pub enum BoundaryCondition{
    Periodic,
    Dirichlet(Complex64, Complex64)
}

pub struct Config{
    pub dx: f64,
    pub dt: f64,                                                                                                                                                                          
    pub boundary_condition_x: BoundaryCondition,
    pub boundary_condition_y: BoundaryCondition,
    pub num_steps: usize,
    pub save_freq: usize,
}

pub fn evolve<F, G>(config: &Config, init: &Vec<Vec<Complex64>>, potential_fn: G, mut save_callback: F) where
    F: FnMut(usize, &Vec<Vec<Complex64>>),
    G: Fn(f64, f64) -> PotentialValue {

    let now = Instant::now();

    let lattice_width = init.len();
    let lattice_height = init[0].len();

    let mut front = init.clone();
    let mut back  = vec![vec![Complex64{x: 0.0, y: 0.0}; lattice_height]; lattice_width];

    let potential = generate_vec2d(lattice_width, lattice_height, |i: usize, j: usize|{
        let x = config.dx * ((i as f64) - 0.5 * (lattice_width as f64));
        let y = config.dx * ((j as f64) - 0.5 * (lattice_height as f64));

        potential_fn(x, y)
    });

    for i in 0..config.num_steps {
        if i%2 == 0 {
            next_step(&front, &mut back, &potential, &config);
        } else {
            next_step(&back, &mut front, &potential, &config);
        }

        if config.save_freq != 0 && i % config.save_freq == 0 {
            let call_number = i/config.save_freq;

            if i%2 == 0 {
                save_callback(call_number, &back);
            } else {
                save_callback(call_number, &front);
            }
        }
    }

    println!("Simulation ({} steps) took {} [s].", 
        config.num_steps, 
        (now.elapsed().as_millis() as f64)/1000.0
    );
}

fn next_step(prev: &Vec<Vec<Complex64>>, curr: &mut Vec<Vec<Complex64>>, 
    potential: &Vec<Vec<PotentialValue>>, config: &Config) {

    for i in 0..curr.len() {
        for j in 0..curr[0].len() {

            match potential[i][j] {
                Infinite => {curr[i][j] = Complex64{x: 0.0, y: 0.0};},

                Finite(potential_value) => {

                    let left = if i!= 0 {prev[i-1][j]} else {
                        match config.boundary_condition_x {
                            BoundaryCondition::Periodic => prev[prev.len()-1][j],
                            BoundaryCondition::Dirichlet(left, _) => left,
                        }
                    };
        
                    let right = if i != prev.len() - 1 {prev[i+1][j]} else {
                        match config.boundary_condition_x {
                            BoundaryCondition::Periodic => prev[0][j],
                            BoundaryCondition::Dirichlet(_, right) => right,
                        }
                    };
        
                    let up = if j != prev[0].len() - 1 {prev[i][j+1]} else {
                        match config.boundary_condition_y {
                            BoundaryCondition::Periodic => prev[i][0],
                            BoundaryCondition::Dirichlet(_, top) => top,
                        }
                    };
        
                    let down = if j != 0 {prev[i][j-1]} else {
                        match config.boundary_condition_y {
                            BoundaryCondition::Periodic => prev[i][prev[0].len() - 1],
                            BoundaryCondition::Dirichlet(bottom, _) => bottom,
                        }
                    };
        
                    let psi = prev[i][j];
                    let ddx = (left + right - 2.0*psi)/(config.dx*config.dx);
                    let ddy = (up   + down  - 2.0*psi)/(config.dx*config.dx);
        
                    //hbar = 1.0, mass = 1.0
                    let kinetic = -0.5*(ddx + ddy);
        
                    let dpsi = - Complex64::i() * (kinetic + potential_value);
        
                    curr[i][j] = psi + config.dt * dpsi;
                },
            }
        }
    }
}