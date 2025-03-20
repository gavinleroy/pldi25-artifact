use proc_macro::TokenStream;
use quote::quote;
use syn::{
  parse::{Parse, ParseStream},
  parse_macro_input, parse_quote,
  token::Comma,
  Ident, ItemImpl, ItemStruct, ItemTrait, LitInt,
  Result, TypePath,
};

type Num = i64;

struct NumericalSystem {
  max: Num,
}

impl Parse for NumericalSystem {
  fn parse(input: ParseStream) -> Result<Self> {
    let max =
      input.parse::<LitInt>()?.base10_parse()?;
    Ok(Self { max })
  }
}

#[proc_macro]
pub fn define_numeric_system(
  item: TokenStream,
) -> TokenStream {
  let input =
    parse_macro_input!(item as NumericalSystem);

  assert!(
    input.max > 0,
    "`generate_numeric_system!` only accepts positive integers as input"
  );

  numerical_system(-input.max, input.max)
}

fn cardinal(i: Num) -> TypePath {
  let s = rust_numerals::number_to_cardinal(i);
  let mut res = String::new();
  for part in s.split(&[' ', '-']) {
    let mut chars = part.chars();
    if let Some(first) = chars.next() {
      res.extend(first.to_uppercase());
      res.extend(
        chars.flat_map(|c| c.to_lowercase()),
      );
    }
  }

  let input: TokenStream = res.parse().unwrap();
  syn::parse(input).unwrap()
}

/// Generates a numerical system in the range [min, max]
///
/// This system defines the traits `Num`, `NumEq`, `IsEven`, `IsOdd`, `IsZero`, `LessOrEqual`.
/// The traits in `std::ops` are used for addition and subtraction, note that subtraction is
/// only defined for numbers where the subtracted result is non-negative.
fn numerical_system(
  _: Num,
  max: Num,
) -> TokenStream {
  // TODO: allow negative numbers;
  let min = 0;

  let num_traits: Vec<ItemTrait> = vec![
    parse_quote! {
      pub trait Num {}
    },
    parse_quote! {
      pub trait IsEven {}
    },
    parse_quote! {
      pub trait IsOdd {}
    },
    parse_quote! {
      pub trait IsZero {}
    },
    parse_quote! {
      pub trait NonZero {}
    },
    parse_quote! {
      pub trait NumEq<T: Num> {}
    },
    parse_quote! {
      pub trait LessOrEqual<T: Num> {}
    },
  ];
  let mut struct_defs: Vec<ItemStruct> = vec![];
  let mut reflexive_impls: Vec<ItemImpl> = vec![];
  let mut binary_impls: Vec<ItemImpl> = vec![];

  // generate structs for all numbers in range [2*min ... 2*max]
  for i in (2 * min) ..= (2 * max) {
    let name = cardinal(i);
    struct_defs.push(parse_quote! {
      // define struct
      pub struct #name;
    });
    reflexive_impls.push(parse_quote! {
      // make it a num
      impl Num for #name {}
    });

    reflexive_impls.push(parse_quote! {
      // define equality on itself
      impl NumEq<#name> for #name {}
    });

    if 0 < i {
      reflexive_impls.push(parse_quote! {
        // define non-zero
        impl NonZero for #name {}
      });
    }

    if i % 2 == 0 {
      reflexive_impls.push(parse_quote! {
        impl IsEven for #name {}
      });
    } else {
      reflexive_impls.push(parse_quote! {
        impl IsOdd for #name {}
      });
    }
  }

  // Define operations for range [min, max]
  for i in min ..= max {
    let i_name = cardinal(i);
    for j in 0 ..= i {
      let j_name = cardinal(j);
      let i_sub_j_name = cardinal(i - j);

      binary_impls.push(parse_quote! {
        // generate less_or_equal
        impl LessOrEqual<#i_name> for #j_name {}
      });

      binary_impls.push(parse_quote! {
        // impl Sub<j> for i
        impl std::ops::Sub<#j_name> for #i_name {
          type Output = #i_sub_j_name;
          fn sub(self, _rhs: #j_name) -> Self::Output {
            #i_sub_j_name
          }
        }
      });
    }

    // generate addition
    for j in min .. max {
      let j_name = cardinal(j);
      let i_add_j_name = cardinal(j + i);

      // impl Add<j> for i
      binary_impls.push(parse_quote! {
        impl std::ops::Add<#j_name> for #i_name {
          type Output = #i_add_j_name;
          fn add(self, _rhs: #j_name) -> Self::Output {
            #i_add_j_name
          }
        }
      });
    }
  }

  reflexive_impls.push(parse_quote! {
    // define zero
    impl IsZero for Zero {}
  });

  TokenStream::from(quote! {
    #(#num_traits)*
    #(#struct_defs)*
    #(#reflexive_impls)*
    #(#binary_impls)*
  })
}

// ============================

struct AllNumbers {
  macro_ident: Ident,
  first: Num,
  second: Num,

  firstp: NumP,
  secondp: NumP,
}

enum NumP {
  Even,
  Odd,
}

impl Parse for AllNumbers {
  fn parse(input: ParseStream) -> Result<Self> {
    let macro_ident = input.parse::<Ident>()?;
    input.parse::<Comma>()?;

    let first =
      input.parse::<LitInt>()?.base10_parse()?;
    input.parse::<Comma>()?;
    let firstp = input.parse::<NumP>()?;

    input.parse::<Comma>()?;

    let second =
      input.parse::<LitInt>()?.base10_parse()?;
    input.parse::<Comma>()?;
    let secondp = input.parse::<NumP>()?;

    Ok(Self {
      macro_ident,
      first,
      second,
      firstp,
      secondp,
    })
  }
}

impl NumP {
  fn to_lambda(self) -> impl Fn(Num) -> bool {
    match self {
      Self::Even => |n| n % 2 == 0,
      Self::Odd => |n| n % 2 != 0,
    }
  }
}

impl Parse for NumP {
  fn parse(input: ParseStream) -> Result<Self> {
    let kind = input.parse::<Ident>()?;
    match kind.to_string().as_str() {
      "even" => Ok(Self::Even),
      "odd" => Ok(Self::Odd),
      _ => Err(syn::Error::new(
        kind.span(),
        "expected `even` or `odd`",
      )),
    }
  }
}

#[proc_macro]
pub fn for_nums_p(
  item: TokenStream,
) -> TokenStream {
  let input =
    parse_macro_input!(item as AllNumbers);
  for_numbers_p(input)
}

// TODO expand to any number of combinations
fn for_numbers_p(cfg: AllNumbers) -> TokenStream {
  let mut invocations = vec![];
  let macro_ident = &cfg.macro_ident;

  let i_p = cfg.firstp.to_lambda();
  let j_p = cfg.secondp.to_lambda();
  for i in 0 .. cfg.first {
    if !i_p(i) {
      continue;
    }

    let i_name = cardinal(i);
    for j in 0 .. cfg.second {
      if !j_p(j) {
        continue;
      }

      let j_name = cardinal(j);
      invocations.push(quote! {
        #macro_ident!(#i_name, #j_name);
      });
    }
  }

  TokenStream::from(quote! {
    #(#invocations)*
  })
}
