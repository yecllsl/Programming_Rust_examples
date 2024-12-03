#![warn(rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

/* 将标准库中的 FromStr特型引入了当前作用域。特型是可以由类型实现的方法集合。任何实现了 FromStr 特型的类型都有一个 from_str 方法，
该方法会尝试从字符串中解析这个类型的值。u64 类型实现了 FromStr，所以我们将调用 u64::from_str 来解析程序中的命令行参数。
尽管我们从未在程序的其他地方用到 FromStr 这个名字，但仍然要 use（使用）它，因为要想使用某个特型的方法，该特型就必须在作用域内。
 */
use std::env;
/* 第二个 use 声明引入了 std::env 模块，该模块提供了与执行环境交互时会用到的几个函数和类型，包括 args 函数，该函数能让我们访问程序中的命令行参数。 */
use std::str::FromStr;

fn main() {
    //main 函数没有返回值，所以可以简单地省略 -> 和通常会跟在参数表后面的返回类型。
    /* numbers 的类型是 Vec<u64>，这是一个可以容纳 u64 类型的值的向量，但和以前一样，不需要把类型写出来。Rust 会推断它，
    一部分原因是我们将 u64 类型的值压入了此向量，另一部分原因是我们将此向量的元素传给了 gcd，后者只接受 u64 类型的值。 */
    let mut numbers = Vec::new();
    /* 这里使用了 for 循环来处理命令行参数，依次将变量 arg 指向每个参数并运行循环体。std::env 模块的 args 函数会返回一个迭代器，
    此迭代器会按需生成1每个参数，并在完成时给出提示。 */
    for arg in env::args().skip(1) {
        /* args 返回的迭代器生成的第一个值永远是正在运行的程序的名称。如果想跳过它，
        就要调用迭代器的 skip方法来生成一个新的迭代器，新迭代器会略去第一个值。 */

        /* 这里我们调用了 u64::from_str 来试图将命令行参数 arg 解析为一个无符号的 64 位整数。u64::from_str 并不是 u64 值上的某个方法，
        而是与 u64 类型相关联的函数，类似于 C++或 Java 中的静态方法。from_str 函数不会直接返回 u64，
        而是返回一个指明本次解析已成功或失败的 Result 值。Result值是以下两种变体之一：形如 Ok(v) 的值，
        表示解析成功了，v 是所生成的值；形如 Err(e) 的值，表示解析失败了，e 是解释原因的错误值。执行任何可能会失败的操作（例如执行输入或输出
        或者以其他方式与操作系统交互）的函数都会返回一个 Result 类型，其Ok 变体会携带成功结果（传输的字节数、打开的文件等）​，
        而其 Err 变体会携带错误码，以指明出了什么问题。与大多数现代语言不同，Rust 没有异常（exception）​：所有错误都使用 Result 或 panic 进行处理，

        我们用 Result 的 expect 方法来检查本次解析是否成功。如果结果是 Err(e)，那么 expect 就会打印出一条包含 e 的消息并直接退出程序。
        但如果结果是 Ok(v)，则 expect 会简单地返回 v 本身，最终我们会将其压入这个数值向量的末尾。 */
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    /* 空数组没有最大公约数，因此要检查此向量是否至少包含一个元素，如果没有则退出程序并报错。这里我们用 eprintln! 宏将错误消息写入标准错误流。 */
    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    // 所以在进行迭代时，需要告诉 Rust，该向量的所有权应该留在 numbers 上，我们只是为了本次循环而借用它的元素。&numbers[1..] 中的 & 运算符会从向量中借用从第二个元素开始的引用
    /* for 循环会遍历这些被引用的元素，让 m 依次借出每个元素。*m 中的 * 运算符会将 m解引用，产生它所引用的值，这就是要传给 gcd 的下一个 u64。
    最后，由于numbers 拥有着此向量，因此当 main 末尾的 numbers 超出作用域时，Rust 会自动释放它。 */
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
