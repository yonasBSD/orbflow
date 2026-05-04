// Copyright 2026 The Orbflow Authors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Shared SSRF (Server-Side Request Forgery) primitives.
//!
//! Provides common functions for checking whether an IP address or URL hostname
//! points to a private, internal, or cloud-metadata address. Used by adapter
//! crates (`orbflow-builtins`, `orbflow-mcp`) for defense-in-depth URL validation.

use std::net::IpAddr;

/// Known cloud metadata hostnames that must always be blocked.
pub const BLOCKED_HOSTNAMES: &[&str] = &[
    "metadata.google.internal",
    "metadata.internal",
    "169.254.169.254",
];

/// Allowed URL schemes for SSRF validation.
pub const ALLOWED_SCHEMES: &[&str] = &["http", "https"];

/// Checks whether an IP address is private, loopback, link-local, or
/// otherwise internal (cloud metadata, etc.).
///
/// Returns `Some(reason)` if the IP is considered private/internal,
/// `None` if it is a public address. When `allow_localhost` is `true`,
/// loopback addresses are permitted.
pub fn is_private_ip(ip: &IpAddr, allow_localhost: bool) -> Option<&'static str> {
    let ip = match ip {
        IpAddr::V6(v6) => {
            if let Some(v4) = v6.to_ipv4_mapped() {
                IpAddr::V4(v4)
            } else if let Some(v4) = v6.to_ipv4() {
                // to_ipv4() catches IPv4-compatible IPv6 addresses like `::127.0.0.1`
                // But wait, `::1` gives `Some(0.0.0.1)`. We must only map it if it's
                // a true IPv4-compatible IPv6 address. IPv4-compatible addresses
                // have the first 96 bits as zero. So `v6.segments()[0..5] == [0,0,0,0,0]`.
                // Note that `::1` has segments `[0,0,0,0,0,0,0,1]`. `to_ipv4()` returns `0.0.0.1`.
                // `0.0.0.1` is not considered private by `v4.is_private()` or loopback by `is_loopback()`,
                // but wait, `v4.octets()[0] == 0` is caught by our check! So `::1` would be blocked as non-routable!
                // But `::1` is loopback. If `allow_localhost` is true, we should allow it.
                // If it becomes `0.0.0.1`, `is_loopback` is false, and it gets blocked by `v4.octets()[0] == 0`.
                // So we must be careful not to blindly convert `::1`.
                let segs = v6.segments();
                if segs[0] == 0
                    && segs[1] == 0
                    && segs[2] == 0
                    && segs[3] == 0
                    && segs[4] == 0
                    && segs[5] == 0
                {
                    if segs[6] == 0 && segs[7] == 1 {
                        // ::1 is loopback, don't convert it to 0.0.0.1
                        *ip
                    } else if segs[6] == 0 && segs[7] == 0 {
                        // :: is unspecified
                        *ip
                    } else {
                        IpAddr::V4(v4)
                    }
                } else {
                    *ip
                }
            } else {
                *ip
            }
        }
        _ => *ip,
    };

    if ip.is_loopback() && !allow_localhost {
        return Some("loopback address");
    }

    match ip {
        IpAddr::V4(v4) => {
            if v4.is_private() {
                return Some("private IPv4 address");
            }
            if v4.is_link_local() || (v4.octets()[0] == 169 && v4.octets()[1] == 254) {
                return Some("link-local address");
            }
            // CGNAT / Shared Address Space (RFC 6598): 100.64.0.0/10
            let octets = v4.octets();
            if octets[0] == 100 && (octets[1] & 0xC0) == 0x40 {
                return Some("shared address space (CGNAT)");
            }
            if v4.is_broadcast() || v4.octets()[0] == 0 {
                return Some("non-routable address");
            }
            None
        }
        IpAddr::V6(v6) => {
            if v6.is_loopback() && !allow_localhost {
                return Some("loopback address");
            }
            let segments = v6.segments();
            // ULA (fc00::/7)
            if (segments[0] & 0xfe00) == 0xfc00 {
                return Some("private IPv6 (ULA) address");
            }
            // Link-local (fe80::/10)
            if (segments[0] & 0xffc0) == 0xfe80 {
                return Some("link-local IPv6 address");
            }
            // AWS IMDSv2 IPv6 (fd00:ec2::254)
            if segments[0] == 0xfd00
                && segments[1] == 0x0ec2
                && segments.iter().skip(2).take(5).all(|&s| s == 0)
                && segments[7] == 0x0254
            {
                return Some("cloud metadata IPv6 address");
            }
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_private_ip() {
        assert_eq!(
            is_private_ip(&"127.0.0.1".parse().unwrap(), false),
            Some("loopback address")
        );
        assert_eq!(
            is_private_ip(&"::1".parse().unwrap(), false),
            Some("loopback address")
        );
        assert_eq!(
            is_private_ip(&"::ffff:127.0.0.1".parse().unwrap(), false),
            Some("loopback address")
        );
        assert_eq!(
            is_private_ip(&"::ffff:169.254.169.254".parse().unwrap(), false),
            Some("link-local address")
        );
        assert_eq!(is_private_ip(&"8.8.8.8".parse().unwrap(), false), None);
        assert_eq!(
            is_private_ip(&"::ffff:8.8.8.8".parse().unwrap(), false),
            None
        );
    }
}
