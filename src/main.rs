fn main() {
    println!("Hello Rust!");

    // 束縛：let
    //    変数 -> 所有権：オブジェクト
    let a = object;
    let a: i32 = 0;

    // 参照：仮の所有権を作成
    let a = &object;

    // 可変性
    let a = object; // 変数a -> 原本 不変、所有権：オブジェクト
    let mut a = object; // 変数a -> 原本 可変、所有権：オブジェクト
    let a = &object; // 変数a -> 仮 不変、所有権：オブジェクト
    let a = &mut object; // 変数a -> 仮 可変、所有権：オブジェクト

    // 仮の所有権から原本の所有権は作れない
    let a = object; // 変数a -> 原本 所有権：オブジェクト
    let b = &a; // 変数b -> 仮 所有権
    let c = b; // 変数c -> 仮 所有権
    let d = &c; // 変数d -> 仮 所有権
}
