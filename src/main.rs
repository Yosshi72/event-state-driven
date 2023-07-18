use rand::Rng;
use std::thread;
use std::time::Duration;
use EventStateDrivenMachine::event::Event;
use EventStateDrivenMachine::event_queue::EventQueue;
use EventStateDrivenMachine::state::{InvincibleState, State};

struct Player {
    current_state: State,
    current_invinsible_state: InvincibleState,
    event_queue: EventQueue,
    game_over: bool,
}

impl Player {
    pub fn new() -> Self {
        let current_state = State::Normal;
        let current_invinsible_state = InvincibleState::NonStarred;
        let event_queue = EventQueue::new();
        Self {
            current_state,
            current_invinsible_state,
            event_queue,
            game_over: false,
        }
    }

    pub fn attacked(&mut self) {
        self.event_queue.enqueue(Event::AttackedByEnemy);
    }

    pub fn get_mashed(&mut self) {
        self.event_queue.enqueue(Event::GetMash)
    }

    pub fn ger_flower(&mut self) {
        self.event_queue.enqueue(Event::GetFlower)
    }

    pub fn ger_star(&mut self) {
        self.event_queue.enqueue(Event::GetStar)
    }

    pub fn time_up(&mut self) {
        self.event_queue.enqueue(Event::TimePassed)
    }

    pub fn handle_event(&mut self, event: Event) {
        match &self.current_invinsible_state {
            &InvincibleState::Starred => match event {
                Event::TimePassed => {
                    self.current_invinsible_state = InvincibleState::NonStarred;
                }
                Event::GetFlower => {
                    self.current_state = State::Fire;
                }
                Event::GetMash => match &self.current_state {
                    &State::Normal => self.current_state = State::Super,
                    _ => (),
                },
                _ => (),
            },
            &InvincibleState::NonStarred => match event {
                Event::GetStar => self.current_invinsible_state = InvincibleState::Starred,
                Event::GetFlower => self.current_state = State::Fire,
                Event::GetMash => match &self.current_state {
                    &State::Normal => self.current_state = State::Super,
                    _ => (),
                },
                Event::AttackedByEnemy => match &self.current_state {
                    &State::Fire => self.current_state = State::Super,
                    &State::Super => self.current_state = State::Normal,
                    &State::Normal => self.game_over = true,
                },
                _ => (),
            },
        }
    }
}

fn occur_random_event(player: &mut Player) {
    let mut r = rand::thread_rng();
    match r.gen_range(0..6) {
        1 => player.attacked(),
        2 => player.get_mashed(),
        3 => player.ger_flower(),
        4 => player.ger_star(),
        5 => player.time_up(),
        _ => (),
    }
}

fn main() {
    let mut mario = Player::new();
    println!("Game Start");
    loop {
        occur_random_event(&mut mario);
        if let Some(event) = mario.event_queue.dequeue() {
            println!(
                "mario info: {{ now_state = {:?}, invinsible = {:?}, input_event : {:?} }}",
                mario.current_state, mario.current_invinsible_state, event
            );

            mario.handle_event(event);
        }
        if mario.game_over {
            println!("Game Over");
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }
}
