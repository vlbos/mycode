#![forbid(unsafe_code)]
#![warn(
	trivial_numeric_casts,
	unused_import_braces,
	unused_qualifications,
	unused_results,
	unreachable_pub,
	clippy::pedantic
)]
#![allow(
	clippy::too_many_lines,
	clippy::many_single_char_names,
	clippy::self_assignment,
	clippy::uninlined_format_args,
	unused_imports
)]

use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Debug, mem::size_of};

use serde_closure::{Fn, FnMut, FnOnce};
// use std::{future::Future, pin::Pin};

// pub struct CronOrchestrator<T> {
//     services: Vec<Service<T>>,
// }

// impl<T> CronOrchestrator<T> {
//     pub fn new() -> CronOrchestrator<T> {
//         CronOrchestrator {
//             services: Vec::new(),
//         }
//     }

//     pub fn add_new_service(&mut self, service: Service<T>) {
//         self.services.push(service);
//     }

//     pub async fn run(&mut self) {
//         let services = &self.services;

//         for service in services {
//             service().await;
//         }
//     }
// }

// type ServiceResponse<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;
// type Service<T> = Box<dyn Fn() -> ServiceResponse<T>>;


// #[tokio::main]
// async fn main() {
//     let mut cron_orchestrator = CronOrchestrator::new();

//     let person = Person {
//         name: "Marcelo".to_string()
//     };

//     cron_orchestrator.add_new_service(Box::new(|| Box::pin(fun1()) ));
//     cron_orchestrator.add_new_service(Box::new(|| Box::pin(fun2("Help me")) ));
//     cron_orchestrator.add_new_service(Box::new(move || Box::pin(fun3(person.clone())) ));

//     cron_orchestrator.run().await;
// }


// async fn fun1() {
//     println!("Help me");
// }

// // ----------

// async fn fun2(text: &str) {
//     println!("{}", text);
// }

// // ----------

// #[derive(Clone)]
// struct Person {
//     pub name: String
// }

// async fn fun3(test: Person) {
//     println!("{}", test.name);
// }
// struct Thing<T> {
//     v: Vec<T>,
// }

// enum Quadrant {
//     TopTriangularQuadrant,
//     LeftTriangularQuadrant,
//     RightTriangularQuadrant,
//     BottomTriangularQuadrant,
// }

// impl<T> Thing<T> {
//     fn side_length(&self) -> usize { 0 }
//     fn len(&self) -> usize { 0 }
//     fn get(&self, x: usize, y: usize) -> &T { &self.v[0] }
//     fn get_quadrant(&self, quadrant: Quadrant) -> Vec<&T> {
//         let width = self.side_length();
//         let get_triangle = |mut start: usize, mut end: usize| {
//             let mut result = Vec::with_capacity(width/2);
//             while end - start > 0 {
//                 let mut row = Vec::with_capacity(width/2);
//                 for i in start..end {
//                     row.push(i);
//                 }
//                 result.push(row);
//                 start -= 1;
//                 end -= 1;
//             }
//             result
//         };
        
//         let start = 0;
//         let get_quadrant_like = |
//             f: &dyn Fn(usize) -> usize, // fn(usize) -> usize, 
//             g: &dyn Fn(usize, usize) -> (usize, usize), // fn(usize, usize) -> (usize, usize)
//         | {
//             let columns = get_triangle(start, width);
//             let mut result = Vec::with_capacity(self.len()/2);
//             for (i, column) in columns.iter().enumerate() {
//                 let fixed = f(i);
//                 for loose in column.iter() {
//                     let (x, y) = g(fixed, *loose);
//                     result.push(self.get(x, y));
//                 }
//             }
//             result
//         };

//         match quadrant {
//             Quadrant::RightTriangularQuadrant => {
//                 let columns = get_triangle(0, width);
//                 let f = |x: usize| width - 1 - x;
//                 let g = |a: usize, b: usize| (a,b);
//                 get_quadrant_like(&f, &g)
//             },
//             Quadrant::TopTriangularQuadrant => todo!(),
//             Quadrant::LeftTriangularQuadrant => todo!(),
//             Quadrant::BottomTriangularQuadrant => todo!(),
//             _ => todo!(),
//         }
//     }
// }

// type InstructionDyn=dyn Fn(&mut i32);
// type InstructionFn=Box<dyn Fn(&mut i32)>;
// #[derive(Clone,Hash,Debug,Eq,PartialEq)]
// pub enum InstructionType{
// Function(InstructionFn)
// }

// fn test(f:InstructionFn){
//     let mut i=0;
//     f(&mut i);
//     println!("{i}");
// }
trait Test{
    fn test(&mut self,v:i32);
fn val(&self)->i32;
}
struct Obj(i32);
impl Test for Obj{
    fn test(&mut self,v:i32){
    self.0=v;
    }
fn val(&self)->i32{
    self.0
    }
}
use std::hash::Hash;
use std::{rc::Rc,cell::RefCell};
struct ObjBox{
    ob:Rc<RefCell<dyn Test>>,
}
#[derive(Clone,Debug)]
struct Eval{
}

            #[derive(Hash, Clone, Debug)]
            struct Prover<F: FnOnce( &mut Eval)
			+ Serialize
			+ DeserializeOwned
			+ PartialEq
			+ Eq
			+ Clone
			+ Debug+Hash> {
                pub f:F,
            }

// use std::rc::Rc;
// use std::cell::RefCell;
trait TestA{
}
struct A{
t:Rc<RefCell<dyn TestA>>,
}

// impl Hash for InstructionDyn + Send + Sync {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.dyn_hash(state)
//     }
// }

// impl PartialEq for InstructionDyn + Send + Sync {
//     fn eq(&self, other: &Self) -> bool {
//         false
//     }
// }

// impl Eq for InstructionDyn + Send + Sync {}

// dyn_clone::clone_trait_object!(Instruction);

// use dyn_clone::{DynClone, clone_trait_object};
// use std::cmp::Ordering;
// use std::collections::HashSet;
// use std::fmt::{Debug, Formatter};
// use std::hash::{Hash, Hasher};
// use enum_dispatch::enum_dispatch;
// trait DynHash {
//     fn dyn_hash(&self, state: &mut dyn Hasher);
// }

// impl<T: Hash> DynHash for T {
//     fn dyn_hash(&self, mut state: &mut dyn Hasher) {
//         self.hash(&mut state)
//     }
// }

// impl Hash for dyn DynHash + '_ {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.dyn_hash(state)
//     }
// }




        //     impl <'a,G:CGConfig+Hash+Clone+Debug> Instruction for Prover<'a,G> {
        //         fn evaluate(&self, evaluator: &mut CircuitEvaluator) {
        //             let myValue = evaluator
        //                 .getWireValuei(self.long_element.clone(), LongElement::CHUNK_BITWIDTH);
        //             let otherValue =
        //                 evaluator.getWireValuei(self.other.clone(), LongElement::CHUNK_BITWIDTH);
        //             let resultValue = myValue.sub(otherValue);
        //             assert!(
        //                 resultValue.sign() != Sign::Minus,
        //                 "Result of subtraction is negative!"
        //             );
        //             evaluator.setWireValuebi(
        //                 self.result.clone(),
        //                 resultValue,
        //                 LongElement::CHUNK_BITWIDTH,
        //             );
        //         }
        //     }

        //     Box::new(Prover::<'_,C> {
        //         long_element: self.clone(),
        //         other: rhs.clone(),
        //         result: result.clone(),
        //     })
        // });




// playground_hacks! {
//     #[my_attr]
//     fn foo ()
//     {
//         let x = "Answer to the Ultimate Question of Life, the Universe, and Everything";
//         assert_eq!(x, 42);
//     }
// }

// extern crate proc_macro2 as proc_macro; // Playground hacks
// mod proc_macro_crate {
//     #![allow(unused_imports)] // typical proc-macro imports.
//     use ::proc_macro::TokenStream;
//     use ::proc_macro2::{Span, TokenStream as TokenStream2};
//     use ::quote::{
//         format_ident,
//         quote,
//         quote_spanned,
//         ToTokens,
//     };
//     use ::syn::{*,
//         parse::{Parse, Parser, ParseStream},
//         punctuated::Punctuated,
//         Result,
//         visit_mut::{self, VisitMut},
//     };
    
//     /// Playground hack
//     macro_rules! parse_macro_input {( $input:expr ) => (
//         match ::syn::parse2($input) {
//             Ok(it) => it,
//             Err(err) => return err.to_compile_error().into(),
//         }
//     )}
    
//     /* Not in the playground
//     #[proc_macro_attribute] // */
//     pub fn my_attr (
//         attrs: TokenStream,
//         input: TokenStream,
//     ) -> TokenStream
//     {
//         let _: parse::Nothing = parse_macro_input!(attrs);
        
//         let mut input: ItemFn = parse_macro_input!(input);
        
//         struct Visitor {
//             /* state here */
//         }

//         impl VisitMut for Visitor {
//             fn visit_expr_lit_mut (&mut self, node: &mut ExprLit)
//             {
//                 // Sub-recurse (not really needed here since there aren't
//                 // sub-expressions within an ExprLit):
//                 visit_mut::visit_expr_lit_mut(self, node);

//                 if matches!(
//                     node.lit,
//                     Lit::Str(ref s)
//                     if s.value() == "\
//                         Answer to the Ultimate Question of Life, \
//                         the Universe, and Everything\
//                     "
//                 )
//                 {
//                     *node = parse_quote!( 42 );
//                 }
//             }
//         }
        
//         let mut visitor = Visitor { /* initial state */ };
        
//         visitor.visit_item_fn_mut(&mut input);
        
//         input.into_token_stream()
//     }
// }

// macro_rules! playground_hacks {(
//     #[$attr:ident $(($($attrs:tt)*))?]
//     $item:item
// ) => (
//     fn main ()
//     {
//         use ::proc_macro2::TokenStream;

//         let attrs: TokenStream = stringify!($($($attrs)*)?).parse().unwrap();
//         let input: TokenStream = stringify!($item).parse().unwrap();
        
//         println!("{}", proc_macro_crate::$attr(attrs, input));
//     }
// )} use playground_hacks;


trait TestDyn<T>{
    fn t(self)->Box<dyn BoxDyn<T>>;
}
trait BoxDyn<T>{
}
struct TT;
struct TestS;
impl<T> BoxDyn<T> for TestS {
   
}
impl<T> TestDyn<T> for TestS {
    fn t(self)->Box<dyn BoxDyn<T>>{
        Box::new(self)
    }
}
fn main() {
let s:Box<dyn TestDyn<TT>> =Box::new(TestS);
//         let a:i32=0;
// let b=a.clone();
    //    let t=ObjBox{ob:Rc::new(RefCell::new(Obj(0)))};
    //    t.ob.borrow_mut().test(3);
    //    println!("{}",t.ob.borrow().val());
    // let s=vec![' '];
    // // let ff=move |e:&mut Eval| };
    // let f=FnOnce!(move |e:&mut Eval|  for v in s{println!("{v:?}");});
    // let p=Prover{f};
//    let mut h=std::collections::HashMap::new();
//     let f=InstructionType::Function(Box::new(|i:&mut i32| {*i+=1;}));
//     h.insert(f.clone(),f.clone());

//    test(Box::new(|i:&mut i32| {*i+=1;}));
}
