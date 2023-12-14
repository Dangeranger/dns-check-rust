// Import necessary crates
use std::env;
use std::sync::mpsc;
use std::thread;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;
use trust_dns_proto::rr::RecordType;

// Define the Server struct
#[derive(Clone)]
struct Server {
    ip: String,
    location: String,
    provider: String,
}

// Main function to parse arguments and start the DNS check
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: dnscheck <domain> <record_type>");
        return;
    }

    let domain = &args[1];
    let record_type = &args[2];

    let servers = vec![
        Server {
            ip: "8.8.8.8".to_string(),
            location: "Mountain View, California".to_string(),
            provider: "Google LLC".to_string(),
        },
        Server {
            ip: "1.1.1.1".to_string(),
            location: "San Francisco, California".to_string(),
            provider: "Cloudflare, Inc".to_string(),
        },

        // ... Add other servers as needed
    ];

    check_dns_propagation(domain, record_type, &servers);
}

// Function to check DNS propagation
fn check_dns_propagation(domain: &str, record_type: &str, servers: &[Server]) {
    let (tx, rx) = mpsc::channel();

    let servers = servers.to_vec();
    let handles: Vec<_> = servers.clone().into_iter().map(|server| {
        let tx = tx.clone();
        let domain = domain.to_string();
        let record_type = record_type.to_string();

        // Spawn the thread and move the cloned server into the closure
        thread::spawn(move || {
            let resolver_config = ResolverConfig::from_parts(
                None,
                vec![],
                NameServerConfigGroup::from_ips_clear(&[server.ip.parse().unwrap()], 53, true),
            );
            let resolver_opts = ResolverOpts::default();
            let resolver = Resolver::new(resolver_config, resolver_opts).unwrap();

            let response = match record_type.as_ref() {
                "A" => resolver.lookup(&domain, RecordType::A),
                "NS" => resolver.lookup(&domain, RecordType::NS),
                "CNAME" => resolver.lookup(&domain, RecordType::CNAME),
                "MX" => resolver.lookup(&domain, RecordType::MX),
                "TXT" => resolver.lookup(&domain, RecordType::TXT),
                "AAAA" => resolver.lookup(&domain, RecordType::AAAA),
                "SRV" => resolver.lookup(&domain, RecordType::SRV),
                "SOA" => resolver.lookup(&domain, RecordType::SOA),
                "PTR" => resolver.lookup(&domain, RecordType::PTR),
                "CAA" => resolver.lookup(&domain, RecordType::CAA),
                "ANY" => resolver.lookup(&domain, RecordType::ANY),
                // Add other record types as needed
                _ => panic!("Unsupported record type"),
            };

            match response {
                Ok(response) => {
                    let answer_str = response
                        .iter()
                        .map(|ip| ip.to_string())
                        .collect::<Vec<_>>()
                        .join("\n - ");
                    tx.send(format!(
                        "Server {} ({}, {}) reports:\n - {}",
                        server.ip, server.provider, server.location, answer_str
                    ))
                    .unwrap();
                }
                Err(err) => tx
                    .send(format!("Error querying {}: {:?}", server.ip, err))
                    .unwrap(),
            }
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    for _ in 0..servers.len() {
        println!("{}", rx.recv().unwrap());
    }
}
