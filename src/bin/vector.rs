fn main() {
    // create method 1
    let mut v: Vec<i32> = Vec::new();
    v.push(1);

    // create method 2
    let v2 = vec![1, 2, 3];

    // create method 3
    let mut v3 = Vec::new();
    // add
    v3.push(1);
}

// remove(): 删除指定索引的元素并返回它
fn remove_vector() {
    let mut vec = vec![1, 2, 3, 4, 5];

    // 删除索引 2 的元素（值为3）
    let removed = vec.remove(2);
    println!("删除的元素: {}", removed);
    println!("现在的 vec: {:?}", vec); // [1, 2, 4, 5]

    // 注意：如果索引超出范围会 panic
    // vec.remove(10); // ❌ panic!
}

// swap_remove(): 用最后一个元素替换要删除的元素（不保持顺序但效率高）
fn swap_remove_vector() {
    let mut vec = vec![1, 2, 3, 4, 5];

    // 删除索引 1 的元素，用最后一个元素填充
    let removed = vec.swap_remove(1);
    println!("删除的元素: {}", removed); // 2
    println!("现在的 vec: {:?}", vec); // [1, 5, 3, 4]
}

// pop(): 删除并返回最后一个元素
fn pop_vector() {
    let mut vec = vec![1, 2, 3];

    // pop 返回 Option<T>
    while let Some(last) = vec.pop() {
        println!("弹出: {}", last);
    }
    println!("空向量: {:?}", vec); // []
}

// clear(): 删除所有元素
fn clear_vector() {
    let mut vec = vec![1, 2, 3, 4, 5];
    vec.clear();
    println!("清空后: {:?}", vec); // []
    println!("长度: {}", vec.len()); // 0
}

// retain(): 保留满足条件的元素
fn retain_vector() {
    let mut vec = vec![1, 2, 3, 4, 5, 6];

    // 只保留偶数
    vec.retain(|&x| x % 2 == 0);
    println!("保留偶数后: {:?}", vec); // [2, 4, 6]

    // 保留大于 3 的元素
    vec.retain(|&x| x > 3);
    println!("保留大于3的数: {:?}", vec); // [4, 6]
}

// drain(): 删除一个范围内的元素并返回迭代器
fn drain_vector() {
    let mut vec = vec![1, 2, 3, 4, 5];

    // 删除索引 1..3 范围的元素
    let drained: Vec<_> = vec.drain(1..3).collect();
    println!("抽取的元素: {:?}", drained); // [2, 3]
    println!("剩余的 vec: {:?}", vec); // [1, 4, 5]
}
