# PSO-Rust

<p style="text-align: center;">
<a href="https://space.bilibili.com/526159315"><img alt="author" src="https://img.shields.io/badge/Author-%E7%BB%9F%E5%90%88%E9%83%A825000mm%E8%A3%85%E7%94%B2%E9%99%84%E7%94%B2(526159315)-blue"></a>
<img alt="GitHub" src="https://img.shields.io/github/license/Embers-of-the-Fire/PSO-Rust?color=yellow">
<img alt="GitHub top language" src="https://img.shields.io/github/languages/top/Embers-of-the-Fire/PSO-Rust?color=green">
<a href="https://crates.io/crates/pso_rust"><img alt="crates" src="https://img.shields.io/badge/crates.io-pso_rust-d7611b"/></a>
</p>

```bash
cargo install pso_rust
```

## What's this?

This is a rust library focus on the PSO method, or Particle Swarm Optimizer.

## What's new?

Unlike the existing packages which can be found on crates.io, this library does not care about what have been inputted. Through various traits, you can get this involved any kind of data structures by implementing those traits.

Although implementing those traits might seem like much more complicated, but the capabilities supported by those definitely worth it.

## How to use?

After all this is a new lib, so it's now only the most basic case of PSO is supported -- the original pso method. And other variations like binary search or discrete search will be supported in the near future.

### Example

#### The Problem

Search for the minimum $y$ in $y = x_1^2 + x_2^2$ where $-10 \le x_1, x_2 \le 10$

#### The Code

```rust
use pso_rust::basic::BasePsoNode;
use pso_rust::traits::{PsoAcc, PsoHandler, PsoParticle};

// Here define the basic problem into solution-formed structure.
struct Particle {
    x1: f64,
    x2: f64,
}
impl PsoParticle<f64> for Particle {
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

// Here to save time and to optimize the length of the code, I defined a versatile data structure to represent those parameters like inertia and just keep them.
// You can define them separately and give them more functions like dynamic value controlled by generations and modified by the node.
// Advanced usage please see below.
#[derive(Debug, Clone)]
struct DATA<T>
where
    T: Debug + Clone
{
    value: T,
}
impl<T> DATA<T>
where
    T: Debug + Clone
{
    fn new(v: T) -> DATA<T> {
        DATA { value: v }
    }
}
impl<T> PsoAcc<BasePsoNode<Particle>, f64, T> for DATA<T>
where
    T: Debug + Clone
{
    fn get_value(&self, generation: &usize, this_node: &BasePsoNode<Particle>) -> T {
        self.value.clone()
    }
    fn init_value(&self) -> T {
        self.value.clone()
    }
}
let speed_field = DATA::new(vec![(-2.0, 2.0), (-2.0, 2.0)]);
let position_field = vec![(-10.0, 10.0), (-10.0, 10.0)];
let inertia = DATA::new(0.5);
let lfactor1 = DATA::new(2.0);
let lfactor2 = DATA::new(2.0);
let mut handler = pso_rust::basic::BasicPsoHandler::new(
    15,             // node amount
    2,              // dimension
    speed_field,    // limit of speed
    position_field, // limit of position for initialize
    inertia,        // inertia of the method
    lfactor1,       // learning factor 1
    lfactor2,       // learning factor 2
    -100.0          // initial best performance
).unwrap();
handler.start(200); // number of generation
println!("Best Performance{}", handler.get_global_best_performance());
let pos = handler.get_global_best_position();
println!("x1: {}, x2: {}", pos[0], pos[1]);
```

#### The Result

```bash
Best Performance-0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000026331326256897253
x1: -0.00000000000000000000000000000000000000000000000037074530548407006, x2: 0.0000000000000000000000000000000000000000000000015797723076922348
```

Of course, the result cannot be completely accurate. This is a computer and an optimization algorithm, after all. However, the result is also very satisfactory when the floating-point precision is ignored.

### Advance Usage

Just like what is showed above, there is two core part of what you need to do: the problem and the parameters.

#### Representing A Problem

Any problem you want to solve must implement `PsoParticle<T>`, where `T` represents how your position data is saved and used. In Basic Solution, it should be `f64`, and the genericity is prepared for the other solutions.

#### Controlling the Parameters

For the parameters, they must implement `PsoAcc<N, T, X>` trait, where `N` represents `PsoNode<T>`(in basic one, `BasePsoNode<T>` where `T` for your problem struct above), `T` represents just the way to save and use the position data. This usage is not showed in the example code above. It's used for the `this_node` param in `get_value` function, as an advance control of your parameters of the method. And, the `X` represents the return value of the method defined in the trait, that is, the type of the parameter.

By using this, you can gain more control over your parameters in the process of the optimization.
