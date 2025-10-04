pub fn parse_scheme_host_port(url: impl AsRef<str>) -> Result<(String, String, u16), &'static str> {
    let (scheme, rest) = url.as_ref().split_once("://").ok_or("missing scheme")?;
    let scheme = scheme.to_ascii_lowercase();

    let authority = rest.split_once('/').map(|(a, _)| a).unwrap_or(rest);
    if authority.is_empty() {
        return Err("empty authority");
    }

    let hostport = authority.rsplit_once('@').map(|(_, h)| h).unwrap_or(authority);

    let (host, port_opt) = if let Some(h) = hostport.strip_prefix('[') {
        let (host6, rest) = h.split_once(']').ok_or("invalid IPv6 bracket")?;
        if let Some(rest) = rest.strip_prefix(':') {
            (host6.to_string(), Some(rest))
        } else if rest.is_empty() {
            (host6.to_string(), None)
        } else {
            return Err("invalid IPv6 authority");
        }
    } else {
        match hostport.split_once(':') {
            Some((h, p)) if !p.is_empty() => (h.to_string(), Some(p)),
            _ => (hostport.to_string(), None),
        }
    };

    if host.is_empty() {
        return Err("empty host");
    }

    let port: u16 = if let Some(p) = port_opt {
        p.parse().map_err(|_| "invalid port number")?
    } else {
        match scheme.as_str() {
            "http" | "ws" => 80,
            "https" | "wss" => 443,
            _ => return Err("port not specified and no default for scheme"),
        }
    };

    Ok((scheme, host, port))
}