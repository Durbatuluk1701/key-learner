use crossterm::
    event::{self, KeyCode, KeyEvent, KeyModifiers};
use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use rand::prelude::*;

fn select_random_element<T>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        None
    } else {
        let random_index = thread_rng().gen_range(0..list.len());
        Some(&list[random_index])
    }
}

fn print_list<T : std::fmt::Display>(list: &VecDeque<T>) {
  for item in list {
    print!("{}", item)
  }
  println!("")
}

fn main() {
    println!("Welcome to the Rust Typing Game!");
    println!("Type characters and press 'q' to quit.");

    // let char_list: [char; 8] = ['n', 'r','t', 'a', 'i','e','s','o'];
    let char_list: [char; 4] = ['t', 'a', 'i','e',];

    let (tx, rx) = mpsc::channel();

    // Spawn a thread to listen for keyboard events
    thread::spawn(move || {
        loop {
            if event::poll(Duration::from_millis(100)).unwrap() {
                if let event::Event::Key(
                  KeyEvent { code, modifiers, kind, .. }
                ) = event::read().unwrap()
                {
                    if modifiers == KeyModifiers::NONE && kind == event::KeyEventKind::Press {
                        tx.send(code).unwrap();
                    }
                }
            }
        }
    });

    let mut chars: VecDeque<char> = VecDeque::new();
    for _ in 0..=5 {
      if let Some(random_element) = select_random_element(&char_list) {
        chars.push_back(*random_element)
      }
    }
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    print_list(&chars);

    loop {
        if let Ok(key_event) = rx.recv() {
            match key_event {
                KeyCode::Char(c) => {
                    if c == chars[0] {
                      chars.pop_front();
                      if let Some(random_element) = select_random_element(&char_list) {
                          chars.push_back(*random_element)
                      }
                      print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    }
                    else if c == 'q' {
                        println!("Quitting the game.");
                        break;
                    } else {
                      println!("WRONG")
                    }
                    print_list(&chars)
                }
                KeyCode::Esc => {
                    println!("Quitting the game.");
                    break;
                }
                _ => (),
            }
        }
    }

    // Clear the terminal before exiting
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    return;
}
