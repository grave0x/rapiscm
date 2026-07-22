//! TCP port scanning, service version detection, and OS fingerprinting.
//!
//! Feature-gated behind `--features ip`. Lightweight connect-scan approach —
//! no raw sockets needed, works without root on all platforms.

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

use serde::Serialize;

/// Result of scanning a single port.
#[derive(Debug, Clone, Serialize)]
pub struct PortResult {
    pub port: u16,
    pub open: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<f64>,
}

/// Result of an OS fingerprint attempt.
#[derive(Debug, Clone, Serialize)]
pub struct OsFingerprint {
    pub ttl: u8,
    pub guessed_os: Option<String>,
    pub tcp_window_size: Option<u16>,
}

/// Full IP scan result.
#[derive(Debug, Clone, Serialize)]
pub struct IpScanResult {
    pub target: String,
    pub ports: Vec<PortResult>,
    pub os: Option<OsFingerprint>,
    pub duration_ms: f64,
    pub open_count: usize,
}

impl IpScanResult {
    pub fn summary(&self) -> String {
        let ports = self
            .ports
            .iter()
            .filter(|p| p.open)
            .map(|p| {
                let svc = p
                    .service
                    .as_deref()
                    .map(|s| format!(" ({s})"))
                    .unwrap_or_default();
                format!("{}{}", p.port, svc)
            })
            .collect::<Vec<_>>()
            .join(", ");

        let os = self
            .os
            .as_ref()
            .and_then(|o| o.guessed_os.as_deref())
            .map(|o| format!("  OS: {o}"))
            .unwrap_or_default();

        format!(
            "{} — {} open ports ({:.0}ms): {}{}",
            self.target,
            self.open_count,
            self.duration_ms,
            ports,
            os
        )
    }
}

/// Default ports to scan (API-relevant + common services).
pub const DEFAULT_PORTS: &[u16] = &[
    22, 80, 443, 8080, 8443, 3000, 5000, 8000, 8080, 8443, 9000, 9090,
];

/// Extended port set for thorough scans.
pub const EXTENDED_PORTS: &[u16] = &[
    21, 22, 23, 25, 53, 80, 110, 111, 135, 139, 143, 443, 445, 993, 995,
    1433, 1521, 2049, 2375, 2376, 3000, 3306, 3389, 4000, 5000, 5432,
    5601, 5900, 6379, 6443, 7000, 8000, 8080, 8081, 8443, 8888, 9000,
    9090, 9200, 9300, 10250, 27017,
];

/// Run a TCP connect scan against a target host on the given ports.
pub fn scan_ports(target: &str, ports: &[u16], timeout: Duration) -> Vec<PortResult> {
    let mut results = Vec::with_capacity(ports.len());

    for &port in ports {
        let addr_str = format!("{target}:{port}");
        let start = std::time::Instant::now();

        match TcpStream::connect_timeout(
            &addr_str.parse().unwrap_or_else(|_| SocketAddr::from(([127, 0, 0, 1], 0))),
            timeout,
        ) {
            Ok(mut stream) => {
                let latency = start.elapsed().as_secs_f64() * 1000.0;
                let banner = grab_banner(&mut stream, port, timeout);
                let service = detect_service(port, banner.as_deref());
                stream.shutdown(std::net::Shutdown::Both).ok();

                results.push(PortResult {
                    port,
                    open: true,
                    service,
                    banner,
                    latency_ms: Some(latency),
                });
            }
            Err(_) => {
                results.push(PortResult {
                    port,
                    open: false,
                    service: None,
                    banner: None,
                    latency_ms: None,
                });
            }
        }
    }

    results
}

/// Try to read a banner from the open port.
fn grab_banner(stream: &mut TcpStream, port: u16, timeout: Duration) -> Option<String> {
    stream
        .set_read_timeout(Some(timeout))
        .ok()?;

    // For HTTP ports, send a simple HTTP request
    let probe = if [80, 443, 8080, 8443, 3000, 5000, 8000, 9000, 9090].contains(&port) {
        b"GET / HTTP/1.0\r\nHost: localhost\r\n\r\n".to_vec()
    } else {
        // Just read what the server sends first
        vec![]
    };

    if !probe.is_empty() {
        stream.write_all(&probe).ok()?;
    }

    let mut buf = [0u8; 1024];
    match stream.read(&mut buf) {
        Ok(n) if n > 0 => {
            // Take first line for banner, strip non-printable
            let raw = String::from_utf8_lossy(&buf[..n]);
            let first_line = raw.lines().next().unwrap_or("").to_string();
            let clean: String = first_line
                .chars()
                .filter(|c| c.is_ascii_graphic() || c.is_ascii_whitespace())
                .take(200)
                .collect();
            if clean.is_empty() {
                None
            } else {
                Some(clean)
            }
        }
        _ => None,
    }
}

/// Detect service from port number and (optionally) banner.
fn detect_service(port: u16, banner: Option<&str>) -> Option<String> {
    let svc = match port {
        21 => "FTP",
        22 => "SSH",
        23 => "Telnet",
        25 => "SMTP",
        53 => "DNS",
        80 => {
            if let Some(b) = banner {
                if b.contains("HTTP") {
                    "HTTP"
                } else {
                    "HTTP?"
                }
            } else {
                "HTTP"
            }
        }
        110 => "POP3",
        111 => "RPC",
        135 => "MSRPC",
        139 => "NetBIOS",
        143 => "IMAP",
        443 => "HTTPS",
        445 => "SMB",
        993 => "IMAPS",
        995 => "POP3S",
        1433 => "MSSQL",
        1521 => "Oracle",
        2049 => "NFS",
        2375 => "Docker",
        2376 => "Docker-TLS",
        3000 => "HTTP-alt",
        3306 => "MySQL",
        3389 => "RDP",
        4000 => "HTTP-alt",
        5000 => "HTTP-alt",
        5432 => "PostgreSQL",
        5601 => "Kibana",
        5900 => "VNC",
        6379 => "Redis",
        6443 => "K8s-API",
        7000 => "Cassandra",
        8000 => "HTTP-alt",
        8080 => "HTTP-alt",
        8081 => "HTTP-alt",
        8443 => "HTTPS-alt",
        8888 => "HTTP-alt",
        9000 => "HTTP-alt",
        9090 => "HTTP-alt",
        9200 => "Elasticsearch",
        9300 => "Elasticsearch",
        10250 => "Kubelet",
        27017 => "MongoDB",
        _ => return None,
    };
    Some(svc.to_string())
}

/// Attempt OS fingerprint via TCP stack analysis.
///
/// Connects to the target and inspects the local TCP stack parameters
/// (TTL on received packets, window scaling, etc.) to guess the OS.
pub fn fingerprint_os(target: &str, port: u16, timeout: Duration) -> Option<OsFingerprint> {
    // Try to connect and grab the raw TCP info via TTL inference
    let addr = format!("{target}:{port}");
    let stream = TcpStream::connect_timeout(
        &addr.parse().ok()?,
        timeout,
    )
    .ok()?;

    // Read TTL from the socket (platform-specific, approximate)
    let ttl = stream.ttl().ok().unwrap_or(64);
    let window_size: u16 = 0; // Not easily accessible from std

    // Very rough OS guess from TTL
    let guessed_os = match ttl {
        64 => Some("Linux/Unix".to_string()),
        128 => Some("Windows".to_string()),
        255 => Some("Solaris/AIX".to_string()),
        _ => None,
    };

    stream.shutdown(std::net::Shutdown::Both).ok();

    Some(OsFingerprint {
        ttl: ttl as u8,
        guessed_os,
        tcp_window_size: if window_size > 0 {
            Some(window_size)
        } else {
            None
        },
    })
}

/// Run a full IP scan: ports + OS fingerprint.
pub fn run_ip_scan(
    target: &str,
    ports: &[u16],
    timeout: Duration,
) -> IpScanResult {
    let start = std::time::Instant::now();

    let port_results = scan_ports(target, ports, timeout);
    let open_count = port_results.iter().filter(|p| p.open).count();

    // Try OS fingerprint on the first open port
    let os = port_results
        .iter()
        .find(|p| p.open)
        .and_then(|p| fingerprint_os(target, p.port, timeout));

    let duration_ms = start.elapsed().as_secs_f64() * 1000.0;

    IpScanResult {
        target: target.to_string(),
        ports: port_results,
        os,
        duration_ms,
        open_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_ports_non_empty() {
        assert!(!DEFAULT_PORTS.is_empty());
        assert!(!EXTENDED_PORTS.is_empty());
    }

    #[test]
    fn test_detect_service_known_ports() {
        assert_eq!(detect_service(22, None), Some("SSH".to_string()));
        assert_eq!(detect_service(80, None), Some("HTTP".to_string()));
        assert_eq!(detect_service(443, None), Some("HTTPS".to_string()));
        assert_eq!(detect_service(3306, None), Some("MySQL".to_string()));
        assert_eq!(detect_service(5432, None), Some("PostgreSQL".to_string()));
        assert_eq!(detect_service(6379, None), Some("Redis".to_string()));
        assert_eq!(detect_service(27017, None), Some("MongoDB".to_string()));
    }

    #[test]
    fn test_detect_service_unknown_port() {
        assert_eq!(detect_service(12345, None), None);
    }

    #[test]
    fn test_scan_localhost_refused() {
        // Scan a port where nothing is listening — should report closed
        let results = scan_ports("127.0.0.1", &[19999], Duration::from_millis(200));
        assert_eq!(results.len(), 1);
        assert!(!results[0].open);
    }
}
