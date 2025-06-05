// src/commands/ping/check.rs
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence};
use std::net::IpAddr;
use std::time::Duration;

// ping_target ip可达检查
pub async fn ping_target(target: &str, timeout_ms: u64, count: u32) -> anyhow::Result<()> {
    // 解析目标 IP（Go的net.ParseIP类似）
    let addr: IpAddr = target.parse().or_else(|_| {
        // DNS解析（类似Go的net.LookupIP）
        let ips = dns_lookup::lookup_host(target)
            .map_err(|e| anyhow::anyhow!("DNS lookup failed: {}", e))?;
        ips.first()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No IP addresses found"))
    })?;

    // 创建客户端配置（类似Go的&net.IPConn{}）
    let config = Config::default();
    let client = Client::new(&config)?;

    // 生成随机标识符（类似Go的rand.Intn()）
    let rng = rand::random::<u16>();
    let identifier = PingIdentifier(rng);

    // 创建Pinger（类似Go的icmp.PacketConn）
    let mut pinger = client.pinger(addr, identifier).await;

    // 设置超时（类似Go的conn.SetDeadline()）
    pinger.timeout(Duration::from_millis(timeout_ms));

    // 发送请求（类似Go的for循环）
    for seq_num in 0..count {
        // 生成序列号（注意转换为u16）
        let seq = seq_num as u16; // 强制类型转换（类似Go的uint16(seqNum)）
        let payload = vec![0; 56]; // 固定负载（类似Go的[]byte{}）

        // 发送ping（类似Go的conn.WriteTo()）
        match pinger.ping(PingSequence(seq), &payload).await {
            Ok((packet, rtt)) => {
                // 获取响应信息（类似Go的net.IP.String()）
                let src_ip = match packet {
                    IcmpPacket::V4(p) => p.get_source().to_string(),
                    IcmpPacket::V6(p) => p.get_source().to_string(),
                };

                // 计算往返时间（类似Go的time.Since()）
                let elapsed_ms = rtt.as_secs_f64() * 1000.0;
                println!("Reply from {}: bytes={} time={:.2}ms",
                         src_ip,
                         payload.len(), // 使用固定负载长度
                         elapsed_ms
                );
            }
            Err(e) => {
                // 错误处理（类似Go的err != nil）
                println!("Request timeout: {}", e);
            }
        }
    }

    Ok(())
}