numerical_macros::define_numeric_system!(10);

// crate::make_simple! {
//     pub Count ==>
//     Zero,
//     One,
//     Two,
//     Three,
//     Four,
//     Five,
//     Six
// }

// pub trait Add1: Count {
//   type Output: Count;
// }

// macro_rules! add1_to {
//     ($([$from:ident, $to:ident]),*) => {
//         $(
//             impl Add1 for $from {
//                 type Output = $to;
//             }
//         )*
//     };
// }

// add1_to! {
//   [Zero, One],
//   [One, Two],
//   [Two, Three],
//   [Three, Four],
//   [Four, Five],
//   [Five, Six]
// }
