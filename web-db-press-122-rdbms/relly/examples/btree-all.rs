use anyhow::Result;
use relly::{
    btree::{BTree, SearchMode},
    buffer::{BufferPool, BufferPoolManager},
    disk::{DiskManager, PageId},
};

fn main() -> Result<()> {
    let disk = DiskManager::open("simple.rly")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    let btree = BTree::new(PageId(0));

    let mut iter = btree.search(&mut bufmgr, SearchMode::Start)?;
    while let Some((key, value)) = iter.next(&mut bufmgr)? {
        println!("{:02x?} = {:02x?}", key, value);
    }
    Ok(())
}
