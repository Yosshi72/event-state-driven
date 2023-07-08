use rand::Rng;
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum State {
    On,
    Off,
}

#[derive(Debug)]
enum Event {
    PushedPowerButton,
    PushedVolumeUpButton,
    PushedVolumeDownButton,
}


struct TV {
    current_state: State,
    current_volume: u8,
    event_queue: EventQueue,
}

impl TV {
    pub fn new() -> Self {
        let current_state = State::Off;
        let event_queue = EventQueue::new();
        let current_volume = 10;
        Self {
            current_state,
            current_volume,
            event_queue,
        }
    }

    pub fn pushed_power_button (&mut self) {
        self.event_queue.enqueue(Event::PushedPowerButton);
    }

    pub fn pushed_volume_up_button (&mut self) {
        self.event_queue.enqueue(Event::PushedVolumeUpButton)
    }

    pub fn pushed_volume_down_button (&mut self) {
        self.event_queue.enqueue(Event::PushedVolumeDownButton)
    }

    pub fn handle_event(&mut self, event: Event) {
        match &self.current_state {
            &State::On => match event {
                Event::PushedPowerButton => {
                    self.current_state = State::Off;
                },
                Event::PushedVolumeUpButton => {
                    self.current_volume += 1;
                },
                Event::PushedVolumeDownButton => {
                    self.current_volume -= 1;
                }
            },
            &State::Off => match event {
                Event::PushedPowerButton => {
                    self.current_state = State::On;
                },
                _ => (),
            },
        }
    }
}


struct EventQueue(VecDeque<Event>);

impl EventQueue {
    pub fn new() -> Self {
        let d = VecDeque::new();
        EventQueue(d)
    }

    pub fn enqueue (&mut self, event: Event) {
        self.0.push_back(event);
    }

    pub fn dequeue (&mut self) -> Option<Event> { 
        self.0.pop_front()
    }
}

fn push_random_button (tv: &mut TV) {
    let mut r = rand::thread_rng();
    match r.gen_range(0..4) {
        1 => tv.pushed_power_button(),
        2 => tv.pushed_volume_up_button(),
        3 => tv.pushed_volume_down_button(),
        _ => (),
    }
}

fn main() {
    let mut tv = TV::new();
    tv.pushed_power_button();
    loop {
        push_random_button(&mut tv);
        if let Some(event) = tv.event_queue.dequeue() {
            println! (
                " tv info: {{ now_state = {:?}, volume = {}, input_event : {:?} }}",
                tv.current_state, tv.current_volume, event
            );

            tv.handle_event(event);
        }
        thread::sleep(Duration::from_secs(1));
    }
}