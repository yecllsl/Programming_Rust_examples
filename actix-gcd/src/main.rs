/* use 声明可以让来自 actix-web crate 的定义用起来更容易些。当我们写下 use actix_web::{...} 时，
花括号中列出的每个名称都可以直接用在代码中，而不必每次都拼出全名 */
use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() {
    /* 我们传给 HttpServer::new 的参数是 Rust 闭包表达式 || { App::new() ... }。
    闭包是一个可以像函数一样被调用的值。这个闭包没有参数，如果有参数，那么可以将参数名
    放在两条竖线 || 之间。{ ... }是闭包的主体。当我们启动服务器时，Actix 会启动一个
    线程池来处理传入的请求。每个线程都会调用这个闭包来获取 App 值的新副本，以告诉
    此线程该如何路由这些请求并处理它们。 */
    let server = HttpServer::new(|| {
        /* 闭包会调用 App::new 来创建一个新的空白 App，然后调用它的 route 方法为路
        径 "/" 添加一个路由。提供给该路由的处理程序 web::get().to(get_index) 会通
        过调用函数 get_index 来处理 HTTP 的GET 请求。route 方法的返回值就是调用它
        的那个 App，不过其现在已经有了新的路由。由于闭包主体的末尾没有分号，因此此
        App 就是闭包的返回值，可供 HttpServer 线程使用。 */
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))//必须将 post_gcd 注册为表单处理程序。
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .await
        .expect("error running server");
}
/* get_index 函数会构建一个 HttpResponse 值，该值表示对 HTTP GET / 请求的响应。
HttpResponse::Ok() 表示 HTTP 200 OK 状态，意味着请求成功。我们会调用它的 content_type
方法和 body 方法来填入该响应的细节，每次调用都会返回在前一次基础上修改过的 HttpResponse。
最后会以 body 的返回值作为 get_index 的返回值。 */
async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        /* 由于响应文本包含很多双引号，因此我们使用 Rust 的“原始字符串”语法来编写它：
        首先是字母r、0 到多个井号（#）标记、一个双引号，然后是字符串本体，并以另一个
        双引号结尾，后跟相同数量的 # 标记。任何字符都可以出现在原始字符串中而不被转义，
        包括双引号。事实上，Rust 根本不认识像 \" 这样的转义序列。我们总是可以在引号
        周围使用比文本内容中出现过的 # 更多的 # 标记，以确保字符串能在期望的地方结束。 */
        r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,
    )
}

use serde::Deserialize;
/* 在类型定义之上放置一个 #[derive(Deserialize)]
属性会要求 serde crate 在程序编译时检查此类型并自动生成代码，以便从 HTML
表单 POST 提交过来的格式化数据中解析出此类型的值。 */
#[derive(Deserialize)]
/* 定义了一个名为 GcdParameters 的新类型，它有两个字段（n 和 m）​，
每个字段都是一个u64，这是我们的 gcd 函数想要的参数类型。 */
struct GcdParameters {
    n: u64,
    m: u64,
}
/* 对于用作 Actix 请求处理程序的函数，其参数必须全都是 Actix 知道该如何从 
HTTP 请求中提取出来的类型。post_gcd 函数接受一个参数 form，其类型为
 web::Form<GcdParameters>。当且仅当 T可以从 HTML 表单提交过来的数据反序列化时，
 Actix 才能知道该如何从 HTTP 请求中提取任意类型为 web::Form<T> 的值。
 由于我们已经将 #[derive(Deserialize)] 属性放在了 GcdParameters 类型定义上，
 Actix 可以从表单数据中反序列化它，因此请求处理程序可以要求以web::Form<GcdParameters>
  值作为参数。这些类型和函数之间的关系都是在编译期指定的。如果使用了 Actix 
  不知道该如何处理的参数类型来编写处理函数，那么 Rust 编译器会直接向你报错。 */
async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    /* 来看看 post_gcd 内部，如果任何一个参数为 0，则该函数会先行返回 
    HTTP 400 BAD REQUEST 错误，因为如果它们为 0，我们的 gcd 函数将崩溃。
    同时，post_gcd 会使用 format! 宏来为此请求构造出响应体。
    format! 与 println! 很像，但它不会将文本写入标准输出，而是会将其作为
    字符串返回。一旦获得响应文本，post_gcd 就会将其包装在 HTTP 200 OK 响应中，
    设置其内容类型，并将它返回给请求者。 */
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response = format!(
        "The greatest common divisor of the numbers {} and {} \
                 is <b>{}</b>\n",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}

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
