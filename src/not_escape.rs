const CHUNK_CNT: usize = 50_000;
const ARR_LEN: usize   = 128;       // 128 * 8 byte = 1 KiB

/// 1 KiB の固定長データを持つ Item
struct Item {
    data: [u64; ARR_LEN],
}

/// Go の `buildLargeResponseItem` と同じ処理を Rust で実装
pub fn build_large_response_item() -> u64 {
    let mut total: u64 = 0;

    for _ in 0..CHUNK_CNT {
        // 値として生成するので（ポインタを外に出さなければ）スタック配置
        let mut it = Item { data: [0; ARR_LEN] };

        for j in 0..ARR_LEN {
            it.data[j] = j as u64;
        }

        total += it.data[0];
    }

    total
}