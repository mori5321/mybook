// Box::<dyn Fn<T> -> A>
// はヒープ上に確保された関数とデータへのポインタを持つクロージャへのスマートポインタである
// fn mul_x(x: u64) -> Box::<dyn Fn(u64) -> u64> {
//     // xは自由変数であるため、
//     // xを借用するか 所有権を移す必要がある
//     Box::new(move |y| x * y)
// }

// 自由変数 = 関数の外で定義される変数
// 束縛変数 = 関数内で定義される変数
//
// C言語は関数内で関数を定義できないため
// 自由変数 = グローバル変数
// 束縛変数 = ローカル変数
// が成り立つ
//
// しかしRustでは 関数内で関数を定義できるため
// グローバル変数 = 自由変数
// ローカル変数 = 自由変数 or 束縛変数 のどちらにもなりうる


fn main() {
    let f = mul_x(3);
    println!("{}", f(4));

    let f2 = |a| move |b| a * b;
    println!("{}", f2(3)(4));
}


// struct Foo {
//     val: u32
// }

// ライフタイムも方の一種である
// ライフタイムを明示できるのは参照のみである

// fn add<'a>(x: &'a Foo, y: &'a Foo) -> u32 {
//     x.val + y.val
// }
//
// fn main() {
//     let x = Foo { val: 3 };
//
//     {
//         let y = Foo { val: 4 };
//         let z = add(&x, &y);
//         println!("{}", z);
//     }
// }

// Shared Nothing を 時間単位での排他性で実現するのがRust
//
//


// unwrapのつかいどころ => 呼んでもpanicが発生しないと思われるとき
// 基本的にはエラーハンドリングを行うべき
//

// use std::thread::spawn;
//
// // fn hello() {
// //     println!("Hello World");
// // }
// //
// // fn main() {
// //     let _ = spawn(hello).join(); // デタッチスレッドであるためjoinする必要はないが、join関数でスレッドの終了を待つこともできる
// //
// //     let h = || println!("Hello World");
// //     let _ = spawn(h).join();
// // }
// //
//
// fn func() {
//     let v = 10;
//     let f = move || v * 2;
//
//     let result = spawn(f).join();
//
//     println!("{}", result.unwrap());
//
//     // スレッドがpanicしたらEither Errがもらえる
//     match spawn(|| panic!("I'm panicked")).join() {
//         Ok(_) => println!("Success"),
//         Err(e) => println!("Error: {:?}", e),
//     }
// }
//
// fn main() {
//     func();
// }
//
