// Project: Sovereign SQRP Radar - Production Build
// Author: Programming & Tech Dept (Standard I)
// Resolution: Fixes E0255 and E0308 from local logs

use ring::digest::{self, Context}; // Fixed: Clean import for ring library
use chrono::Utc;

#[derive(Debug)]
struct RadarTarget {
    id: u32,
    distance_km: f64,
    velocity_ms: f64,
}

impl RadarTarget {
    fn threat_level(&self) -> &str {
        if self.velocity_ms > 1000.0 && self.distance_km < 100.0 {
            "CRITICAL: HYPERSONIC BREACH"
        } else if self.velocity_ms > 500.0 {
            "HIGH: FAST APPROACH"
        } else {
            "LOW: ROUTINE TRACKING"
        }
    }
}

fn main() {
    println!("--- SOVEREIGN RADAR: GLOBAL AUDIT ACTIVE ---");
    
    let targets = vec![
        RadarTarget { id: 2001, distance_km: 25.5, velocity_ms: 1550.0 }, // Test target
        RadarTarget { id: 2002, distance_km: 400.0, velocity_ms: 300.0 },
    ];

    // Using underscore to satisfy compiler unused-variable warning [Screenshot_63]
    let _audit_timestamp = Utc::now().timestamp();

    for target in &targets {
        let signature = generate_secure_signature(target, _audit_timestamp);
        let masked_coord = apply_obfuscation(target.distance_km);
        
        println!("[{}] ID: {} | Masked: {} | Seal: {}", 
                 target.threat_level(), 
                 target.id, 
                 masked_coord,
                 &signature[..14]); 
    }

    println!("--- CORE STATUS: SECURE & STABLE ---");
}

fn generate_secure_signature(target: &RadarTarget, ts: i64) -> String {
    // Fixed: Using ring's official SHA256 algorithm reference
    let mut context = Context::new(&digest::SHA256);
    context.update(&target.id.to_be_bytes());
    context.update(&target.distance_km.to_bits().to_be_bytes());
    context.update(&target.velocity_ms.to_bits().to_be_bytes());
    context.update(&ts.to_be_bytes());

    let digest = context.finish();
    hex_encode(digest.as_ref())
}

fn apply_obfuscation(coord: f64) -> String {
    // Security layer: Masking the distance data
    let bits = coord.to_bits();
    format!("0x{:x}", bits ^ 0x1234_5678_9ABC_DEF0)
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}