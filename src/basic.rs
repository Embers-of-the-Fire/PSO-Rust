use rand::Rng;

use crate::{error::Error, traits::PsoNode};

#[derive(Debug, Clone)]
pub struct BasicPsoHandler<T, SF, I, LF>
where
    T: crate::traits::PsoParticle<f64>,
    SF: crate::traits::PsoAcc<BasePsoNode<T>, f64, Vec<(f64, f64)>>,
    I: crate::traits::PsoAcc<BasePsoNode<T>, f64, f64>,
    LF: crate::traits::PsoAcc<BasePsoNode<T>, f64, f64>,
{
    nodes: Vec<BasePsoNode<T>>,
    node_amount: usize,
    speed_field: SF,
    inertia: I,
    lfactor1: LF,
    lfactor2: LF,
    global_best_position: Vec<f64>,
    global_best_performance: f64,
}

impl<T, SF, I, LF> BasicPsoHandler<T, SF, I, LF>
where
    T: crate::traits::PsoParticle<f64>,
    SF: crate::traits::PsoAcc<BasePsoNode<T>, f64, Vec<(f64, f64)>>,
    I: crate::traits::PsoAcc<BasePsoNode<T>, f64, f64>,
    LF: crate::traits::PsoAcc<BasePsoNode<T>, f64, f64>,
{
    pub fn new(
        node_amo: usize,
        dimension: usize,
        speed_field: SF,
        position_field: Vec<(f64, f64)>,
        inertia: I,
        lfactor1: LF,
        lfactor2: LF,
        init_best: f64,
    ) -> Result<Self, Error> {
        let inv = speed_field.init_value().len();
        if inv != dimension {
            return Err(Error::new(
                format!(
                    "Position field {} does not match dimension {}",
                    inv, dimension
                ),
                Some("BasicPsoHandler-new".to_string()),
            ));
        }
        let mut s = Self {
            nodes: Vec::new(),
            node_amount: node_amo,
            speed_field,
            inertia,
            lfactor1,
            lfactor2,
            global_best_performance: init_best,
            global_best_position: Vec::new(),
        };
        s.init(position_field);
        Ok(s)
    }

    fn random_list(rng: &mut rand::rngs::ThreadRng, flimit: &Vec<(f64, f64)>) -> Vec<f64> {
        let flen = flimit.len();
        let mut res: Vec<f64> = Vec::with_capacity(flen);
        for i in 0..flen {
            let limit = flimit[i];
            let n = rng.gen_range(limit.0..=limit.1);
            res.push(n);
        }
        res
    }

    fn init(&mut self, position_field: Vec<(f64, f64)>) -> &mut Self {
        let mut nds: Vec<BasePsoNode<T>> = Vec::with_capacity(self.node_amount);
        let mut rng = rand::thread_rng();
        let init_limit = position_field;
        for _ in 0..self.node_amount {
            let rlist = Self::random_list(&mut rng, &init_limit);
            // let d = T::init(rlist);
            let bn: BasePsoNode<T> = BasePsoNode::new(rlist);
            nds.push(bn);
        }
        self.global_best_position = nds[0].get_position().clone();
        self.nodes = nds;
        self
    }
}

impl<T, SF, I, LF> crate::traits::PsoHandler<f64> for BasicPsoHandler<T, SF, I, LF>
where
    T: crate::traits::PsoParticle<f64>,
    SF: crate::traits::PsoAcc<BasePsoNode<T>, f64, Vec<(f64, f64)>>,
    I: crate::traits::PsoAcc<BasePsoNode<T>, f64, f64>,
    LF: crate::traits::PsoAcc<BasePsoNode<T>, f64, f64>,
{
    fn get_global_best_performance(&self) -> f64 {
        self.global_best_performance
    }
    fn get_global_best_position(&self) -> &Vec<f64> {
        &self.global_best_position
    }
    fn start(&mut self, generation: usize) {
        let mut rng = rand::thread_rng();
        for gene in 0..generation {
            for node in self.nodes.iter_mut() {
                let spf = self.speed_field.get_value(&gene, &node);
                let ine = self.inertia.get_value(&gene, &node);
                let l1 = self.lfactor1.get_value(&gene, &node);
                let l2 = self.lfactor2.get_value(&gene, &node);
                let gb = &self.global_best_position;
                let nbest = node.update_position(&mut rng, &spf, &ine, &l1, &l2, &gb);
                if nbest > self.global_best_performance {
                    self.global_best_performance = nbest;
                    self.global_best_position = node.get_position().clone();
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct BasePsoNode<T>
where
    T: crate::traits::PsoParticle<f64>,
{
    pub particle: T,
    position: Vec<f64>,
    speed: Vec<f64>,
    self_best_position: Vec<f64>,
    self_best_performance: f64,
}

impl<T> BasePsoNode<T>
where
    T: crate::traits::PsoParticle<f64>,
{
    pub fn new(position: Vec<f64>) -> Self {
        let d = position.len();
        let pz = position.clone();
        let ptc = T::init(position);
        let mut v = Vec::with_capacity(d);
        let pms = ptc.get_performance();
        let pzb = pz.clone();
        for _ in 0..d {
            v.push(0.0)
        }
        Self {
            particle: ptc,
            position: pz,
            speed: v,
            self_best_position: pzb,
            self_best_performance: pms,
        }
    }
}

impl<T> crate::traits::PsoNode<f64> for BasePsoNode<T>
where
    T: crate::traits::PsoParticle<f64>,
{
    fn get_best_position(&self) -> &Vec<f64> {
        &self.self_best_position
    }

    fn get_best_performance(&self) -> f64 {
        self.self_best_performance
    }

    fn get_position(&self) -> &Vec<f64> {
        &self.position
    }

    fn get_performance(&self) -> f64 {
        self.particle.get_performance()
    }

    fn get_speed(&self) -> &Vec<f64> {
        &self.speed
    }

    fn update_position(
        &mut self,
        rng: &mut rand::rngs::ThreadRng,
        vlimit: &Vec<(f64, f64)>,
        inertia: &f64,
        lfactor1: &f64,
        lfactor2: &f64,
        global_best: &Vec<f64>,
    ) -> f64 {
        let len = self.position.len();
        let mut npos: Vec<f64> = Vec::with_capacity(len);
        for i in 0..len {
            let xi = self.position[i];
            let vi = self.speed[i];
            let rand1 = rng.gen::<f64>();
            let rand2 = rng.gen::<f64>();
            let gi = global_best[i];
            let pi = self.self_best_position[i];
            let vlim = vlimit[i];
            let mut vin =
                inertia * vi + lfactor1 * rand1 * (pi - xi) + lfactor2 * rand2 * (gi - xi);
            if vin < vlim.0 {
                if (vlim.0 - vin) / vlim.0 >= 1.0 {
                    vin = rng.gen_range(vlim.0..=vlim.1);
                } else {
                    vin = vlim.0;
                }
            } else if vin > vlim.1 {
                if (vlim.1 - vin) / vlim.1 >= 1.0 {
                    vin = rng.gen_range(vlim.0..=vlim.1);
                } else {
                    vin = vlim.1;
                }
            }
            let xin = xi + vin;
            npos.push(xin);
        }
        let res = self.particle.update_position(&npos);
        match res {
            None => self.position = npos.clone(),
            Some(v) => self.position = v,
        }
        let performance = self.particle.get_performance();
        if performance > self.self_best_performance {
            self.self_best_performance = performance;
            self.self_best_position = npos;
        };
        performance
    }
}
