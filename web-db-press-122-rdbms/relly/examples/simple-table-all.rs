use anyhow::Result;
use relly::{
    btree::{BTree, SearchMode},
    buffer::{BufferPool, BufferPoolManager},
    disk::{DiskManager, PageId},
    tuple,
};

fn main() -> Result<()> {
    let disk = DiskManager::open("simple.rly")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    let btree = BTree::new(PageId(0));

    let mut iter = btree.search(&mut bufmgr, SearchMode::Start)?;
    while let Some((key, value)) = iter.next(&mut bufmgr)? {
        // 行の内容を保持するバッファ
        let mut record = vec![];
        // プライマリキーの列をバッファに追加
        tuple::decode(&key, &mut record);
        // プライマリキー以外の列をバッファに追加
        tuple::decode(&value, &mut record);
        // 読みやすくして出力
        println!("{:?}", tuple::Pretty(&record));
    }
    Ok(())
}
