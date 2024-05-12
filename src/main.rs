fn main() {
    // 2の3乗。
    println!("{}", 2u8.pow(3));

    helloworld();

    // 2-4 “RPN計算機プログラムとデバッガによる実行”
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";

    let ans = rpn(exp);
    println!("{} = {:.4}", exp, ans);
    debug_assert_eq!("26.2840", format!("{:.4}", ans));
}

// 2-2 Hello world
fn helloworld() {
    // 最後に!がついているものはマクロ
    println!("Hello, world!{}", add(1.0, 2.0));

    // 複数の値を渡すことは可能。前から順番に埋め込まれる
    // {:.1}は小数点以下何桁まで表示するかを指定
    println!(
        "半径 {:.1}、円周率 {:.3}、面積 {:.3}",
        3.2,
        std::f64::consts::PI,
        3.2f64.powi(2) * std::f64::consts::PI
    )
}   

fn add(x: f64, y: f64) -> f64 {
    x + y
}


fn rpn(ext: &str) -> f64 {

    // Vecの型をここで宣言していないけど、推論してくれる
    // もしf64と、文字列型を混在させようとすると mismatched types expected `f64`, found `&str`rustc になる
    // mutをつけるとmutableで変更可能にできる
    let mut stack = Vec::new();

    // 空白で分割してくれる。便利。
    for token in ext.split_whitespace() {
        println!("token {}", token);

        // if let Okが分からなかった。
        // parseは指定された型に変換できるか試して、Result型で返す。成功したらOkが返る。
        // その結果をOkとパターンマッチングして評価している、且つnumに値を代入もされている
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else {
            match token {
                "+" => apply2(&mut stack, | x, y | x + y),  // mutをつけることで関数内で変更できる
                "-" => apply2(&mut stack, | x, y | x - y),  // | x, y | x - yの部分はクロージャ
                "*" => apply2(&mut stack, | x, y | x * y),
                "/" => apply2(&mut stack, | x, y | x / y),
                _ => panic!("想定外のoperator: {}", token),  // _は何にでもマッチする
            }
        }
    }
    return stack.pop().expect("数値がありません");  // returnと;を付けずに書くと、最後の式が返り値になる。明示的に付けたほうがミス減りそう
}

// トレイト境界とは、ジェネリクスの型パラメータとして受け付ける型を制限するための機能
// x + y + zを渡そうとすると、closure is expected to take 2 arguments, but it takes 3 arguments というエラーになる
fn apply2<F>(stack: &mut Vec<f64>, fun: F)
where F: Fn(f64, f64) -> f64
{
    // stackの最後の２つの数字を取り出している
    if let(Some(y), Some(x)) = (stack.pop(), stack.pop()) {
        let z = fun(x, y);
        stack.push(z);
    } else {
        panic!("例外処理")
    }
}

