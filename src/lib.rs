use wasm_bindgen::prelude::*;
use md5::{Context, Digest};
use std::cell::RefCell;
use std::collections::HashMap;

// 存储多个哈希计算器的全局状态
thread_local! {
    static HASHERS: RefCell<HashMap<u32, Context>> = RefCell::new(HashMap::new());
}

#[wasm_bindgen]
pub fn calculate_md5(data: &[u8]) -> String {
    let digest = md5::compute(data);
    format!("{:x}", digest)
}

// 创建一个新的流式MD5计算器
#[wasm_bindgen]
pub fn create_md5_hasher() -> u32 {
    let id = (js_sys::Math::random() * 1000000.0) as u32;
    HASHERS.with(|hashers| {
        hashers.borrow_mut().insert(id, Context::new());
    });
    id
}

// 向指定的哈希计算器添加数据块
#[wasm_bindgen]
pub fn update_md5_hasher(id: u32, data: &[u8]) -> bool {
    HASHERS.with(|hashers| {
        if let Some(hasher) = hashers.borrow_mut().get_mut(&id) {
            hasher.consume(data);
            true
        } else {
            false
        }
    })
}

// 完成计算并获取最终的MD5值
#[wasm_bindgen]
pub fn finalize_md5_hasher(id: u32) -> Option<String> {
    HASHERS.with(|hashers| {
        if let Some(hasher) = hashers.borrow_mut().remove(&id) {
            let digest = hasher.compute();
            Some(format!("{:x}", digest))
        } else {
            None
        }
    })
}

// 清理指定的哈希计算器（可选，用于提前清理）
#[wasm_bindgen]
pub fn cleanup_md5_hasher(id: u32) -> bool {
    HASHERS.with(|hashers| {
        hashers.borrow_mut().remove(&id).is_some()
    })
}

