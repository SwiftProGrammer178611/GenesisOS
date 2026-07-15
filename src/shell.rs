use alloc::string::String;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;
use pc_keyboard::KeyCode;
use crate::{print, println};

lazy_static!{
    pub static ref SHELL: Mutex<Shell> = Mutex::new(Shell::new());
}

pub struct Shell {
    buffer: String,
    history: Vec<String>,
    history_index:usize,
}

impl Shell {
    fn new() -> Self{
        Shell {
            buffer:String::new(),
            history: Vec::new(),
            history_index:0,
        }
    }
    pub fn handle_key(&mut self, c:char){
        match c{
            '\n' => {
                println!();
                let line = self.buffer.clone();
                if !line.trim().is_empty() {
                    self.history.push(line.clone());
                }
                self.history_index = self.history.len();
                self.buffer.clear();
                execute(&line);
                print!("> ");
            }
            // This was the worst. Originally had the "\u{8} \u{8} trick to backspace, but vga has no cocnept of the cursoor moving backwards. Rather, it treated it liek a kidna glyph so I had to write my own real backspace method on WRiter to make this wokr"
            '\u{8}' => {
                if self.buffer.pop().is_some(){
                    crate::vga_buffer::WRITER.lock().backspace();
                }
            }
            c => {
                self.buffer.push(c);
                print!("{}",c);
            }
        }
    }

    pub fn handle_raw_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::ArrowUp => self.history_prev(),
            KeyCode::ArrowDown => self.history_next(),
            _=>{}
        }
    }

    fn clear_input_line(&mut self){
        for _ in 0..self.buffer.len(){
            crate::vga_buffer::WRITER.lock().backspace();
        }
    }
    //here, the order matters bc the clearinput line rads the self.buffer.len to know hwo much to erase. so it has to run bef. self.buffer gets overwritten
    fn set_buffer(&mut self, new_line: String){
        self.clear_input_line();
        self.buffer = new_line;
        print!("{}", self.buffer);
    }

    fn history_prev(&mut self){
        if self.history.is_empty(){
            return;
        }
        if self.history_index>0 {
            self.history_index -= 1;
        }
        let line = self.history[self.history_index].clone();
        self.set_buffer(line);
    }

    fn history_next(&mut self){
        if self.history.is_empty() {
            return;
        }
        if self.history_index < self.history.len() {
            self.history_index += 1;
        }
        if self.history_index == self.history.len(){
            self.set_buffer(String::new());
        }else{
            let line = self.history[self.history_index].clone();
            self.set_buffer(line);
        }
    }
}



pub fn init() {
    print!("> ");
    lazy_static::initialize(&SHELL);
}

fn execute(line: &str){
    let mut parts = line.trim().split_whitespace();
    let cmd = match parts.next() {
        Some(c) => c,
        None => return,
    };
    let args : Vec<&str> = parts.collect();

    match cmd {
        "help" => println!("commands: help, clear, echo, meminfo, uptime, color, reboot panic"),
        "clear" => crate::vga_buffer::WRITER.lock().clear_all(),
        "echo" => println!("{}", args.join(" ")),
        //pulls real info from tracking allocator
        "meminfo" => crate::allocator::print_heap_stats(),
        "uptime" => println!(
            "ticks: {}",
            crate::interrupts::TICKS.load(core::sync::atomic::Ordering::Relaxed)
        ),
        "color" => set_color(args.get(0).copied()),
        "reboot" => reboot(),
        "panic" => panic!("user requested panic"),
        other => println!("unknown command: {}", other),
    }
}
fn set_color(name: Option<&str>){
    use crate::vga_buffer::Color;
    let color = match name {
        Some("black") => Color::Black,
        Some("blue") => Color::Blue,
        Some("green") => Color::Green,
        Some("cyan") => Color::Cyan,
        Some("red") => Color::Red,
        Some("magenta") => Color::Magenta,
        Some("brown") => Color::Brown,
        Some("lightgray") => Color::LightGray,
        Some("darkgray") => Color::DarkGray,
        Some("lightblue") => Color::LightBlue,
        Some("lightgreen") => Color::LightGreen,
        Some("lightcyan") => Color::LightCyan,
        Some("lightred") => Color::LightRed,
        Some("pink") => Color::Pink,
        Some("yellow") => Color::Yellow,
        Some("white") => Color::White,
        _=> {
            println!("usage: color <name> (black, blue, green, cyan, red, magenta, brown, lightgray, darkgray, lightblue, lightgreen, lightcyan ,lightred, pink, yellow, white)");
            return;
        }
    };
    crate::vga_buffer::WRITER.lock().set_color(color);
    println!("color set to{:?}", color);
}

// This is a REAL hardware reset. writing to 0xfe to the 8042 Keeb's cmd port 0x64 pulses the cpu reset to low whcih is the classic way the BIOSes used to reboot machines.
// This works bc QEMU emulates the controller. 

fn reboot() -> ! {
    use x86_64::instructions::port::Port;
    unsafe{
        let mut port: Port<u8> =Port::new(0x64);
        port.write(0xFEu8);
    }
    loop {
        //just in case reset doesn't work
        x86_64::instructions::hlt();
    }
}