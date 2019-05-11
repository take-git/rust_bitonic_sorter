// pubはこのsort関数が他のモジュールからアクセスできることを示す。
// 引数xの型&mut[u32]について
// &はポインタ経由で借用することを示す
// mutは値が変更可能であることを示す

// u32型は32ビット符号なし整数
// [u32]型はu32のスライス（１次元の配列のようなもの）
pub fn sort(x: &mut [u32], up: bool) {
    //未実装の意味。コンパイルは徹が、実行するとpanicする。
    unimplemented!();
}

fn sub_sort(x: &mut [u32], up: bool) {
    unimplemented!();
}

fn compare_and_swap(x: &mut [u32], up: bool) {
    unimplemented!();
}
