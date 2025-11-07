#![feature(generic_const_exprs)]

pub trait Scalar {}

pub trait VectorAbstract<TValue: Scalar>: 
{
    fn dot(lhs: &Self, rhs: &Self) -> TValue;
}

pub trait VectorDynamic<TValue: Scalar>: VectorAbstract<TValue> {
    fn dim_count(&self) -> usize;
    fn from_list(arr: Vec<TValue>) -> Self;
}

pub trait VectorStatic1<TValue: Scalar>: VectorAbstract<TValue> {
    const DIM_COUNT: usize;
    fn from_array(arr: [TValue; Self::DIM_COUNT]) -> Self;
}

pub trait VectorStatic2<TValue: Scalar, const DIM_COUNT: usize>: VectorAbstract<TValue> {
    fn from_array(arr: [TValue; DIM_COUNT]) -> Self;
}

struct VectorDynamicWrapper<T>(T);
struct VectorStaticWrapper1<T>(T);
struct VectorStaticWrapper2<T, const DIM_COUNT: usize>(T);

impl<TValue: Scalar, TVector: VectorDynamic<TValue>> VectorAbstract<TValue> for VectorDynamicWrapper<TVector> {
    fn dot(lhs: &Self, rhs: &Self) -> TValue {
        let lhs = &lhs.0;
        let rhs = &rhs.0;
        todo!()
    }
}

impl<TValue: Scalar, TVector: VectorStatic1<TValue>>
    VectorAbstract<TValue> for VectorStaticWrapper1<TVector>
{
    fn dot(lhs: &Self, rhs: &Self) -> TValue {
        let lhs = &lhs.0;
        let rhs = &rhs.0;
        let dim_count = TVector::DIM_COUNT;
        todo!()
    }
}

impl<TValue: Scalar, const DIM_COUNT: usize, TVector: VectorStatic2<TValue, DIM_COUNT>>
    VectorAbstract<TValue> for VectorStaticWrapper2<TVector, DIM_COUNT>
{
    fn dot(lhs: &Self, rhs: &Self) -> TValue {
        let lhs = &lhs.0;
        let rhs = &rhs.0;
        todo!()
    }
}