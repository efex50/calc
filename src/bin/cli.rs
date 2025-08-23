use std::collections::HashMap;

use calc::tree::{self, Tree, TreeType};

fn main() {
    
    let mut t = Tree{ inner: Some(Box::new(
        TreeType::Plus(
            Box::new(TreeType::Number(calc::types::Number::Number(31))),
            Box::new(TreeType::Number(calc::types::Number::Number(52)))
        ))),
        variables:  HashMap::new()
    };
    println!("{}",t.infixprint().unwrap());
    println!("-------------");
    Tree::parse_auto(&mut t,"+asfas+213+442,b".to_string()).unwrap();
    println!("+++++++++++++");
    let a = tree::split_str(&"a+v+34-ht-adfa*/*+j".to_string(), &["+","-","*","/","*/*+"],&[]);
    println!("{:?}",a);
}
