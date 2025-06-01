/// 5 万個 × 1 KiB のバッファを確保してダミー解析
pub fn build_large_response() -> String {
    let mut bufs: Vec<Vec<u8>> = Vec::with_capacity(50_000);
    for _ in 0..50_000 {
        bufs.push(vec![0u8; 1_024]); // 5 万回 malloc
    }

    // 解析フェーズ（ダミー）
    // parse(&bufs).await.expect("parse failed");
    "done".to_string()
}

// /// ダミーの非同期 parse
// async fn parse(_bufs: &[Vec<u8>]) -> Result<(), ()> {
//     // 何らかの I/O / CPU 処理を想定
//     Ok(())
// }
