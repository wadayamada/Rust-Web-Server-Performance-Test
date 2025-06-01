mod person_handler;
mod not_escape;
mod heap_alloc;

use std::env;
use crate::not_escape::build_large_response_item;
use crate::heap_alloc::build_large_response;
use sysinfo::{System, SystemExt, PidExt, ProcessExt};

use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const RUNS: usize = 1000;      // 繰り返し回数

    let mut total = 0.0;

    for i in 1..=RUNS {
        let start = Instant::now();

        let mode = env::args().nth(1).unwrap_or_else(|| "slice".to_string());

        let sum = match mode.as_str() {
            "item"  => {
                println!("Running Item (stack) version");
                build_large_response_item()
            }
            "slice" | "vec" | "default" => {
                println!("Running Vec (heap) version");
                build_large_response();
                1
            }
            other => {
                eprintln!("Unknown mode: {other}. Use 'slice' or 'item'");
                std::process::exit(1);
            }
        };
        
        let elapsed = start.elapsed().as_secs_f64();
        total += elapsed;

        let mut sys = System::new();
        sys.refresh_memory();     // システムメモリ
        sys.refresh_processes();
        print_process_mem_mb(&sys);
        println!("Run {:3}: {:.6} seconds", i, elapsed);
    }

    println!("Average over {} runs: {:.6} seconds", RUNS, total / RUNS as f64);

    Ok(())
}

fn print_mem_usage_mb() {
    // プロセス情報を取得
    let mut sys = System::new();
    sys.refresh_processes();

    let pid = sysinfo::get_current_pid().unwrap();
    if let Some(proc) = sys.process(pid) {
        // kB → MB（1024 で割る）
        let rss_mb  = proc.memory()          as f64 / 1024.0;
        let virt_mb = proc.virtual_memory()  as f64 / 1024.0;

        println!("Resident  : {:.2} MB", rss_mb);
        println!("Virtual   : {:.2} MB", virt_mb);
    } else {
        eprintln!("process info not found");
    }
}

fn print_system_mem_mb(sys: &System) {
    // システム全体の使用量
    let total_mb = sys.total_memory() as f64 / 1024.0;
    let used_mb  = sys.used_memory()  as f64 / 1024.0;
    let free_mb  = sys.free_memory()  as f64 / 1024.0;

    println!("System Memory  Total: {:.2} MB  Used: {:.2} MB  Free: {:.2} MB",
             total_mb, used_mb, free_mb);
}

fn print_process_mem_mb(sys: &System) {
    // 自プロセスの RSS
    let pid = sysinfo::get_current_pid().unwrap();
    if let Some(proc) = sys.process(pid) {
        let rss_mb = proc.memory() as f64 / 1024.0; // kB → MB
        println!("Process RSS : {:.2} MB", rss_mb);
    }
}
