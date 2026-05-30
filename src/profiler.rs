/// Lightweight built-in profiler for Nimble programs
use std::collections::HashMap;
use std::time::Instant;

pub struct Profiler {
    enabled: bool,
    timers: HashMap<String, (Instant, u64, f64)>, // name -> (start, count, total_time)
    output_file: Option<String>,
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Profiler {
    pub fn new() -> Self {
        Profiler {
            enabled: std::env::var("NIMBLE_PROFILE").is_ok(),
            timers: HashMap::new(),
            output_file: std::env::var("NIMBLE_PROFILE_OUT").ok(),
        }
    }

    pub fn start(&mut self, name: &str) {
        if self.enabled {
            self.timers
                .insert(name.to_string(), (Instant::now(), 0, 0.0));
        }
    }

    pub fn end(&mut self, name: &str) {
        if self.enabled
            && let Some((start, count, total)) = self.timers.get_mut(name)
        {
            let elapsed = start.elapsed().as_secs_f64();
            *count += 1;
            *total += elapsed;
        }
    }

    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Nimble Profiler Report ===\n");
        report.push_str(&format!(
            "{:<30} {:>8} {:>12} {:>12}\n",
            "Timer", "Count", "Total (s)", "Avg (s)"
        ));
        report.push_str(&"-".repeat(64));
        report.push('\n');

        let mut sorted: Vec<_> = self.timers.iter().collect();
        sorted.sort_by(|a, b| {
            b.1.2
                .partial_cmp(&a.1.2)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for (name, (_, count, total)) in &sorted {
            let avg = if *count > 0 {
                total / *count as f64
            } else {
                0.0
            };
            report.push_str(&format!(
                "{:<30} {:>8} {:>12.6} {:>12.9}\n",
                name, count, total, avg
            ));
        }

        report
    }

    pub fn write_report(&self) {
        if let Some(ref path) = self.output_file {
            if let Ok(mut f) = std::fs::File::create(path) {
                use std::io::Write;
                let _ = f.write_all(self.report().as_bytes());
            }
        } else {
            println!("{}", self.report());
        }
    }
}
