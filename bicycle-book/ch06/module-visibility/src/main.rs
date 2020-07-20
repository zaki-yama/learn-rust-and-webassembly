mod server {
    // server::echo()はserverモジュールの含まれるクレートに対してはパブリック
    pub(crate) fn echo() {
        println!("Server");
    }
}

mod client {
    pub fn echo() {
        println!("Client");
    }
}

mod network;

fn main() {
    server::echo();
    client::echo();

    // ルートモジュールからの相対指定で、networkモジュールの中のping()関数の呼び出し
    network::ping();

    // crateはクレートのルートモジュールを指す特別な名前
    // 次のように書くこともできる
    crate::network::ping();

    // selfは現在地のモジュールを指す特別な名前
    // ここでのselfはルートモジュール
    self::network::ping();
}
