use clap::{Parser, ValueEnum, CommandFactory};
use std::io::{self, Read, IsTerminal};
use calc::{tree::Tree, util::as_i128};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    string: Option<String>,

    #[arg(short, long)]
    var: Vec<String>,

    #[arg(long, default_value_t = false)]
    simplify: bool,

    #[arg(long, default_value_t = false)]
    space: bool,

    #[arg(long, value_enum)]
    print: Vec<PrintType>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum PrintType {
    Infix,
    Prefix,
    Postfix,
    Debug,
}

fn main() {
    let args = Args::parse();
    let input_str = match args.string {
        Some(s) => s,
        None => {
            if !io::stdin().is_terminal() {
                let mut buffer = String::new();
                if let Err(e) = io::stdin().read_to_string(&mut buffer) {
                    eprintln!("Hata: Stdin okunamadı: {}", e);
                    std::process::exit(1);
                }
                buffer.trim().to_string()
            } else {
                let mut cmd = Args::command();
                cmd.error(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "Bir ifade girilmedi! Lütfen '-s \"ifade\"' kullanın veya pipe (|) ile veri gönderin."
                ).exit();
            }
        }
    };

    let mut tree = {
        if args.space{
            Tree::new(' ')
        }else {   
            Tree::new(',')
        }
    };

    if let Err(_) = tree.parse_auto(input_str) {
        eprintln!("Hata: İfade ayrıştırılamadı! Sözdizimini kontrol edin.");
        std::process::exit(1);
    }

    for v in args.var {
        if let Some((key, val_str)) = v.split_once('=') {
            if let Ok(num) = as_i128(val_str) {
                tree.set_var(key.to_string(), num);
            } else if let Ok(flt) = val_str.parse::<f64>() {
                tree.set_var(key.to_string(), flt);
            } else {
                eprintln!("Uyarı: '{}' sayı değil, '{}' değişkeni atlanıyor.", val_str, key);
            }
        } else {
            eprintln!("Uyarı: Geçersiz format '{}'. 'x=5' formatını kullanın.", v);
        }
    }

    if args.simplify {
        tree.simplify();
    }

    if args.print.is_empty() {
        if let Some(res) = tree.is_number() {
            println!("{}", res.as_str()); 
        } else {
            println!("{}", tree.infixprint().unwrap_or_default());
        }
    } else {
        for print_type in &args.print {
            match print_type {
                PrintType::Infix => println!("{}", tree.infixprint().unwrap_or("Hata".into())),
                PrintType::Prefix => println!("{}", tree.prefixprint().unwrap_or("Hata".into())),
                PrintType::Postfix => println!("{}", tree.postfixprint().unwrap_or("Hata".into())),
                PrintType::Debug => println!("{:#?}", tree),
            }
        }
        std::process::exit(1);
        
    }

    if args.simplify && !args.print.is_empty() {
        if let Some(n) = tree.is_number() {
            println!("=> Hesaplanan Değer: {}", n.as_str());
        }
    }
}