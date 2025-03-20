use brew::describe_recipe;

use super::*;

describe_recipe! {
  spec
    [[botanical Shrivelfig]
     [botanical Wiggentree]] ==> Potion
}

#[test]
fn is_correct() {
  spec(secret_recipe);
}
