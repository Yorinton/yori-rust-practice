// Listは再帰的な列挙子を含んでいる = Listは自身の別の値を保持している
// この場合、コンパイラはこの型の値を格納するのに必要な領域を計算できない
// List型の値の格納に必要なメモリ領域を計算する際に、まずはConsを見てi32とListを1つずつ格納出来るメモリ領域が必要だと判断する
// 次にそのListの格納に必要なメモリ領域を計算するために、またConsを見にいく、というように無限に続いてしまう
// List型を参照にすることで必要なサイズを確定できる。なぜならポインタの格納に必要なサイズは決まっているから
// enumなので実際の値はCons(i32, Box<List>)かNilのどちらかになる
#[derive(Debug)] // std::fmt::Debugを実装？
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::rc::Rc;
use std::cell::RefCell;
use std::any::{type_name};
use std::ops::Add;
use std::cmp::{PartialOrd};
use std::fs::File;
#[allow(unused_imports)]
use std::io::{self, Error, ErrorKind, Read, Write, BufRead, BufWriter};
use std::path::Path;

// 異なるモジュールから同名の要素(structなど)をimportすることは出来ない
// RustがどちらのResultを使っているか分からないから
/*
use std::io::Result;
use std::fmt::Result;
*/
// 以下のようにimportして、
// io::Result、fmt::Resultのように使う
#[allow(unused_imports)]
use std::io::{Seek}; // std::ioとstd::io::Seekをこのスコープに取り込みたい場合の書き方(selfを使う)

// asを使ってエイリアスを設定することも可能
#[allow(unused_imports)]
use std::fmt as FmtResult;

// globパターン
// パッケージの全ての公開要素を取り込む
// 該当の要素がlocalで定義されたものか、useで外部から取り込んだものか分かりづらくなる
// テストで使うことが多い
#[allow(unused_imports)]
use std::fs::*;

// src直下のファイル名(拡張子抜き)をモジュール名としてimportできる
mod summary;
mod back_of_house;
mod guess;
// mod xxxxでモジュールをimportして、useでモジュール内の要素(structやenumなど)をimportできる
use summary::{Tweet, Summary};
use back_of_house::{BreakFast, Language};
use guess::{Guess};

// 相対パス：呼び出し側と定義側を一緒に移動する可能性が高いならこっち
// use lib::traits::Summary;

// 構造体やenumをimportする時は関数とは違いフルパスで書くのが慣習
// use lib::back_of_house::{BreakFast, Language};

enum MyEnum {
    Variant1,
    Variant2(u32, u32),
    Variant3 { x: u8, y: u8 }
}

fn en(val: MyEnum) -> () {
    match val {
        MyEnum::Variant1 => println!("{}", "this is variant1"),
        MyEnum::Variant2(x, y) => println!("this is variant2 {},{}", x, y),
        MyEnum::Variant3{ x, y } => println!("this is variant3 {},{}", x, y),
    }
}

// 外部パッケージをuse
// cargo add randでCargo.tomlに追加した上でuse
// cargo build か cargo runでrandも一緒にコンパイルされる
use rand::{thread_rng, Rng};
fn foo<R: Rng + ?Sized>(rng: &mut R) -> f32 {
    rng.gen()
}

#[allow(unused_variables)]
fn main() {
    let mut rng = thread_rng();
    let rng_value = foo(&mut rng);
    println!("{}", &rng_value);

    // 関数をimportする際はuseでモジュールだけimportする
    // 使う時はモジュール::関数()のようにして使う
    // こうすることで、localで定義した関数なのかモジュールからimportした関数なのか明確になる
    // このやり方がRustの慣習
    let val = back_of_house::sample_function();

    let lang = "ja";
    use std::str::FromStr;
    let langage_code = Language::from_str(lang);
    // @TODO unwrapを使ったらErrの場合にpanicになるので良くない
    match langage_code.unwrap() {
        Language::Japanese => println!("{}", "日本語！"),
        Language::English => println!("{}", "英語！"),
    }

    let toast = "france";
    let mut bf = BreakFast::summer(toast);
    // bf自体がmutableであり、toastプロパティはpublicなので変更可能
    bf.toast = String::from("chocolate");
    // bf自体がmutableだけど、seasonal_fruitはprivateなので参照も修正も出来ずコンパイルエラーになる
    // bf.seasonal_fruit = String::from("peach");

    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    println!("{}", localhost_v4);
    println!("{}", localhost_v6);

    let list;

    {
        // Box<T>はデータをヒープ領域に置きつつ、参照として機能する
        // Box<T>の値がスコープを抜けるとボックスが参照しているヒープ領域のデータもdropされる
        // Rc<T>は複数の所有者が許容されるため全て不変参照になる
        list = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
        // Rc::cloneはディープコピーではなく参照カウントをインクリメントするだけなのでパフォーマンス上の問題にはならない
        // ディープコピーはデータ量によってはパフォーマンス上のボトルネックになりうる
        // println!("2, {}", Rc::strong_count(&list));
        // let b = Cons(4, Rc::clone(&list));
        // let c = Cons(5, Rc::clone(&list));
        // println!("3, {}", Rc::strong_count(&list));
        // println!("4, {}", Rc::strong_count(&list));
        // let d = Cons(5, Rc::clone(&list));
        // let e = Cons(5, Rc::clone(&list));
        // println!("5, {}", Rc::strong_count(&list));
    }
    println!("6, {}", Rc::strong_count(&list));

    println!("{:p}", &list);
    println!("{:?}", list);

    let s = String::from("Please don't forget it");
    let s1 = &s; // sの参照をs1に借用
    // println!("ここに、{}を出力", s1);
    // println!("ここに、{}を出力", s); // 所有権をmoveしている訳では無いのでここでもsを扱える

    let my_enum_value_3 = MyEnum::Variant3 {
        x: 100, y: 200
    };
    let my_enum_value_1 = MyEnum::Variant1;
    let my_enum_value_2 = MyEnum::Variant2(11, 22);
    en(my_enum_value_1);
    en(my_enum_value_2);
    en(my_enum_value_3);

    // 「文字列スライス」：文字列の一部への参照
    // 0番目から5番目の文字列への参照
    // コンピューターで扱えるデータの最小単位は1byte = 8bit
    // 1byte = 8bitは0 ~ 255
    // ASCII文字は1byteの範囲(0~127)で扱える
    // ASCII以外の文字は128 ~ 255を組み合わせて作られる(マルチバイト文字)
    // UTF-8の場合、マルチバイト文字は2 ~ 4byteの可変長
    // UTF-8の文字境界以外のところで区切った場合はエラーになる
    // let string_slice = &s1[..6];
    // println!("{}", string_slice);

    let word = first_word(&s1[..]);
    // s1.clear(); // s1の一部への参照が存在する(word)のでs1をclearしようとするエラーになる
    println!("{}", word);


    // &[u8; 5]：u8のスライスへの参照
    // let x: &[u8; 5] = b"hello"; // バイトリテラル型. b"hello"は参照
    //println!("b\"hello\" {:#?}", x);

    // xはスコープを抜ける際にdropされる
    // そのため、スコープの外でxの参照を使うことは出来ない
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("{}", r);

    // a, b共に文字列リテラルなのでスタック領域に保持される
    // スコープを抜けるタイミングでdropされることがない
    // そのためライフタイムを意識しなくていい？ = ライフタイムで決定したスコープ外で参照してもコンパイルエラーにならない
    let a: &str = "good night";
    let ret: &str;
    {
        let b: &str = "good afternoon";
        ret = longest(a, b);
    }
    println!("longest sentence is {}", ret);


    // String::fromで生成した文字列はヒープ領域に割り当てられる
    // スコープを抜けるタイミングでdropされるのでライフタイムを意識する必要がある
    // ライフタイムで決定したスコープ外で参照するとコンパイルエラーになる
    let x: String = String::from("hello world");
    let result: &str;
    {
        let y: String = String::from("good morning");
        result = longest(x.as_str(), y.as_str());
        println!("longest sentence is {}", result);
        let mut add: String = result.to_string();
        add.push_str(" and evening and ホゲホゲ");
        println!("add str: {}", add);
    }
    //println!("longest sentence is {}", result);
    println!("{}", hoge());

    // "Katsu"はヒープに格納される
    // bは値"Katsu"を所有しているため、スコープを抜ける際にメモリが解放される
    // 解放されるのはスタックに格納されているBoxとヒープに格納されている"Katsu"のデータ
    // コンパイル時にコンパイラが知っておかねばならないのは、ある型が占有する領域の大きさ
    // 再帰的な型はコンパイル時にサイズが分からない
    let b: Box<&str> = Box::new("Katsu");
    println!("{}", b);

    // RefCell<T>で内部可変化する
    // imutableで定義しても可変参照を持てるようになる
    // 特定の関数内でのみ可変にしたい場合などに使える
    let msgs = Rc::new(RefCell::new(vec![]));
    add_message(&msgs, String::from("hello"));

    msgs.borrow_mut().push(String::from("bbbb"));

    let num = Rc::new(RefCell::new(11));
    // core::cell::Ref<i32>
    println!("{:#}", type_of(num.borrow()));
    // i32
    println!("{:#}", type_of(*num.borrow()));
    // *で参照外しを行うことでi32として演算が可能になる
    *num.borrow_mut() += 10;
    
    // RefMut<T>に対する複数の可変参照を持っているため実行時エラーになる
    // let mut msg2 = msgs.borrow_mut();
    // let mut msg3 = msgs.borrow_mut();
    // msg2.push(String::from("aaaa"));
    // msg3.push(String::from("bbbb"));

    // .borrow()で不変参照を得る
    println!("{:#?}", msgs.borrow());


    struct Sample<T> {
        x: T,
        y: T
    }

    // TがAdd<Output = T>トレイトとCopyトレイトを実装している場合のみ
    // addメソッドを実装する
    // それ以外の型の場合はaddメソッドはない
    impl<T: Add<Output = T> + Copy> Sample<T> {
        fn add(&self) -> T {
            self.x + self.y
        }
    }
    let sample = Sample {
        x: 22.5,
        y: 33.5,
    };
    println!("{}", sample.add());
    println!("{}", sample.add());

    // sample_aのプロパティの型はAddやCopyトレイトを実装していないため
    // addメソッドが存在せず、addメソッドを使おうとしたらコンパイルエラーになる
    // let sample_a = Sample {
    //     x: String::from("hello"),
    //     y: String::from("bye"),
    // };
    // println!("{}", sample_a.add());

    let tweet = Tweet {
        author: String::from("Katsu"),
        text: String::from("I'm very happy! Yeah!"),
    };
    
    println!("{:#}", type_of(&tweet.summarize()));
    
    println!("{}", &tweet.default());

    notify(&tweet);

    // sampleはSummaryトレイトを実装してないのでコンパイルエラー
    // notify(&sample);

    let vec: Vec<u32> = vec![1, 199, 22, 18];
    let largest = largest(&vec);
    println!("{}", largest);
    println!("{:?}", vec);


    // let mut b = &a;とlet b = &mut a;は違う
    // 前者はaに入ったポインタが可変という意味なので、ポインタの値が別のポインタに変わる = 別のアドレスのデータを参照する可能性がある
    // 後者はaを通してbの値自体変わる可能性がある。(その場合はb自体も可変である必要がある)
    let a = String::from("hello");
    let mut b = &a;
    println!("first:{}", b);
    let c = String::from("bye");
    b = &c;
    println!("second:{}", b);
    println!("third:{}", a);

    // Vec<T>
    // Vec<T>は同じ型の値のコレクション
    // コンパイル時にサイズは決まっておらず、ヒープ領域に保持される
    // 生成方法new()かvec![](マクロ)
    // 生成時に値が空の場合は型注釈必須
    let v_new: Vec<String> = Vec::new();
    let mut v_macro = vec![1,12,15]; // 初期値のあるVec<T>を生成する方が一般的
    v_macro.push(24); // 末尾に追加
    v_macro.pop(); // 末尾から取り除く
    v_macro.remove(0); // 指定したインデックスを削除
    v_macro.reverse(); // 順番を逆にする
    println!("{:?}", &v_macro); // [15, 12]

    // Vec<T>の要素を読む
    let first = &v_macro[0]; // 要素が存在しないインデックスにアクセスしようとした場合panic
    let first_get: Option<&i32> = v_macro.get(21); // 要素が存在しないインデックスにアクセスしようとした場合Noneが返る
    match first_get {
        Some(&ele) => println!("get {:?}", &ele),
        None => println!("{}", "No element"),
    }

    // Vec<T>の要素の型がスタックに保持されるような型の場合
    let mut v_borrow = vec![9, 22, 42, 500];
    // v_borrow[1]の値がsecondにコピーされる(Vec<T>の要素がi32で、スタックに保持されるため)
    // secondとv_borrow[1]はそれぞれスタックの別々のアドレスに保持されている
    let second = v_borrow[1];
    // v_borrowの値を書き換えてもsecondの値は書き換わらない
    v_borrow[1] = 100;
    // 以下の2つは異なるアドレスを指す
    println!("{:p}", &v_borrow[1]);
    println!("{:p}", &second);

    let mut v_string = vec![String::from("h"), String::from("a"), String::from("c"), String::from("k")];
    let first_st = &mut v_string[2];
    // v_string[2]のデータはfirst_stに借用中なので変更できない
    // 通常、mutableで借用すれば可変なはずだが、
    // Vec<T>の場合、変更の際に連続したメモリ領域を確保できなかった場合に別の領域を確保し直し、
    // 全く別のアドレスになってしまう可能性があるから
    // その場合、first_stはダングリングポインタになってしまう
    // ↓ そのため、これらはコンパイルエラーになる
    // v_string.push(String::from("ccc"));
    // v_string[2] = String::from("bb");
    println!("{}", first_st);

    // for in 文で内部の要素を走査出来る
    let v_for = vec![40, 22, 499, 211];
    // itemは不変な参照
    for item in &v_for {
        println!("{}", item);
    }

    let mut v_for_mut = vec![40, 22, 499, 211];
    // itemは可変な参照
    for item in &mut v_for_mut {
        *item = 19;
        println!("{}", item);
    }

    // 異なる型のコレクションを保持したい場合はenumを使う
    enum SpreadssheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }
    let row = vec![
        SpreadssheetCell::Int(21),
        SpreadssheetCell::Float(4.4),
        SpreadssheetCell::Text(String::from("hello")),
    ];

    // 配列 = 要素数が固定 = 各要素がメモリに格納され、開始点への「ポインタ」「長さ」「キャパシティ」をarrに持つ
    let arr: [u32; 3] = [22,33,44];
    // 配列の一部or全部への参照 = 開始点のポインタと長さを持つ
    let slice: &[u32] = &arr[1..2];
    // String = ヒープ領域に文字列が格納され、そのメモリへの「ポインタ」、「長さ」、「キャパシティ」をstring_valが持つ
    // String型は伸長可能、可変、所有権を持つ
    // Stringも&strもUTFエンコードされたデータが入る
    let string_val: String = String::from("aaaa");
    // &str = Stringの一部or全部の参照(&Stringの場合も&strの型注釈を付けれるっぽい) = 開始点のポインタと長さを持つ
    let str_val: &str = &string_val;
    let str_val_part: &str = &string_val[1..2];
    let str_literal: &str = "aaaa";

    let hira = String::from("あ");
    // 文字列をバイトスライスに変換
    // "あ" => UTF-8エンコード => E3,81,82(16進数)
    // E3,81,82 => 16進数から10進数に変換 => 227,129,130
    // [227,129,130]がバイトスライス
    let bytes_hira = hira.as_bytes();
    println!("{:?}", bytes_hira);

    // ASCII文字の場合は直接10進数表現と対応している
    let alpha = String::from("a");
    let bytes_alpha = alpha.as_bytes();
    println!("{:?}", bytes_alpha); // [97]

    // 文字列連結
    let mut str_base = String::from("base");
    let str_pushed_literal = " and pushed";
    let str_pushed_real = String::from(" and pushed");
    // push_strは引数にStringではなく&strを渡すため、引数に指定した変数の所有権を奪わない(借用するだけ)
    str_base.push_str(&str_pushed_real);
    // push_strに渡した後も参照できる
    println!("{}", &str_pushed_real);
    println!("{}", &str_base);

    let s11 = String::from("Hello, ");
    let s22 = String::from("world!");
    // + 演算子は裏側でadd(self, s:&str) -> Stringを呼び出している
    // 引数が&strになるので、+ の右は参照にする必要がある
    // addは第１引数でselfの所有権を奪っている
    // そのため、+ の左側は参照ではなく実体が指定されている
    // 処理の内容としては、s11の所有権を奪い、その後に続けてs22の中身のデータをコピーして、出来上がったデータの所有権を返している
    // 2つ目の文字列の参照は&Stringのはずだが、Rustのコンパイラが「参照外し型強制」を行い、&Stringを&strに変換している
    //let s33 = s11 + &s22; // s1はムーブされ、もう使用できないことに注意
    let s44 = String::from("aaa");
    let sa = s11 + &s22 + &s44;
    let sb = s22.add(&sa).add(&s44);

    // 文字列のフォーマット
    let pen = String::from("pen");
    let apple = String::from("apple");
    let pinappo = String::from("pinappo");
    // format!マクロはprintln!とほぼ一緒だが、画面に出力する代わりにフォーマットした文字列を返す
    let pen_pinappo_apple_pen = format!("{}-{}-{}-{}", pen, pinappo, apple, pen);
    println!("{}", pen_pinappo_apple_pen);

    // Stringへの添字アクセスはコンパイルに失敗する
    // StringはVec<u8>のラッパー
    // Stringに添字アクセスした場合、該当のindexが表すのはu8の値(バイトスライスの要素の1つ)
    // 添字でアクセスしたデータが文字として意味のある単位になっていないこともある
    // またStringの添字の場合、処理がO(1)になることを保証できない
    // 文字として意味のある単位かどうかを探すために、最初の要素から走査していく必要があるから
    // let part = &pinappo[2]; // `String` cannot be indexed by `{integer}`

    // 範囲を指定して文字列スライスを作成する場合、
    // 指定した範囲が文字の単位からずれていた場合パニックになる
    let moji = "文字列はバイト値、スカラー値、書記素クラスタで表現できる";
    // let part = &moji[..5]; // 'byte index 5 is not a char boundary; it is inside '字' (bytes 3..6) of `文字`'

    // 文字列を走査するメソッド
    // char型(スカラー値)を取得
    let mut char_arr: Vec<char> = vec![];
    for item in moji.chars() {
        // 日本語が1文字ずつchar_arrに入る
        // ただUnicodeスカラー値には2バイト以上から成るものもあるため、
        // 綺麗に意図通りに書記素が取得できるとは限らない
        // 標準ライブラリにはないのでクレートを探す必要がある
        char_arr.push(item);
    }
    println!("{:?}", &char_arr);

    // バイト値(u8 = 8bit = 1byte)
    let mut byte_arr: Vec<u8> = vec![];
    for item in moji.bytes() {
        byte_arr.push(item);
    }
    // バイト(10進数)配列
    println!("{:?}", &byte_arr);

    use std::collections::HashMap;
    // HashMapのキー、値はそれぞれ全て同じ型でなければならない
    // Vec同様に値はヒープに保持される
    let mut hash: HashMap<String, i32> = HashMap::new();
    hash.insert(String::from("sha"), 256);
    hash.insert(String::from("RSA"), 4096);

    // 2つのベクタの要素をそれぞれキーと値にしてHashMapを生成する
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![98, 74];
    // .zipで２つのベクタからタプルのベクタを生成[("Blue", 98), ("Red", 74)]
    // .collectでHashMapを生成
    // 型注釈の<_,_>の型はコンパイル時にベクタの値の型からコンパイルが型推論する
    let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
    // println!("{:?}", &scores);

    // HashMapに値が渡される場合、
    // Copyトレイトを実装している型はコピーされ、そうでない場合は所有権がムーブされる
    let first_name = String::from("konan");
    let last_name = String::from("edogawa");
    let mut full_name: HashMap<String, String> = HashMap::new();
    // String型なので所有権がムーブされる
    full_name.insert(first_name, last_name);
    // 所有権がムーブされているので以下はコンパイルエラー
    // println!("{}", first_name);

    // .getでOption型が返る
    let name: Option<&String> = full_name.get("konan");
    match name {
        Some(ele) => println!("{:?}", ele),
        None => println!("no name"),
    }

    for (key, value) in &full_name {
        println!("{}", &value);
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 値の更新
    // 上書き
    full_name.insert("konan".to_string(), "yodogawa".to_string());
    println!("{:?}", &full_name);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // キーが存在したら無視、キーが存在しない場合はinsert
    scores.entry(String::from("Yellow")).or_insert(50); // .entryは新しい値への可変参照を返す
    scores.entry(String::from("Blue")).or_insert(50); // .entryは既存の値への可変参照を返す
    println!("{:?}", scores); // {"Yellow": 50, "Blue": 10}

    let text = "Hello everyone. Please be quiet! Please! Please!";
    let mut map: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        // キーwordに対応する値への可変参照をcountに保持
        let count = map.entry(word).or_insert(0);
        // countの参照先の値をインクリメント
        *count += 1;
    }
    println!("{:?}", map);

    // 平均値
    let mut vec_int: Vec<i32> = vec![10, 105, 1, 1, 1, 223, 45];
    let mut sum: f64 = 0.0;
    for val in &vec_int {
        sum += f64::from(*val);
    }
    let mean: f64 = sum / vec_int.len() as f64;
    println!("{}", &mean);

    // 中央値
    vec_int.sort();
    println!("{:?}", &vec_int);
    #[allow(unused_assignments)]
    let mut center: usize = 0;
    if let 0 = vec_int.len() % 2 {
        center = vec_int.len() / 2 - 1;
    } else {
        center = vec_int.len() / 2;
    }
    let median = vec_int.get(center);
    println!("{:?}", median);

    // 最頻値
    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    // Vec<T>をインデックス付きでループするには.iter().enumerate()でkey,valueのタプルを取得できる
    for (key, val) in vec_int.iter().enumerate() {
        // key, valはforループ内でのみ有効なローカル変数なので、
        // forループ内で他で使わなければ所有権渡しちゃってもOK
        let count = hash_map.entry(*val).or_insert(0);
        // countは可変参照なので、インクリメントすることでhash_mapの値が実際に更新される
        *count += 1;
    }
    let mut max = 0;
    let mut mode = 0;
    for (k, v) in &hash_map {
        if v > &max {
            max = *v;
            // 最頻値を更新
            mode = *k;
        }
    }
    println!("{:?}", &mode);

    // ピッグ・ラテン
    let a = "penpen";
    let b = "apple";
    println!("{:?}", pig_latin_ascii(&a));
    println!("{:?}", pig_latin_ascii(&b));

    let mut departments: HashMap<&String, Vec<String>> = HashMap::new();
    let department_name = String::from("技術部");
    let exist_members = vec![String::from("Aさん"), String::from("Bさん")];
    departments.insert(&department_name, exist_members);
    
    // Option<T>内のSome(T)は不変参照になる match xxx { Some(&ele) => println!("{}", ele) }
    // そのため、取得したOptionからSomeの値を得て、その値を更新したい場合は、
    // .clone()で値をメモリの別領域にコピーしてから編集を行う必要がある
    let mut mems = departments.get(&department_name).unwrap().clone();
    mems.push(String::from("Cさん"));
    departments.insert(&department_name, mems);
    println!("{:?}", departments);

    let vec = vec![2,11];
    // 値の存在しないindexにアクセスしようとするとpanic
    // Cなどの他言語では該当の箇所のメモリを読みにいこうとする(バッファオーバーリード)ものがある
    // 攻撃者が配列の後ろにある、読めるべきでないデータを読めるよう添字を操作できたらセキュリティ脆弱性に繋がる可能性もある
    // vec[99];

    // panicではなくResultが返る例
    let file_path = "hello.txt";
    // let f: Result<File, Error> = File::open(file_path);
    // let f = match f {
    //     Ok(file) => {
    //         println!("{:?}", file);
    //     },
    //     Err(err) => {
    //         panic!("can't open the file :{}, error: {:?}", file_path, err);
    //     }
    // };

    let f2 = File::open(file_path);
    let f2 = match f2 {
        Ok(file) => file,
        // マッチガード
        // refはerrが条件式にムーブしないように必要
        // refをつけると値にマッチしてその値への参照を返す(変数に入れる)
        // &をつけると参照にマッチして値を返す(変数に入れる)
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            // ErrorKind::NotFoundにマッチした場合、ファイルを作成する
            match File::create(file_path) {
                Ok(file_created) => file_created,
                Err(err) => {
                    panic!("Tried to create file {:?} but there was a problem: {:?}", file_path, err);
                }
            }
        },
        Err(err) => {
            panic!("can't open the file {:?}, error: {:?}", file_path, err);
        }
    };
    println!("{:?}", f2);

    // unwrap()
    // エラーの場合panic!マクロを呼ぶ
    // let f3 = File::open("world.txt").unwrap();

    // expect()
    // エラーの場合メッセージを指定してpanic!マクロを呼ぶ
    // let err_message = format!("can't open the file {:?}", file_path);
    // let f4 = File::open("test.txt").expect(&err_message);

    // ?はエラーの場合にErr(x)をreturnするため、
    // std::ops::Tryトレイト(Result、Optionなど)を実装した型を返す関数内でしか使えない
    // let f5 = File::open("test.txt")?;

    let guess = Guess::new(20);
    println!("{:?}", guess.value());

    // valueはprivateなので、以下はコンパイルエラーになる
    // これにより、new関数を通してしかGuessインスタンスを生成できない
    // let guess2 = Guess {
    //     value: 22,
    // };

    // match file_create() {
    //     // Result<T,E>はOk(T)かErr(E)を返す
    //     // Err(err)のerr変数にはE型の値が入る
    //     Ok(()) => println!("success"),
    //     Err(err) => println!("failed: {}", err)
    // }

    // match file_read() {
    //     Ok(content) => println!("content is {:?}", content),
    //     Err(e) => println!("failed: {:?}", e)
    // }

    // match file_append() {
    //     Ok(()) => println!("append success"),
    //     Err(e) => println!("failed: {:?}", e)
    // }

    match file_read_use_buf_reader() {
        Ok(()) => println!("read_line success"),
        Err(e) => println!("read_line fail")
    }

    match file_write_use_buf_writer() {
        Ok(()) => println!("write success"),
        Err(e) => println!("write fail")
    }
}

// BufReader <R>は、同じファイルまたはネットワークソケット(まだメモリに無いデータ)に対して、
// 小さく繰り返し読み取りを行うプログラムの速度を向上させる
// 一度に大量のデータを読んだり、読み取り頻度の低い場合は利点はない
// Vec<u8>のように既にメモリ内にあるソースから読み込む場合も利点はない
use std::io::BufReader;
fn file_read_use_buf_reader() -> io::Result<()> {
    let f = File::open("Cargo.toml")?;
    let mut reader = BufReader::new(f);
 
    let mut line = String::new();
    let mut len: usize;
    loop {
        // read_lineは全てのbyteをnewline(0xA)が来るまで取得し、
        // 引数に与えられたbufferに保持する
        // read_lineを呼ぶごとに１行ずつ処理が進んでいく
        // 残りの行数がlenに入る
        len = reader.read_line(&mut line)?;
        if len == 0 {
            break;
        }
    }
    println!("{}", line);
    Ok(())
}

fn file_write_use_buf_writer() -> Result<(), io::Error> {
    let f = OpenOptions::new().append(true).open("zero.txt")?;
    let mut writer = BufWriter::new(f);
    let texts: Vec<&str> = vec![
        "aaaaa\n",
        "bbbbb\n",
        "ccccc\n"
    ];
    for s in texts {
        // 書き込みの度にシステムコールが実行されるのではなく、
        // メモリバッファに保持された上で、
        // writer.flushのタイミングで１回だけシステムコールが呼ばれる
        writer.write(s.as_bytes()).unwrap();
    }
    writer.flush()
}

fn file_append() -> Result<(), Error> {
    println!("Enter file name");

    // 指定した名前でファイルを開く
    let mut name = String::new();
    // 標準入力を値をname変数にいれる
    io::stdin().read_line(&mut name)?;

    // append(true)にすることで編集可能になる
    // write(truw)の場合は上書き、append(true)の場合は追加
    // File::openやFile::createは、OpenOptionsの色んな設定をラップしている
    let mut file = OpenOptions::new().append(true).open(name.trim())?;

    println!("Enter append content");
    let mut content = String::new();
    io::stdin().read_line(&mut content)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

// 入力値からファイルを作成
fn file_create() -> Result<(), Error> {
    println!("Enter file name");
    
    // 指定した名前でファイル作成
    let mut name = String::new();
    // 標準入力を値をname変数にいれる
    io::stdin().read_line(&mut name)?;
    if Path::new(name.trim()).exists() {
        panic!("path have already exist: {:?}", name.trim());
    }
    let mut file = File::create(name.trim())?;

    println!("Enter content of file");

    // ファイルに書き込み
    let mut content = String::new();
    io::stdin().read_line(&mut content)?;
    // write_allメソッドにはバイト列を渡す
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn file_read() -> Result<String, Error> {
    println!("Enter file name");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename)?;
    let mut content = String::new();
    // 標準入力から値を受け取る際に空白文字が入ってしまうので.trim()が必要
    File::open(filename.trim())?.read_to_string(&mut content)?;
    Ok(content)
}

// エラー処理を上位の関数に移譲する
// Result型を返したい場合は、Ok(x)かErr(x)を返却するようにすればいい
#[allow(dead_code)]
fn read_username_from_file(file_path: &str) -> Result<String, io::Error> {
    let f = File::open(file_path);
    let mut file = match f {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut buf = String::new();
    match file.read_to_string(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(e)
    }
}

// Resultを返す場合、「?」で代用できる
// Result値を返す式の後ろに?をつけることで、
// 成功時はOk()、失敗時はErr()を返す
#[allow(dead_code)]
fn read_username_from_file_short(file_path: &str) -> Result<String, io::Error> {
    let mut f = File::open(file_path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    // read_to_stringは引数にデータを入れる
    // そのためResultとして返す場合は最後にOk(buf)とする必要がある
    Ok(buf)
}

// メソッド連結版
// ?をつけた場合、matchと異なり、返却する型をシグネチャで定義した型に変換してくれる(Fromトレイトを実装しているから)
// 関数内の処理が複数の理由で失敗する可能性があるのに、1つの型で返却する必要がある場合に使える
#[allow(dead_code)]
fn read_username_from_file_very_short(file_path: &str) -> Result<String, io::Error> {
    let mut buf = String::new();
    File::open(file_path)?.read_to_string(&mut buf)?;
    Ok(buf)
}

fn pig_latin_ascii(text: &str) -> String {
    let mut ret = text.to_string();
    // .collect()でVec<T>を生成
    let vec_char: Vec<char> = ret.chars().collect();
    let first: char = vec_char[0];

    // 文字列に特定のcharを追加
    ret.push('-');
    
    // Vec内に特定の要素が存在するか検証
    let vowels: Vec<char> = vec!['a', 'i', 'u', 'e', 'o'];
    if vowels.contains(&first) == true {
        ret.push_str("hay");
    } else {
        // 文字列から特定の文字(char)を削除する
        ret.retain(|c| c != first);
        ret.push(first);
        ret.push_str("ay");
    }
    ret
}

mod rectangle;

// cfgアトリビュートは、環境に応じたコンパイルを行うためのもの
// cfg(test)はテスト環境
// cfg(target_os = "linux")はosがlinuxの場合のみコンパイルする
// cfg(not(target_os = "linux"))はosがlinuxでない場合のみコンパイルする
// if cfg!(target_os = "linux") {} のようにマクロも使える
#[cfg(test)]
mod tests {    
    use super::*;
    use super::rectangle::Rectangle;
    use super::guess::{Guess};

    // panicが起こることをテストする
    // expected引数でpanicで出力するメッセージを検証できる
    #[test]
    #[should_panic(expected = "too big number: 199")]
    fn test_guess_greater_than_100() {
        Guess::new(199);
    }

    #[test]
    #[should_panic(expected = "too small number: 0")]
    fn test_guess_smaller_than_1() {
        Guess::new(0);
    }

    // 各テストは新規スレッドで実行される
    // メインスレッドがテストスレッドが死んだと確認した時 = panicが発生しプログラムが停止した時？
    // テストは失敗したと位置付けられる
    #[test]
    fn it_works() {
        // assert_eq!は、2つの引数が同一でない場合内部でpanic!が実行されている
        assert_eq!(2+2, 4);
    }

    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // use std::fs::File;
    // use std::io::{self, Read, Write, BufReader};
    // #[test]
    // fn test_write_file() -> io::Result<()> {
    //     let file_path: &str = "hello.txt";
    //     let text: String = String::from("baba");
    //     let buf = BufReader::new(text.as_bytes()).bytes().collect::<io::Result<Vec<u8>>>()?;
    //     // createはwrite-only、openはread-only
    //     let mut file = File::create(file_path)?;
    //     file.write_all(&buf)?;
    //     file.flush()?;
    //     Ok(())
    // }

    #[test]
    fn test_file_open() -> io::Result<()> { // io::Result<T> = result::Result<T, Error>
        let file_path = "hello.txt";
        let mut buf: String = String::new();
        // ?をつけることで失敗したら自動でErrorが返る
        // テスト内で何らかの工程がErrヴァリアントを返したときに失敗するべきテストを書くのに便利
        File::open(file_path)?.read_to_string(&mut buf)?;
        // bufにファイルの中身が入るのでここでテスト
        assert_eq!(buf, String::from("file for test"));
        Ok(())
    }

    // #[test]
    // fn test_fail() {
    //     panic!("このテストは失敗する");
    // }

    #[test]
    fn test_type_of() {
        let val: &str = "hello";
        let type_name: String = type_of(val);
        assert_eq!(type_name, String::from("&str"));

        let val2: String = String::from("wei");
        let type_name_2: String = type_of(val2);
        // assert_eq!、assert_ne!は2つの引数の順番は関係ない
        // leftとrightが等しいか(等しくないか)を判定する
        assert_eq!(String::from("alloc::string::String"), type_name_2);
        assert_ne!(String::from("&str"), type_name_2);
    }

    #[test]
    fn test_large_can_hold_small() {
        let large = Rectangle::new(22, 44);
        let small = Rectangle::new(11, 33);
        assert!(large.can_hold(&small));
    }

    #[test]
    fn test_not_equal_rectangles() {
        let large = Rectangle::new(22, 44);
        let small = Rectangle::new(11, 33);
        // largeとsmallが異なることをアサート
        assert_ne!(large, small);
    }

    #[test]
    fn test_equal_rectangles() {
        let left = Rectangle::new(11, 33);
        let right = Rectangle::new(11, 33);
        // leftとrightが同一であることをアサート
        // 第３引数以降でdebug用のカスタムメッセージを設定できる
        // カスタムメッセージは内部でformat!マクロに渡される
        assert_eq!(left, right, "left: {:?} and right: {:?} were not equal", left, right);
    }

    #[test]
    fn test_small_can_not_hold_large() {
        let large = Rectangle::new(22, 44);
        let small = Rectangle::new(11, 33);
        assert!(!small.can_hold(&large));
    }

}

// Summaryトレイトを実装したインスタンス(の参照)のみ受け付ける
fn notify(item: &impl Summary) {
    println!("Summary is {:?} for me", item.summarize());
}

// トレイト境界を使った書き方
#[allow(dead_code)] // 勉強用に描いただけなのでdead_codeを許容
fn notify_boundary<T: Summary>(item: &T) {
    println!("Summary is {:?} for me", item.summarize());
}

// whereを使った書き方
#[allow(dead_code)] // 勉強用に描いただけなのでdead_codeを許容
fn notify_where<T>(item: &T) 
    where T: Summary
{
    println!("Summary is {:?} for me", item.summarize());
}

// 戻り値に特定のトレイトを実装した型を指定
#[allow(dead_code)]
fn return_summarize() -> impl Summary {
    Tweet {
        author: String::from("Katsu"),
        text: String::from("I am implemented Summary"),
    }
}


// <_: T>引数は不要だけど引数の型だけ使いたい場合のジェネリクス
fn type_of<T>(_: T) -> String{
    let a = type_name::<T>();
    return a.to_string();
}


fn add_message(msgs: &Rc<RefCell<Vec<String>>>, msg: String) {
    // borrow_mut()でRefCell<T>の内部の値(Vec<String>)への可変参照を得る
    // borrow_mut(), borrow()はそれぞれRefMut<T>, Ref<T>型の値を返す
    // どちらもDerefトレイトを実装しているため参照のように扱える
    msgs.borrow_mut().push(msg);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     // bytes.iter()でイテレータを返すことでitemを参照できる
//     // bytes.iter().enumerate()によりイテレータの各要素をラップして添字iと&itemのタプルを取り出せるようになる
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i
//         }
//     }
//     s.len()
// }

// 文字列スライス(&str)を返す
// &strは不変な参照
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    // bytes.iter()でイテレータを返すことでitemを参照できる
    // bytes.iter().enumerate()によりイテレータの各要素をラップして添字iと&itemのタプルを取り出せるようになる
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}

// 'a：ジェネリックなライフタイム引数
// <>で与えられたライフタイムが各引数と戻り値のライフタイムに設定されている
// 引数と戻り値のライフタイムが同じことを表している
// <'a>に渡されるライフタイムは、xかyのライフタイムの短い方になる
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// largest関数はT型のスライスを受け取る
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    // スライスの一部の値をコピーする = largestのデータがスタックにコピーされる(コピートレイトを実装してるため)
    let mut largest = list[0];

    // itemにはlistの各値が入る(参照ではなく実態がスタックにコピーされる)
    // スタックに積まれている実体同士を比較するためにitemではなく&itemにしている
    // &itemにすることによりitemには実体が入る
    for &item in list {
        // スタックに存在する値同士を比較
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[allow(dead_code)]
fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    // list[0]の参照をlargestに代入 = 所有権はムーブしない
    // largestはlist[0]へのポインタを保持しており、largestはmutableになっている
    // = largestのポインタは書き換え可能なので、別のポインタが入ることができる
    // = largestが可変だからといって、list[0]が可変な訳ではない
    let mut largest = &list[0];

    // listの各要素の参照がitemに入る
    // itemはimmutable
    for item in list {
        // 参照している値同士を比較
        if item > largest {
            // largestに入っているポインタの値を
            // itemに入っているポインタの値で上書きする
            // largestはmutableなので可能
            largest = item;
        }
    }
    // largestが指している値をcloneして返却
    // cloneしているのでlargestが指しているデータの所有権はムーブしない
    largest.clone()
}

#[allow(dead_code)]
fn largest_reference<T: PartialOrd>(list: &[T]) -> &T {
    // largestには参照が入る
    let mut largest = &list[0];

    // itemにも参照が入る
    for item in list {
        // 参照先の値同士を比較
        if item > largest {
            // 参照に入っているポインタの値を上書き
            // 実態に影響はない
            largest = item;
        }
    }
    // 参照をそのまま返す
    largest
}



// ローカル変数の参照を返すことは出来ない
// なぜなら、参照の方がローカル変数の実体より長く生きてしまうから
// ローカル変数を生成してそれをreturnするなら参照ではなく実体をreturnして所有権ごと渡してしまった方がいい
// この関数自体、return後にローカル変数を使うことが無いため、所有権を渡しても問題ない
fn hoge() -> String {
    let hoge = "hello".to_string();
    hoge
}



// pub struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// // implキーワードの横の<'a>でライフタイムを定義
// impl<'a> ImportantExcerpt<'a> {
//     // 戻り値のライフタイムは&selfのライフタイムになる
//     fn level(&self, announcement: &str) -> &str {

//         // 'staticライフタイムはプログラム全体の期間を表す
//         // 文字列リテラルはそのままバイナリに埋め込まれる(スタック領域)ため'staticになる
//         let literal: &'static str = "literal has static lifetime";
//         println!("{}", literal);

//         println!("{}", announcement);
//         self.part
//     }
// }