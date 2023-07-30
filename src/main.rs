use rand;
use std::thread;
use std::sync::{Arc, Mutex};

struct Loader {
    progress: Arc<Mutex<usize>>
}

impl Loader {
    fn new() -> Self {
        Loader {
            progress: Arc::new(Mutex::new(0))
        }
    }

    fn run(&self) -> thread::JoinHandle<()> {
        let progress = self.progress.clone();

        thread::spawn(move || {
            let mut is_running = true;
            while is_running {
                thread::sleep(std::time::Duration::from_millis(500));
                let mut progress = progress.lock().unwrap();
                if *progress >= 100 {
                    is_running = false;
                }
                else if rand::random() {
                    *progress += 1;
                }
            }
        })
    }
}

struct Printer<'a> {
    bars: &'a Vec<Loader>
}

impl<'a> Printer<'a> {
    fn new(loaders: &'a Vec<Loader>) -> Self {
        Printer { bars: loaders }
    }

    fn print(&self, is_first: bool) {
        let lines_num = self.bars.len() + 1;

        if is_first == false {
            println!("\x1b[{}F", lines_num);
        }

        for bar in self.bars.iter() {
            let progress = bar.progress.lock().unwrap();
            let done = "=".repeat(*progress / 5);
            let awaiting = " ".repeat(20 - (*progress / 5));

            match *progress {
                100 => println!("\x1b[32m[{}>{}] {}%", done, awaiting, *progress),
                _  => println!("\x1b[37m[{}>{}] {}%", done, awaiting, *progress)
            }
        }
    }
}

fn main() {
    let mut bars = vec![];
    for _ in 0..10 {
        bars.push(Loader::new());
    }
    let printer = Printer::new(&bars);

    printer.print(true);
    let mut threads: Vec<thread::JoinHandle<_>> = vec![];
    for bar in bars.iter() {
        threads.push(bar.run());
    }

    loop {
        let mut is_finished = true;
        for t in threads.iter() {
            if t.is_finished() != true {
                is_finished = false;
            }
        }
        thread::sleep(std::time::Duration::from_millis(500));
        printer.print(false);
        if is_finished {
            break;
        }
    }

}
