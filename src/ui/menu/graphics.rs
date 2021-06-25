use std::{sync::mpsc::Receiver, thread::sleep, time::{Duration, Instant}};

pub fn loading_screen(recv: Receiver<Option<String>>, amt: u64, min_millis: u64, bar_size: u64) {
    let mut time = Instant::now();
    let mut message = String::new();
    let mut counter = 0;
    while let Ok(val) = recv.recv() {
        if let Some(val) = val {
            message = val;
        }
        print!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
        print!("Loading...\n");
        print!("{}\n", message);
        for _ in 0..(counter * bar_size) / amt {
            print!("=");
        }
        for _ in (counter * bar_size) / amt .. bar_size - 1 {
            print!("_");
        }
        println!("");
        let random:f64 = rand::random::<f64>() + 0.5;
        let new_millis = ((min_millis as f64) * random) as u64;
        if time + Duration::from_millis(new_millis) > Instant::now() {
            sleep(time + Duration::from_millis(new_millis) - Instant::now());
        }
        time = Instant::now();
        counter += 1;
        if counter >= amt {
            break;
        }
    }
}