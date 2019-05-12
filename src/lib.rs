pub mod first;
pub mod second; //この行を追加

// SortOrderを列挙型として定義する
pub enum SortOrder {
    // SortOrderには2つのバリアント型がある。
    Ascending, // 昇順
    Descending, // 降順 
}
