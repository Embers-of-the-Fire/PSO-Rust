use std::fmt::Debug;

pub trait PsoParticle<T> {
    fn get_performance(&self) -> f64;
    /// Here return value represents the status of the new position.
    /// The lib does not care about the actual problem space, but only whether the node is outside of it.
    /// But it will ask you to input a limit when initialize the nodes in order to make random particles.
    ///
    /// None if the node is not outside of it, and Some(Vec<T>) otherwise. And here your return value represents the valid position.
    ///
    /// __You have to process the node position yourself, this lib does not support it.__
    ///
    /// Tips: you are not forced to save the position, since the node will always save that.
    fn update_position(&mut self, position: &Vec<T>) -> Option<Vec<T>>;
    fn init(position: Vec<T>) -> Self;
}

pub trait PsoAcc<N, T, X>
where
    X: Debug + Clone,
    N: PsoNode<T>
{
    fn get_value(&self, generation: &usize, this_node: &N) -> X;
    fn init_value(&self) -> X;
}

pub trait PsoNode<T> {
    fn get_position(&self) -> &Vec<T>;
    fn get_performance(&self) -> f64;
    fn get_best_position(&self) -> &Vec<T>;
    fn get_best_performance(&self) -> f64;
    fn get_speed(&self) -> &Vec<f64>;
    fn update_position(
        &mut self,
        rng: &mut rand::rngs::ThreadRng,
        vlimit: &Vec<(f64, f64)>,
        inertia: &f64,
        lfactor1: &f64,
        lfactor2: &f64,
        global_best: &Vec<T>,
    ) -> f64;
}

pub trait PsoHandler<T> {
    fn get_global_best_position(&self) -> &Vec<T>;
    fn get_global_best_performance(&self) -> f64;
    fn start(&mut self, generation: usize);
}
