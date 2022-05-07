use anyhow::Result;
use relly::{
    buffer::{BufferPool, BufferPoolManager},
    disk::{DiskManager, PageId},
    table::{Table, UniqueIndex},
};

/* CREATE TABLE
  |id    |first_name|last_name|
  |------|----------|---------|
  |z     |Alice     |Smith    |
  |x     |Bob       |Johnson  |
  |y     |Charlie   |Williams |
  |w     |Dave      |Miller   |
  |v     |Eve       |Brown    |
*/
fn main() -> Result<()> {
    // 1. バッファプールとバッファプールマネージャの初期化
    let disk = DiskManager::open("table.rly")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    // 2. テーブルのスキーマ定義
    let mut table = Table {
        meta_page_id: PageId::INVALID_PAGE_ID,
        num_key_elems: 1,
        unique_indices: vec![UniqueIndex {
            meta_page_id: PageId::INVALID_PAGE_ID,
            skey: vec![2],
        }],
    };
    // スキーマをもとにテーブルを作成
    table.create(&mut bufmgr)?;

    // テーブルに行を追加
    table.insert(&mut bufmgr, &[b"z", b"Alice", b"Smith"])?;
    table.insert(&mut bufmgr, &[b"x", b"Bob", b"Johnson"])?;
    table.insert(&mut bufmgr, &[b"y", b"Charlie", b"Williams"])?;
    table.insert(&mut bufmgr, &[b"w", b"Dave", b"Miller"])?;
    table.insert(&mut bufmgr, &[b"v", b"Eve", b"Brown"])?;

    bufmgr.flush()?;
    Ok(())
}
