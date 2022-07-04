rust 创建稀疏文件

use async_std::prelude::*;
use async_std::fs::File;
use async_std::fs::OpenOptions;

let file = OpenOptions::new()
    .write(true)
    .open("a.txt")
    .await?;

file.set_len(10).await?;
file.seek(SeekFrom::Start(3)).await?;
file.write_all(b"Hello, world!").await?;

请求文件

[译] [论文] BBR：基于拥塞（而非丢包）的拥塞控制（ACM, 2017）
https://arthurchiao.art/blog/bbr-paper-zh/

从流量控制算法谈网络优化 – 从 CUBIC 到 BBRv2 算法
https://aws.amazon.com/cn/blogs/china/talking-about-network-optimization-from-the-flow-control-algorithm/

接收文件大小 分片哈希 offset 分片哈希

发包间隔 10 毫秒
发包速度 10
发包速度增速 10
收包速度 0

收包速度 = (收包速度 * 63 + 收包速度) / 64

每 64 个周期，记录一次收包速度
如果收包 >= 发包，发包速度倍增
否则发包 = 收包速度+1
