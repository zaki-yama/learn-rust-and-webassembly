use anyhow::Result;

use crate::btree::{self, BTree, SearchMode};
use crate::buffer::BufferPoolManager;
use crate::disk::PageId;
use crate::tuple;

pub type Tuple = Vec<Vec<u8>>;
pub type TupleSlice<'a> = &'a [Vec<u8>];

pub enum TupleSearchMode<'a> {
    Start,
    Key(&'a [&'a [u8]]),
}

impl<'a> TupleSearchMode<'a> {
    fn encode(&self) -> SearchMode {
        match self {
            TupleSearchMode::Start => SearchMode::Start,
            TupleSearchMode::Key(tuple) => {
                let mut key = vec![];
                tuple::encode(tuple.iter(), &mut key);
                SearchMode::Key(key)
            }
        }
    }
}

pub struct ExecSeqScan<'a> {
    table_iter: btree::Iter,
    while_cond: &'a dyn Fn(TupleSlice) -> bool,
}

pub trait Executor {
    fn next(&mut self, bufmgr: &mut BufferPoolManager) -> Result<Option<Tuple>>;
}

impl<'a> Executor for ExecSeqScan<'a> {
    fn next(&mut self, bufmgr: &mut BufferPoolManager) -> Result<Option<Tuple>> {
        let (pkey_bytes, tuple_bytes) = match self.table_iter.next(bufmgr)? {
            Some(pair) => pair,
            None => return Ok(None),
        };
        let mut pkey = vec![];
        tuple::decode(&pkey_bytes, &mut pkey);
        if !(self.while_cond)(&pkey) {
            return Ok(None);
        }
        let mut tuple = pkey;
        tuple::decode(&tuple_bytes, &mut tuple);
        Ok(Some(tuple))
    }
}

pub type BoxExecutor<'a> = Box<dyn Executor + 'a>;
/// 実行計画のノード
pub trait PlanNode {
    fn start(&self, bufmgr: &mut BufferPoolManager) -> Result<BoxExecutor>;
}

/// プライマリキーで範囲検索するための実行計画のノード
pub struct SeqScan<'a> {
    pub table_meta_page_id: PageId,
    pub search_mode: TupleSearchMode<'a>,
    pub while_cond: &'a dyn Fn(TupleSlice) -> bool,
}

impl<'a> PlanNode for SeqScan<'a> {
    fn start(&self, bufmgr: &mut BufferPoolManager) -> Result<BoxExecutor> {
        let btree = BTree::new(self.table_meta_page_id);
        let table_iter = btree.search(bufmgr, self.search_mode.encode())?;
        Ok(Box::new(ExecSeqScan {
            table_iter,
            while_cond: self.while_cond,
        }))
    }
}

/// 不要な行をフィルタする種類の実行計画のノード
pub struct Filter<'a> {
    pub inner_plan: &'a dyn PlanNode,
    pub cond: &'a dyn Fn(TupleSlice) -> bool,
}

impl<'a> PlanNode for Filter<'a> {
    fn start(&self, bufmgr: &mut BufferPoolManager) -> Result<BoxExecutor> {
        let inner_iter = self.inner_plan.start(bufmgr)?;
        Ok(Box::new(ExecFilter {
            inner_iter,
            cond: self.cond,
        }))
    }
}

pub struct ExecFilter<'a> {
    inner_iter: BoxExecutor<'a>,
    cond: &'a dyn Fn(TupleSlice) -> bool,
}

impl<'a> Executor for ExecFilter<'a> {
    fn next(&mut self, bufmgr: &mut BufferPoolManager) -> Result<Option<Tuple>> {
        loop {
            // 一致する行が出るまでループを回す
            match self.inner_iter.next(bufmgr)? {
                Some(tuple) => {
                    // 条件に一致していたら返す
                    if (self.cond)(&tuple) {
                        return Ok(Some(tuple));
                    }
                }
                None => return Ok(None),
            }
        }
    }
}

/// セカンダリインデックスを使ってテーブル検索する実行計画に対応するクエリエクスキュータ
pub struct ExecIndexScan<'a> {
    // 2回目の検索で使うテーブルのツリー
    table_btree: BTree,
    // 1回目の検索で使うセカンダリインデックスのツリーのイテレータ
    index_iter: btree::Iter,
    while_cond: &'a dyn Fn(TupleSlice) -> bool,
}

impl<'a> Executor for ExecIndexScan<'a> {
    fn next(&mut self, bufmgr: &mut BufferPoolManager) -> Result<Option<Tuple>> {
        // セカンダリインデックスからセカンダリキーとプライマリキーを取り出す
        let (skey_bytes, pkey_bytes) = match self.index_iter.next(bufmgr)? {
            Some(pair) => pair,
            None => return Ok(None),
        };
        let mut skey = vec![];
        tuple::decode(&skey_bytes, &mut skey);
        if !(self.while_cond)(&skey) {
            return Ok(None);
        }

        let mut table_iter = self
            .table_btree
            .search(bufmgr, SearchMode::Key(pkey_bytes))?;
        let (pkey_bytes, tuple_bytes) = table_iter.next(bufmgr)?.unwrap();
        let mut tuple = vec![];
        tuple::decode(&pkey_bytes, &mut tuple);
        tuple::decode(&tuple_bytes, &mut tuple);
        Ok(Some(tuple))
    }
}

pub struct IndexScan<'a> {
    pub table_meta_page_id: PageId,
    pub index_meta_page_id: PageId,
    pub search_mode: TupleSearchMode<'a>,
    pub while_cond: &'a dyn Fn(TupleSlice) -> bool,
}

impl<'a> PlanNode for IndexScan<'a> {
    fn start(&self, bufmgr: &mut BufferPoolManager) -> Result<BoxExecutor> {
        let table_btree = BTree::new(self.table_meta_page_id);
        let index_btree = BTree::new(self.index_meta_page_id);
        let index_iter = index_btree.search(bufmgr, self.search_mode.encode())?;
        Ok(Box::new(ExecIndexScan {
            table_btree,
            index_iter,
            while_cond: self.while_cond,
        }))
    }
}
