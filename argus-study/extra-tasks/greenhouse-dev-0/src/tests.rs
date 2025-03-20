use brew::describe_recipe;

use super::*;

describe_recipe! {
  spec
    [[botanical Reflower]
     [mineral Phosphorus]
     [botanical Alihotsy]
     [botanical Shrivelfig]] ==> Remedy
}

#[test]
fn is_correct() {
  spec(secret_recipe);
}
