use std::collections::HashMap;

use calc::{tree::{self, Tree}, types::TreeType};

fn main() {
    
    let mut t = Tree{ inner: Some(Box::new(
        TreeType::Plus(
            Box::new(TreeType::Number(calc::types::Number::Number(31))),
            Box::new(TreeType::Number(calc::types::Number::Number(52)))
        ))),
        variables:  HashMap::new()
    };
    println!("{}",t.infixprint().unwrap());

    //Tree::parse_auto(&mut t, "+42,+53+(+44,33),4".to_owned());
    Tree::parse_auto(&mut t, "5+5+5+4ad+(5*4*3)".to_owned()).unwrap();
    
    // Tree::parse_auto(&mut t,"(31+53)+3".to_string()).unwrap();
    //let a = tree::split_str(&"a+v+34-ht-adfa*/*+j".to_string(), &["+","-","*","/","*/*+"],&[]);
    //println!("{}",t.prefixprint().unwrap());
    println!("{:#?}",t);
}


enum Expr {
    Num(i32),
    Op {
        op: char,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}
