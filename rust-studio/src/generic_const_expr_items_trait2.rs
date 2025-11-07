#![allow(incomplete_features, dead_code)]
#![feature(generic_const_exprs)]

trait MathTraitBase {
    const NDIM: usize;
}

trait MathTrait: MathTraitBase {
    fn calculate_math(&mut self, data: &mut [f64]);
}

struct Solver<'a, M>
where
    M: MathTrait + Sized,
    [f64; M::NDIM]: Sized,
{
    pub x: [f64; M::NDIM],
    problem: &'a mut M,
}

impl<'a, M> Solver<'a, M> 
where
    M: MathTrait + Sized,
    [f64; M::NDIM]: Sized,
{
    pub fn new(problem: &'a mut M) -> Solver<'a, M> {
        Solver::<'a, M> {
            x: [0.0_f64; M::NDIM],
            problem,
        }
    }
}

// NOTE: Added `Clone` superbound, see the other NOTE below for reason.
trait PL: Clone {
    const NSIZE: usize;
}

#[derive(Clone)]
struct PhysicsComp {}

impl PL for PhysicsComp {
    const NSIZE: usize = 32;
}

struct Physics<P: PL> {
    comp: P,
}

impl<P: PL> MathTraitBase for Physics<P> {
    const NDIM: usize = P::NSIZE;
}

impl<P: PL> MathTrait for Physics<P>
where
    [f64; Self::NDIM]: Sized,
{
    fn calculate_math(&mut self, data: &mut [f64]) {
        let x = [1.0; Self::NDIM];
        data.copy_from_slice(&x);
    }
}

struct DoStuff<P: PL> {
    data: P,
}

impl<P: PL> DoStuff<P>
where
    [f64; Physics::<P>::NDIM]: Sized,
{
    fn problem(&self) -> bool {
        let mut phys = Physics {
            // NOTE: I've changed `PhysicsComp {}` here to `self.data.clone()`
            // because the `solver` below expects a `Physics<P>` not `Physics<PhysicsComp>`.
            comp: self.data.clone(),
        };
        let mut solver = Solver::<Physics::<P>>::new(&mut phys);
        solver.problem.calculate_math(&mut solver.x);
        true
    }
}


fn main() {
    let failure = DoStuff {
        data: PhysicsComp {},
    };
    failure.problem();
}