use std::collections::HashMap;

use calc::{tree::Tree, types::{Number, TreeType}};

fn main() {
    println!("a:{}",2);
    let mut t = Tree{ inner: Some(Box::new(
        TreeType::Plus(
            Box::new(TreeType::Number(calc::types::Number::Number(31))),
            Box::new(TreeType::Number(calc::types::Number::Number(52)))
        ))),
        variables:  HashMap::new()
    };
    println!("{}",t.infixprint().unwrap());

    let str = "5+5+5+ad*(5*4*3)";
    //let str = "5 31 + 5 +";
    //let str = "+5,+ 555 5";
    //Tree::parse_auto(&mut t, "+42,+53+(+44,33),4".to_owned());
    Tree::parse_auto(&mut t, str.to_owned()).unwrap();
    t.set_var("ad", 0);
    t.simplify();
    println!("is number? {:?}",t.is_number());
    // Tree::parse_auto(&mut t,"(31+53)+3".to_string()).unwrap();
    //let a = tree::split_str(&"a+v+34-ht-adfa*/*+j".to_string(), &["+","-","*","/","*/*+"],&[]);
    //println!("{}",t.prefixprint().unwrap());
    println!("{:?}",t.infixprint().unwrap());
}


