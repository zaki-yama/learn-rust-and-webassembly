use anyhow::Result;
use relly::{
    buffer::{BufferPool, BufferPoolManager},
    disk::{DiskManager, PageId},
    table::SimpleTable,
};
fn main() -> Result<()> {
    // 1. バッファプールとバッファプールマネージャの初期化
    let disk = DiskManager::open("simple.rly")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    // 2. テーブルのスキーマ定義
    // プライマリキーは左端の1列だけ、meta_page_id は仮の値
    let mut table = SimpleTable {
        meta_page_id: PageId::INVALID_PAGE_ID,
        num_key_elems: 1,
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
