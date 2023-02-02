use std::fmt::Debug;

use pso_rust::{
    self,
    basic::BasePsoNode,
    traits::{PsoAcc, PsoHandler},
};

#[test]
fn main_test() {
    #[derive(Debug, Clone)]
    struct Particle {
        x1: f64,
        x2: f64,
        performance: f64,
    }
    impl pso_rust::traits::PsoParticle<f64> for Particle {
        fn get_performance(&self) -> f64 {
            self.performance
        }
        fn init(position: Vec<f64>) -> Self {
            let mut p = Particle {
                x1: position[0],
                x2: position[1],
                performance: 0.0,
            };
            p.performance = -(p.x1.powi(2) + p.x2.powi(2));
            p
        }
        fn update_position(&mut self, position: &Vec<f64>) -> Option<Vec<f64>> {
            let mut flag = false;
            if position[0] > 10.0 || position[0] < -10.0 {
                self.x1 = 0.0;
                flag = true;
            } else if position[1] > 10.0 || position[1] < -10.0 {
                self.x2 = 0.0;
                flag = true;
            }
            if flag == true {
                self.performance = -(self.x1.powi(2) + self.x2.powi(2));
                Some(vec![self.x1, self.x2])
            } else {
                self.x1 = position[0];
                self.x2 = position[1];
                self.performance = -(self.x1.powi(2) + self.x2.powi(2));
                None
            }
        }
    }

    #[derive(Debug, Clone)]
    struct DATA<T>
    where
        T: Debug + Clone,
    {
        value: T,
    }

    impl<T> DATA<T>
    where
        T: Debug + Clone,
    {
        fn new(v: T) -> DATA<T> {
            DATA { value: v }
        }
    }

    impl<T> PsoAcc<BasePsoNode<Particle>, f64, T> for DATA<T>
    where
        T: Debug + Clone,
    {
        fn get_value(&self, generation: &usize, this_node: &BasePsoNode<Particle>) -> T {
            self.value.clone()
        }

        fn init_value(&self) -> T {
            self.value.clone()
        }
    }
    let spf = DATA::new(vec![(-2.0, 2.0), (-2.0, 2.0)]);
    let pf = DATA::new(vec![(-10.0, 10.0), (-10.0, 10.0)]);
    let inertia = DATA::new(0.5);
    let lfactor1 = DATA::new(2.0);
    let lfactor2 = DATA::new(2.0);

    let mut handler = pso_rust::basic::BasicPsoHandler::new(
        15, 2, spf, pf.value, inertia, lfactor1, lfactor2, -100.0,
    )
    .unwrap();
    handler.start(200);
    println!("Best Performance{}", handler.get_global_best_performance());
    let pos = handler.get_global_best_position();
    println!("x1: {}, x2: {}", pos[0], pos[1]);
}
