use std::vec;

use crate::{btree::BTree, buffer::BufferPoolManager, disk::PageId, tuple};
use anyhow::Result;

pub struct SimpleTable {
    pub meta_page_id: PageId,
    // 左からいくつの列がプライマリキーなのか
    pub num_key_elems: usize,
}

impl SimpleTable {
    pub fn create(&mut self, bufmgr: &mut BufferPoolManager) -> Result<()> {
        let btree = BTree::create(bufmgr)?;
        self.meta_page_id = btree.meta_page_id;
        Ok(())
    }

    pub fn insert(&self, bufmgr: &mut BufferPoolManager, record: &[&[u8]]) -> Result<()> {
        let btree = BTree::new(self.meta_page_id);
        let mut key = vec![];
        tuple::encode(record[..self.num_key_elems].iter(), &mut key);
        let mut value = vec![];
        tuple::encode(record[self.num_key_elems..].iter(), &mut value);
        btree.insert(bufmgr, &key, &value)?;
        Ok(())
    }
}

/// セカンダリインデックス (ユニークインデックス)
#[derive(Debug)]
pub struct UniqueIndex {
    pub meta_page_id: PageId,
    // セカンダリキーに含める列を指定するフィールド。複数指定できるようにVec
    pub skey: Vec<usize>,
}

impl UniqueIndex {
    pub fn create(&mut self, bufmgr: &mut BufferPoolManager) -> Result<()> {
        let btree = BTree::create(bufmgr)?;
        self.meta_page_id = btree.meta_page_id;
        Ok(())
    }

    pub fn insert(
        &self,
        bufmgr: &mut BufferPoolManager,
        // エンコードされたプライマリキー
        pkey: &[u8],
        record: &[impl AsRef<[u8]>],
    ) -> Result<()> {
        let btree = BTree::new(self.meta_page_id);
        let mut skey = vec![];
        // セカンダリキーをエンコード
        tuple::encode(
            self.skey.iter().map(|&index| record[index].as_ref()),
            &mut skey,
        );
        // キー: セカンダリキー, バリュー: プライマリキー
        btree.insert(bufmgr, &skey, pkey)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Table {
    pub meta_page_id: PageId,
    // 左からいくつの列がプライマリキーなのか
    pub num_key_elems: usize,
    pub unique_indices: Vec<UniqueIndex>,
}

impl Table {
    pub fn create(&mut self, bufmgr: &mut BufferPoolManager) -> Result<()> {
        let btree = BTree::create(bufmgr)?;
        self.meta_page_id = btree.meta_page_id;

        for unique_index in &mut self.unique_indices {
            unique_index.create(bufmgr)?;
        }
        Ok(())
    }

    pub fn insert(&self, bufmgr: &mut BufferPoolManager, record: &[&[u8]]) -> Result<()> {
        let btree = BTree::new(self.meta_page_id);
        let mut key = vec![];
        tuple::encode(record[..self.num_key_elems].iter(), &mut key);
        let mut value = vec![];
        tuple::encode(record[self.num_key_elems..].iter(), &mut value);
        btree.insert(bufmgr, &key, &value)?;

        for unique_index in &self.unique_indices {
            unique_index.insert(bufmgr, &key, record)?;
        }
        Ok(())
    }
}
