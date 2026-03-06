use std::default;

use easy_storage::Storeable;
use serde::{Deserialize, Serialize};

trait Print {
    fn print(&self, is_jp: bool) -> String;
}

#[derive(Debug)]
enum ArgType {
    LongOption(OptionV),
    ShortOption(OptionV),
    PositionalOption(String),
}

impl Print for ArgType {
    fn print(&self, is_jp: bool) -> String {
        let head = if is_jp { "引数タイプ" } else { "arg type" };
        match self {
            ArgType::LongOption(option_v) => {
                format!(
                    "{}: {}\n{}",
                    head,
                    if is_jp { "ロング引数" } else { "long arg" },
                    option_v.print(is_jp)
                )
            }
            ArgType::ShortOption(option_v) => {
                format!(
                    "{}: {}\n{}",
                    head,
                    if is_jp {
                        "ショート引数"
                    } else {
                        "short arg"
                    },
                    option_v.print(is_jp)
                )
            }
            ArgType::PositionalOption(s) => s.to_string(),
        }
    }
}

#[derive(Debug)]
struct OptionV {
    name: String,
    value: Option<String>,
}

impl Print for OptionV {
    fn print(&self, is_jp: bool) -> String {
        let opt_head = if is_jp {
            "オプション名"
        } else {
            "option name"
        };
        let val_head = if is_jp {
            "オプション値"
        } else {
            "option value"
        };
        format!(
            "{}: {}\n{}: {}\n",
            opt_head,
            self.name,
            val_head,
            self.value.clone().unwrap_or("None".to_string())
        )
    }
}

enum OptType {
    Long,
    Short,
    Posi,
}

fn strict_starts_with<T: AsRef<str>>(s: T) -> OptType {
    let mut count = 0;
    for (_, c) in s.as_ref().char_indices() {
        if c == '-' {
            count += 1;
        }
    }
    match count {
        1 => OptType::Short,
        2 => OptType::Long,
        _ => OptType::Posi,
    }
}

fn parse(args: Vec<String>) -> Vec<ArgType> {
    let mut parsed = Vec::new();

    let mut iter = args.into_iter().skip(1).peekable();
    while let Some(arg) = iter.next() {
        let argv = arg.trim().to_string();

        let res = match strict_starts_with(argv.clone()) {
            OptType::Long => {
                let v = match iter.peek() {
                    Some(next) if matches!(strict_starts_with(next), OptType::Posi) => {
                        iter.next() // Posiのときだけ消費
                    }
                    _ => None,
                };
                ArgType::LongOption(OptionV {
                    name: argv,
                    value: v,
                })
            }
            OptType::Short => {
                let v = match iter.peek() {
                    Some(next) if matches!(strict_starts_with(next), OptType::Posi) => iter.next(),
                    _ => None,
                };
                ArgType::ShortOption(OptionV {
                    // ShortはShortOptionでは？
                    name: argv,
                    value: v,
                })
            }
            OptType::Posi => ArgType::PositionalOption(argv),
        };

        parsed.push(res);
    }
    parsed
}

impl Print for Vec<ArgType> {
    fn print(&self, is_jp: bool) -> String {
        self.iter()
            .map(|f| format!("{}\n", f.print(is_jp)))
            .collect::<String>()
    }
}

trait Table {
    fn to_table(&self) -> String;
}

impl Table for Vec<ArgType> {
    fn to_table(&self) -> String {
        let mut table = String::new();
        table.push_str("| option | type | value |\n| :---: | :---: | :---: |\n");
        for i in self {
            let res = match i {
                ArgType::LongOption(option_v) => {
                    format!(
                        "| {} | Long | {} |",
                        option_v.name,
                        option_v.value.clone().unwrap_or("None".to_string())
                    )
                }
                ArgType::ShortOption(option_v) => {
                    format!(
                        "| {} | Short | {} |",
                        option_v.name,
                        option_v.value.clone().unwrap_or("None".to_string())
                    )
                }
                ArgType::PositionalOption(v) => {
                    format!("| {} | Position | - |", v)
                }
            };

            table.push_str(&format!("{}\n", res));
        }
        // let (max_op, value_op) = {
        //     let (mut tmp_max_op, mut tmp_max_val) = (6, 5);
        //     for f in self {
        //         match f {
        //             ArgType::LongOption(option_v) => {
        //                 if tmp_max_op < option_v.name.char_indices().count() {
        //                     tmp_max_op = option_v.name.char_indices().count()
        //                 }
        //                 if let Some(v) = option_v.value.clone()
        //                     && tmp_max_val < v.char_indices().count()
        //                 {
        //                     tmp_max_val = v.char_indices().count()
        //                 }
        //             }
        //             ArgType::ShortOption(option_v) => {
        //                 if tmp_max_op < option_v.name.char_indices().count() {
        //                     tmp_max_op = option_v.name.char_indices().count()
        //                 }
        //                 if let Some(v) = option_v.value.clone()
        //                     && tmp_max_val < v.char_indices().count()
        //                 {
        //                     tmp_max_val = v.char_indices().count()
        //                 }
        //             }
        //             ArgType::PositionalOption(v) => {
        //                 if tmp_max_op < v.char_indices().count() {
        //                     tmp_max_op = v.char_indices().count()
        //                 }
        //             }
        //         }
        //     }
        //     (tmp_max_op, tmp_max_val)
        // };
        //
        // table.push_str(&format!(
        //     "┌{}┬{}┬{}┐\n│{}│{}│{}│\n└{}┴{}┴{}┘",
        //     "─".repeat(max_op + 2),
        //     "─".repeat(7),
        //     "─".repeat(value_op + 2),
        //     {
        //         let btw_space = " ".repeat((max_op + 1 - 6) / 2);
        //         format!("│{}│{}│{}│", btw_space, "Option", btw_space)
        //     },
        //     "│ type │",
        //     {
        //         let btw_s = " ".repeat((value_op + 1 - 5) / 2);
        //         format!("│{}│{}│{}│", btw_s, "value", btw_s)
        //     },
        //     "─".repeat(max_op + 2),
        //     "─".repeat(7),
        //     "─".repeat(value_op + 2),
        // ));
        //
        // for i in self {
        //     match i {
        //         ArgType::LongOption(option_v) => {
        //             format!("│k")
        //         }
        //         A::rgType::ShortOption(option_v) => todo!(),
        //         ArgType::PositionalOption(_) => todo!(),
        //     }
        // }

        table
    }
}

#[test]
fn test_parse() {
    let args = vec![
        "prog".to_string(),
        "--foo".to_string(),
        "bar".to_string(),
        "--foobar".to_string(),
    ];
    println!("{:?}", args);
    let result = parse(args);
    // println!("{}", result.print(false));
    println!("{}", result.to_table());
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Config {
    show_as_table: bool,
}

impl easy_storage::Storeable for Config {}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    let conf = Config::load_by_extension("").unwrap_or_default();

    let res = if args.len() < 2 {
        String::from("Args isn't.")
    } else {
        let p = parse(args);
        if !conf.show_as_table {
            p.to_table()
        } else {
            p.print(false)
        }
    };

    println!("{}", res);
}
