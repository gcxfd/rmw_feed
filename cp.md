文件传输协议

客户端
  服务器地址
    每秒收到的有效消息数
    任务编号
      远程文件路径
      本地文件路径
      offset 内容缓存
      1MB 为单位，尚未请求的 bitmap

服务器端
  客户端地址
    mtu
    每秒发送的有效消息数

    基准延时 (xxhash64 + 时间 + 秘钥) // http://www.calvinneo.com/2017/12/05/libutp%E6%BA%90%E7%A0%81%E7%AE%80%E6%9E%90/
      两分钟内的最小延迟值 delay_base
      超过最小时延 100ms 以上，就会减少发送

    任务编号
      本地文件路径
      偏移
      缓存

https://docs.rs/rocket-file-cache/latest/rocket_file_cache/struct.Cache.html

# Mac 系统 ping 不用提权的原因
https://blog.csdn.net/aa642531/article/details/85461294
发现它可以使用 socket(AF_INET,SOCK_DGRAM, IPPROTP_ICMP) 这种套接字，这种套接字怎么来理解呢？
我们平常使用最多的是 TCP 和 UDP 的协议，即 socket(AF_INET,SOCK_STREAM, 0) 和 socket(AF_INET,SOCK_DGRAM, 0) 这两种套接字。

# support set IP_DONTFRAG
https://github.com/rust-lang/socket2/issues/318
