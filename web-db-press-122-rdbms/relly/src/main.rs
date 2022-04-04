use std::{fs::{File, OpenOptions}, io, path::Path};

pub struct PageId(pub u64);
pub struct DiskManager {
    // ヒープファイルのファイルディスクリプタ
    heap_file: File,
    // 採番するページIDを決めるカウンタ
    next_page_id: u64,
}

impl DiskManager {
    // コンストラクタ
    pub fn new(data_file: File) -> io::Result<Self> {
        // (省略)
    }

    // ファイルパスを指定して開く
    pub fn open(heap_file_path: impl AsRef<Path>) -> io::Result<Self> {
        let heap_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(heap_file_path)?;
        Self::new(heap_file)
    }

    // 新しいページIDを採番する
    pub fn allocate_page(&mut Self) -> PageId {
    }

    // ページのデータを読み出す
    pub fn read_page_data(&mut Self, page_id:PageId, data: &mut [u8]) -> io::Result<()>{
    }

    // データをページに書き出す
    pub fn write_page_data(&mut Self, page_id: PageId, data: &[u8])->io::Result<()>{
    }
}

fn main() {
    println!("Hello, world!");
}
