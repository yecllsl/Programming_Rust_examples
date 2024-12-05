
#[derive(Debug)]//属性会让编译器生成一些额外的代码，这能让我们在 println! 中使用 {:?} 来格式化 Arguments 结构体。
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

use text_colorizer::*;

/* 如果用户输入的参数个数不对，那么通常会打印出一份关于如何使用本程序的简单说明。
我们会使用一个名为 print_usage 的简单函数来完成此操作，并从 text-colorizer 导入所有内容，
以便为这些输出添加一些颜色 */
fn print_usage() {
    eprintln!("{} - change occurrences of one string into another",
              "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

use std::env;

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();// 从命令行参数中获取参数并将它们收集到一个 Vec 中。.skip(1) 会跳过第一个参数，因为第一个参数是程序的名称。
/* 首先 collect() 方法会生成一个 Vec 参数。然后我们会检查它的参数个数是否正确，
如果不正确，则打印一条信息并以返回一个错误代码的形式退出。接下来我们再次对部分
信息进行着色，并用.bold() 把这段文本加粗。如果参数个数正确，就把它们放入一个
 Arguments 结构体中，并返回该结构体。 */
    if args.len() != 4 {
        print_usage();
        eprintln!("{} wrong number of arguments: expected 4, got {}.",
            "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone()
    }
}
use std::fs;
fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
           eprintln!("{} failed to read from file '{}': {:?}",
               "Error:".red().bold(), args.filename, e);
           std::process::exit(1);
        }
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}",
                "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to write to file '{}': {:?}",
                "Error:".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };
}
/* fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read from file '{}': {:?}",
                      "Error:".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed to write to file '{}': {:?}",
                "Error:".red().bold(), args.output, e);
            std::process::exit(1);
        }
    };
} */
/* fn main() {
    let args = parse_args();
    println!("{:?}", args);
} */

use regex::Regex;
fn replace(target: &str, replacement: &str, text: &str)
    -> Result<String, regex::Error>
{
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}