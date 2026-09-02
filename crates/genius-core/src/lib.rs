#![no_std]
const SEQ_MAX_LENGTH: usize = 10;

/**
 * Vamos pensar em como atender a spec gerada pela IA. Ela propôs essa crate separada.
 * o game loop basico é o seguinte:
 * - jogo iniciado
 * - mostra primeira sequencia (b,g,r)
 * - espera length input do usuário
 * - usuário acertou
 *    - feedback de acertou
 *    - acrescenta 1 na sequencia (b,g,r,b)
 *    - recomeça loop
 * - usuário errou
 *    - feedback de errou
 *    - estado de erro até y
 *
 * {
 *  sequence: []Color
 * }
 */
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Status {
    Startup,         // aceita apenas colorgiven e gamestarted
    PlayingSequence, // aceita apenas sequenceplaybackcompleted e reset
    AwaitingInput,   // aceita apenas colorPressed e reset
    RoundSuccess,    // aceita apenas colorgiven, game started e reset
    GameOver,        // aceita apenas reset
    GameCompleted,   // aceita apenas reset
}
#[derive(PartialEq)]
pub enum Event {
    GameStarted,               //startup, roundsuccess
    ColorGiven(Color),         //startup roundsucess
    ColorPressed(Color),       // awaitinginput
    ResetRequested,            // todos
    SequencePlaybackCompleted, // playingsequence
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone)]
pub struct State {
    status: Status,
    sequence: [Option<Color>; 10],
    current_sequence_size: usize,
    next_player_try: usize,
}

impl State {
    pub fn new() -> Self {
        Self {
            status: Status::Startup,
            sequence: [None; SEQ_MAX_LENGTH],
            current_sequence_size: 0,
            next_player_try: 0,
        }
    }
    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn sequence(&self) -> [Option<Color>; 10] {
        self.sequence
    }

    pub fn current_sequence_size(&self) -> usize {
        self.current_sequence_size
    }

    pub fn next_player_try(&self) -> usize {
        self.next_player_try
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

pub fn state_machine(state: State, event: Event) -> State {
    match event {
        Event::ColorGiven(color) => {
            if state.status != Status::Startup && state.status != Status::RoundSuccess {
                return State { ..state };
            }
            let i = state.current_sequence_size;

            if i >= SEQ_MAX_LENGTH {
                return State {
                    current_sequence_size: SEQ_MAX_LENGTH,
                    ..state
                };
            }
            let mut sequence = state.sequence;
            sequence[i] = core::prelude::v1::Some(color);

            State {
                sequence,
                current_sequence_size: i + 1,
                ..state
            }
        }

        Event::GameStarted => {
            //startup, roundsuccess
            if state.status != Status::Startup && state.status != Status::RoundSuccess {
                return State { ..state };
            }
            State {
                status: Status::PlayingSequence,
                ..state
            }
        }

        Event::SequencePlaybackCompleted => {
            // playingsequence
            if state.status != Status::PlayingSequence {
                return State { ..state };
            }
            State {
                status: Status::AwaitingInput,
                ..state
            }
        }

        Event::ResetRequested => State::new(),

        Event::ColorPressed(color) => {
            // awaitinginput
            if state.status != Status::AwaitingInput {
                return State { ..state };
            }

            if state.sequence[state.next_player_try] != core::prelude::v1::Some(color) {
                return State {
                    status: Status::GameOver,
                    ..state
                };
            }

            if state.next_player_try == SEQ_MAX_LENGTH - 1
                && state.current_sequence_size == SEQ_MAX_LENGTH
            {
                return State {
                    status: Status::GameCompleted,
                    ..state
                };
            }

            if !state.sequence.is_empty()
                && state.next_player_try == state.current_sequence_size - 1
            {
                return State {
                    status: Status::RoundSuccess,
                    next_player_try: 0,
                    ..state
                };
            }
            State {
                status: Status::AwaitingInput,
                next_player_try: state.next_player_try + 1,
                ..state
            }
        }
    }
}

//Test cases
// acerto parcial mantém AwaitingInput e avança a posição [ok]
// ColorPressed não altera estados que não aceitam cores (PlayingSequence, GameOver, GameCompleted, Startup e RoundSuccess) [ok]
// adicionar cor com a sequência cheia não causa panic [ok]

//Outros ajustes
//Trocar os bytes por um tipo Color [ok]

#[cfg(test)]
mod tests {
    use crate::Event::ColorPressed;

    use super::*;

    #[test]
    fn test_state_getters() {
        let initial_state = State {
            status: Status::Startup,
            sequence: [None; 10],
            current_sequence_size: 1,
            next_player_try: 2,
        };

        let desired_state_status = Status::Startup;
        let desired_state_sequence = [None; 10];
        let desired_state_current_sequence_size = 1;
        let desired_state_next_player_try = 2;

        let resulting_state_status = initial_state.status();
        let resulting_state_sequence = initial_state.sequence();
        let resulting_state_current_sequence_size = initial_state.current_sequence_size();
        let resulting_state_next_player_try = initial_state.next_player_try();

        assert_eq!(
            *resulting_state_status, desired_state_status,
            "testing state.status resulting: {:#?} desired: {:#?}",
            resulting_state_status, desired_state_status
        );
        assert_eq!(
            resulting_state_sequence, desired_state_sequence,
            "testing state.sequence resulting: {:?} desired: {:?}",
            resulting_state_sequence, desired_state_sequence
        );
        assert_eq!(
            resulting_state_current_sequence_size, desired_state_current_sequence_size,
            "testing state.current_sequence_size resulting: {} desired: {}",
            resulting_state_current_sequence_size, desired_state_current_sequence_size
        );
        assert_eq!(
            resulting_state_next_player_try, desired_state_next_player_try,
            "testing state.next_player_try resulting: {} desired: {}",
            resulting_state_next_player_try, desired_state_next_player_try
        );
    }

    #[test]
    fn test_color_given_event() {
        let initial_state = State {
            status: Status::Startup,
            sequence: [None; 10],
            current_sequence_size: 0,
            next_player_try: 0,
        };

        let mut seq = [None; 10];
        seq[0] = core::prelude::v1::Some(Color::Green);

        let desired_state = State {
            status: Status::Startup,
            sequence: seq,
            current_sequence_size: 1,
            next_player_try: 0,
        };

        let resulting_state = state_machine(initial_state, Event::ColorGiven(Color::Green));
        assert_eq!(
            resulting_state.status, desired_state.status,
            "testing state.status resulting: {:#?} desired: {:#?}",
            resulting_state.status, desired_state.status
        );
        assert_eq!(
            resulting_state.sequence, desired_state.sequence,
            "testing state.sequence resulting: {:?} desired: {:?}",
            resulting_state.sequence, desired_state.sequence
        );
        assert_eq!(
            resulting_state.current_sequence_size, desired_state.current_sequence_size,
            "testing state.current_sequence_size resulting: {} desired: {}",
            resulting_state.current_sequence_size, desired_state.current_sequence_size
        );
        assert_eq!(
            resulting_state.next_player_try, desired_state.next_player_try,
            "testing state.next_player_try resulting: {} desired: {}",
            resulting_state.next_player_try, desired_state.next_player_try
        );
    }

    #[test]
    fn test_color_given_event_with_full_sequence() {
        let mut seq = [None; 10];
        seq[0] = core::prelude::v1::Some(Color::Green);
        seq[1] = core::prelude::v1::Some(Color::Green);
        seq[2] = core::prelude::v1::Some(Color::Green);
        seq[3] = core::prelude::v1::Some(Color::Green);
        seq[4] = core::prelude::v1::Some(Color::Green);
        seq[5] = core::prelude::v1::Some(Color::Green);
        seq[6] = core::prelude::v1::Some(Color::Green);
        seq[7] = core::prelude::v1::Some(Color::Green);
        seq[8] = core::prelude::v1::Some(Color::Green);
        seq[9] = core::prelude::v1::Some(Color::Green);

        let initial_state = State {
            status: Status::RoundSuccess,
            sequence: seq,
            current_sequence_size: SEQ_MAX_LENGTH,
            next_player_try: 8,
        };

        let desired_state = State {
            status: Status::RoundSuccess,
            sequence: seq,
            current_sequence_size: SEQ_MAX_LENGTH,
            next_player_try: 8,
        };

        let resulting_state = state_machine(initial_state, Event::ColorGiven(Color::Green));
        assert_eq!(
            resulting_state.status, desired_state.status,
            "testing state.status resulting: {:#?} desired: {:#?}",
            resulting_state.status, desired_state.status
        );
        assert_eq!(
            resulting_state.sequence, desired_state.sequence,
            "testing state.sequence resulting: {:?} desired: {:?}",
            resulting_state.sequence, desired_state.sequence
        );
        assert_eq!(
            resulting_state.current_sequence_size, desired_state.current_sequence_size,
            "testing state.current_sequence_size resulting: {} desired: {}",
            resulting_state.current_sequence_size, desired_state.current_sequence_size
        );
        assert_eq!(
            resulting_state.next_player_try, desired_state.next_player_try,
            "testing state.next_player_try resulting: {} desired: {}",
            resulting_state.next_player_try, desired_state.next_player_try
        );
    }

    #[test]
    fn test_color_given_event_with_full_sequence_add_do_nothing() {
        let mut seq = [None; 10];
        seq[0] = core::prelude::v1::Some(Color::Green);
        seq[1] = core::prelude::v1::Some(Color::Green);
        seq[2] = core::prelude::v1::Some(Color::Green);
        seq[3] = core::prelude::v1::Some(Color::Green);
        seq[4] = core::prelude::v1::Some(Color::Green);
        seq[5] = core::prelude::v1::Some(Color::Green);
        seq[6] = core::prelude::v1::Some(Color::Green);
        seq[7] = core::prelude::v1::Some(Color::Green);
        seq[8] = core::prelude::v1::Some(Color::Green);
        seq[9] = core::prelude::v1::Some(Color::Green);

        let initial_state = State {
            status: Status::RoundSuccess,
            sequence: seq,
            current_sequence_size: SEQ_MAX_LENGTH + 1,
            next_player_try: 8,
        };

        let desired_state = State {
            status: Status::RoundSuccess,
            sequence: seq,
            current_sequence_size: SEQ_MAX_LENGTH,
            next_player_try: 8,
        };

        let resulting_state = state_machine(initial_state, Event::ColorGiven(Color::Green));
        assert_eq!(
            resulting_state.status, desired_state.status,
            "testing state.status resulting: {:#?} desired: {:#?}",
            resulting_state.status, desired_state.status
        );
        assert_eq!(
            resulting_state.sequence, desired_state.sequence,
            "testing state.sequence resulting: {:?} desired: {:?}",
            resulting_state.sequence, desired_state.sequence
        );
        assert_eq!(
            resulting_state.current_sequence_size, desired_state.current_sequence_size,
            "testing state.current_sequence_size resulting: {} desired: {}",
            resulting_state.current_sequence_size, desired_state.current_sequence_size
        );
        assert_eq!(
            resulting_state.next_player_try, desired_state.next_player_try,
            "testing state.next_player_try resulting: {} desired: {}",
            resulting_state.next_player_try, desired_state.next_player_try
        );
    }

    #[test]
    fn test_start_event() {
        let mut seq = [None; 10];
        seq[0] = core::prelude::v1::Some(Color::Green);

        let initial_state = State {
            status: Status::Startup,
            sequence: seq,
            current_sequence_size: 1,
            next_player_try: 0,
        };

        let desired_state = State {
            status: Status::PlayingSequence,
            sequence: seq,
            current_sequence_size: 1,
            next_player_try: 0,
        };

        let resulting_state = state_machine(initial_state, Event::GameStarted);
        assert_eq!(
            resulting_state.status, desired_state.status,
            "testing state.status resulting: {:#?} desired: {:#?}",
            resulting_state.status, desired_state.status
        );
        assert_eq!(
            resulting_state.sequence, desired_state.sequence,
            "testing state.sequence resulting: {:?} desired: {:?}",
            resulting_state.sequence, desired_state.sequence
        );
        assert_eq!(
            resulting_state.current_sequence_size, desired_state.current_sequence_size,
            "testing state.current_sequence_size resulting: {} desired: {}",
            resulting_state.current_sequence_size, desired_state.current_sequence_size
        );
        assert_eq!(
            resulting_state.next_player_try, desired_state.next_player_try,
            "testing state.next_player_try resulting: {} desired: {}",
            resulting_state.next_player_try, desired_state.next_player_try
        );
    }

    #[test]
    fn test_sequence_playback_completed() {
        let mut seq = [None; 10];
        seq[0] = core::prelude::v1::Some(Color::Green);

        let initial_state = State {
            status: Status::PlayingSequence,
            sequence: seq,
            current_sequence_size: 1,
            next_player_try: 0,
        };

        let desired_state = State {
            status: Status::AwaitingInput,
            sequence: seq,
            current_sequence_size: 1,
            next_player_try: 0,
        };

        let resulting_state = state_machine(initial_state, Event::SequencePlaybackCompleted);
        assert_eq!(
            resulting_state.status, desired_state.status,
            "testing state.status resulting: {:#?} desired: {:#?}",
            resulting_state.status, desired_state.status
        );
        assert_eq!(
            resulting_state.sequence, desired_state.sequence,
            "testing state.sequence resulting: {:?} desired: {:?}",
            resulting_state.sequence, desired_state.sequence
        );
        assert_eq!(
            resulting_state.current_sequence_size, desired_state.current_sequence_size,
            "testing state.current_sequence_size resulting: {} desired: {}",
            resulting_state.current_sequence_size, desired_state.current_sequence_size
        );
        assert_eq!(
            resulting_state.next_player_try, desired_state.next_player_try,
            "testing state.next_player_try resulting: {} desired: {}",
            resulting_state.next_player_try, desired_state.next_player_try
        );
    }

    #[test]
    fn test_color_pressed_event_round_fail() {
        let mut seq = [None; 10];
        seq[0] = core::prelude::v1::Some(Color::Green);

        let initial_state = State {
            status: Status::AwaitingInput,
            sequence: seq,
            current_sequence_size: 1,
            next_player_try: 0,
        };

        let desired_state = State {
            status: Status::GameOver,
            sequence: seq,
            current_sequence_size: 1,
            next_player_try: 0,
        };

        let resulting_state = state_machine(initial_state, Event::ColorPressed(Color::Red));
        assert_eq!(
            resulting_state.status, desired_state.status,
            "testing state.status resulting: {:#?} desired: {:#?}",
            resulting_state.status, desired_state.status
        );
        assert_eq!(
            resulting_state.sequence, desired_state.sequence,
            "testing state.sequence resulting: {:?} desired: {:?}",
            resulting_state.sequence, desired_state.sequence
        );
        assert_eq!(
            resulting_state.current_sequence_size, desired_state.current_sequence_size,
            "testing state.current_sequence_size resulting: {} desired: {}",
            resulting_state.current_sequence_size, desired_state.current_sequence_size
        );
        assert_eq!(
            resulting_state.next_player_try, desired_state.next_player_try,
            "testing state.next_player_try resulting: {} desired: {}",
            resulting_state.next_player_try, desired_state.next_player_try
        );
    }

    #[test]
    fn test_color_pressed_event_round_success() {
        let mut seq = [None; 10];
        seq[0] = core::prelude::v1::Some(Color::Green);

        let initial_state = State {
            status: Status::AwaitingInput,
            sequence: seq,
            current_sequence_size: 1,
            next_player_try: 0,
        };

        let desired_state = State {
            status: Status::RoundSuccess,
            sequence: seq,
            current_sequence_size: 1,
            next_player_try: 0,
        };

        let resulting_state = state_machine(initial_state, Event::ColorPressed(Color::Green));
        assert_eq!(
            resulting_state.status, desired_state.status,
            "testing state.status resulting: {:#?} desired: {:#?}",
            resulting_state.status, desired_state.status
        );
        assert_eq!(
            resulting_state.sequence, desired_state.sequence,
            "testing state.sequence resulting: {:?} desired: {:?}",
            resulting_state.sequence, desired_state.sequence
        );
        assert_eq!(
            resulting_state.current_sequence_size, desired_state.current_sequence_size,
            "testing state.current_sequence_size resulting: {} desired: {}",
            resulting_state.current_sequence_size, desired_state.current_sequence_size
        );
        assert_eq!(
            resulting_state.next_player_try, desired_state.next_player_try,
            "testing state.next_player_try resulting: {} desired: {}",
            resulting_state.next_player_try, desired_state.next_player_try
        );
    }

    #[test]
    fn test_partial_success_keepawaiting_input() {
        let mut seq = [None; 10];
        seq[0] = core::prelude::v1::Some(Color::Green);
        seq[1] = core::prelude::v1::Some(Color::Red);

        let initial_state = State {
            status: Status::AwaitingInput,
            sequence: seq,
            current_sequence_size: 2,
            next_player_try: 0,
        };

        let desired_state = State {
            status: Status::AwaitingInput,
            sequence: seq,
            current_sequence_size: 2,
            next_player_try: 1,
        };

        let resulting_state = state_machine(initial_state, Event::ColorPressed(Color::Green));
        assert_eq!(
            resulting_state.status, desired_state.status,
            "testing state.status resulting: {:#?} desired: {:#?}",
            resulting_state.status, desired_state.status
        );
        assert_eq!(
            resulting_state.sequence, desired_state.sequence,
            "testing state.sequence resulting: {:?} desired: {:?}",
            resulting_state.sequence, desired_state.sequence
        );
        assert_eq!(
            resulting_state.current_sequence_size, desired_state.current_sequence_size,
            "testing state.current_sequence_size resulting: {} desired: {}",
            resulting_state.current_sequence_size, desired_state.current_sequence_size
        );
        assert_eq!(
            resulting_state.next_player_try, desired_state.next_player_try,
            "testing state.next_player_try resulting: {} desired: {}",
            resulting_state.next_player_try, desired_state.next_player_try
        );
    }

    #[test]
    fn test_color_pressed_event_game_success() {
        let mut seq = [None; 10];
        seq[SEQ_MAX_LENGTH - 1] = core::prelude::v1::Some(Color::Green); //Last in sequence

        let initial_state = State {
            status: Status::AwaitingInput,
            sequence: seq,
            current_sequence_size: SEQ_MAX_LENGTH,
            next_player_try: 9,
        };

        let desired_state = State {
            status: Status::GameCompleted,
            sequence: seq,
            current_sequence_size: SEQ_MAX_LENGTH,
            next_player_try: 9,
        };

        let resulting_state = state_machine(initial_state, Event::ColorPressed(Color::Green));
        assert_eq!(
            resulting_state.status, desired_state.status,
            "testing state.status resulting: {:#?} desired: {:#?}",
            resulting_state.status, desired_state.status
        );
        assert_eq!(
            resulting_state.sequence, desired_state.sequence,
            "testing state.sequence resulting: {:?} desired: {:?}",
            resulting_state.sequence, desired_state.sequence
        );
        assert_eq!(
            resulting_state.current_sequence_size, desired_state.current_sequence_size,
            "testing state.current_sequence_size resulting: {} desired: {}",
            resulting_state.current_sequence_size, desired_state.current_sequence_size
        );
        assert_eq!(
            resulting_state.next_player_try, desired_state.next_player_try,
            "testing state.next_player_try resulting: {} desired: {}",
            resulting_state.next_player_try, desired_state.next_player_try
        );
    }

    #[test]
    fn test_two_colors_in_sequence() {
        //Round 1
        let initial_state = State::new();
        let first_color_given_state = state_machine(initial_state, Event::ColorGiven(Color::Green));
        let playing_sequence_state = state_machine(first_color_given_state, Event::GameStarted);
        let awaiting_input_state =
            state_machine(playing_sequence_state, Event::SequencePlaybackCompleted);
        let round_success_state = state_machine(awaiting_input_state, ColorPressed(Color::Green));

        // Round 2
        let second_color_given_state =
            state_machine(round_success_state, Event::ColorGiven(Color::Blue));
        let second_playing_sequence_state =
            state_machine(second_color_given_state, Event::GameStarted);
        let awaiting_input_state = state_machine(
            second_playing_sequence_state,
            Event::SequencePlaybackCompleted,
        );

        let first_color_pressed_state =
            state_machine(awaiting_input_state, ColorPressed(Color::Green));
        let resulting_state = state_machine(first_color_pressed_state, ColorPressed(Color::Blue));

        let mut desired_seq = [None; 10];
        desired_seq[0] = core::prelude::v1::Some(Color::Green);
        desired_seq[1] = core::prelude::v1::Some(Color::Blue);
        let desired_state = State {
            status: Status::RoundSuccess,
            sequence: desired_seq,
            current_sequence_size: 2,
            next_player_try: 0,
        };

        assert_eq!(
            resulting_state.status, desired_state.status,
            "testing state.status resulting: {:#?} desired: {:#?}",
            resulting_state.status, desired_state.status
        );
        assert_eq!(
            resulting_state.sequence, desired_state.sequence,
            "testing state.sequence resulting: {:?} desired: {:?}",
            resulting_state.sequence, desired_state.sequence
        );
        assert_eq!(
            resulting_state.current_sequence_size, desired_state.current_sequence_size,
            "testing state.current_sequence_size resulting: {} desired: {}",
            resulting_state.current_sequence_size, desired_state.current_sequence_size
        );
        assert_eq!(
            resulting_state.next_player_try, desired_state.next_player_try,
            "testing state.next_player_try resulting: {} desired: {}",
            resulting_state.next_player_try, desired_state.next_player_try
        );
    }

    #[test]
    fn test_color_pressed_ignore_states() {
        let mut seq = [None; 10];
        seq[0] = core::prelude::v1::Some(Color::Green); //Last in sequence

        let cases = [
            (
                "should_ignore_game_over",
                State {
                    status: Status::GameOver,
                    sequence: seq,
                    current_sequence_size: 1,
                    next_player_try: 0,
                },
            ),
            (
                "should_ignore_game_completed",
                State {
                    status: Status::GameCompleted,
                    sequence: seq,
                    current_sequence_size: 1,
                    next_player_try: 0,
                },
            ),
            (
                "should_ignore_startup",
                State {
                    status: Status::Startup,
                    sequence: seq,
                    current_sequence_size: 1,
                    next_player_try: 0,
                },
            ),
            (
                "should_ignore_round_success",
                State {
                    status: Status::RoundSuccess,
                    sequence: seq,
                    current_sequence_size: 1,
                    next_player_try: 0,
                },
            ),
            (
                "should_ignore_playing_sequence",
                State {
                    status: Status::PlayingSequence,
                    sequence: seq,
                    current_sequence_size: 1,
                    next_player_try: 0,
                },
            ),
        ];

        for (name, state) in cases {
            let desired_state = state.clone();
            let resulting_state = state_machine(state, Event::ColorPressed(Color::Green));
            assert_eq!(
                resulting_state.status, desired_state.status,
                "testing {} state.status resulting: {:#?} desired: {:#?}",
                name, resulting_state.status, desired_state.status
            );
            assert_eq!(
                resulting_state.sequence, desired_state.sequence,
                "testing {} state.sequence resulting: {:?} desired: {:?}",
                name, resulting_state.sequence, desired_state.sequence
            );
            assert_eq!(
                resulting_state.current_sequence_size, desired_state.current_sequence_size,
                "testing {} state.current_sequence_size resulting: {} desired: {}",
                name, resulting_state.current_sequence_size, desired_state.current_sequence_size
            );
            assert_eq!(
                resulting_state.next_player_try, desired_state.next_player_try,
                "testing {} state.next_player_try resulting: {} desired: {}",
                name, resulting_state.next_player_try, desired_state.next_player_try
            );
        }
    }

    #[test]
    fn test_reset_requested_from_states() {
        let clean_seq = [None; 10];
        let mut one_seq = [None; 10];
        one_seq[0] = core::prelude::v1::Some(Color::Green);
        let mut two_seq = [None; 10];
        two_seq[0] = core::prelude::v1::Some(Color::Green);
        two_seq[1] = core::prelude::v1::Some(Color::Red);

        let cases = [
            (
                "startup",
                State {
                    status: Status::Startup,
                    sequence: clean_seq,
                    current_sequence_size: 0,
                    next_player_try: 0,
                },
            ),
            (
                "playing_sequence_with_1",
                State {
                    status: Status::PlayingSequence,
                    sequence: one_seq,
                    current_sequence_size: 1,
                    next_player_try: 0,
                },
            ),
            (
                "playing_sequence_with_2",
                State {
                    status: Status::PlayingSequence,
                    sequence: two_seq,
                    current_sequence_size: 2,
                    next_player_try: 1,
                },
            ),
            (
                "awaiting_input",
                State {
                    status: Status::AwaitingInput,
                    sequence: one_seq,
                    current_sequence_size: 1,
                    next_player_try: 0,
                },
            ),
            (
                "round_success",
                State {
                    status: Status::RoundSuccess,
                    sequence: one_seq,
                    current_sequence_size: 1,
                    next_player_try: 0,
                },
            ),
            (
                "game_over",
                State {
                    status: Status::GameOver,
                    sequence: one_seq,
                    current_sequence_size: 1,
                    next_player_try: 1,
                },
            ),
            (
                "game_completed",
                State {
                    status: Status::GameCompleted,
                    sequence: two_seq,
                    current_sequence_size: 2,
                    next_player_try: 1,
                },
            ),
        ];

        for (name, initial_state) in cases {
            let desired_state = State {
                status: Status::Startup,
                sequence: [None; 10],
                current_sequence_size: 0,
                next_player_try: 0,
            };

            let resulting_state = state_machine(initial_state, Event::ResetRequested);
            assert_eq!(
                resulting_state.status, desired_state.status,
                "testing reset from state {:#?} state.status resulting: {:#?} desired: {:#?}",
                name, resulting_state.status, desired_state.status
            );
            assert_eq!(
                resulting_state.sequence, desired_state.sequence,
                "testing reset from state {:#?} state.sequence resulting: {:?} desired: {:?}",
                name, resulting_state.sequence, desired_state.sequence
            );
            assert_eq!(
                resulting_state.current_sequence_size, desired_state.current_sequence_size,
                "testing reset from state {:#?} state.current_sequence_size resulting: {} desired: {}",
                name, resulting_state.current_sequence_size, desired_state.current_sequence_size
            );
            assert_eq!(
                resulting_state.next_player_try, desired_state.next_player_try,
                "testing reset from state {:#?} state.next_player_try resulting: {} desired: {}",
                name, resulting_state.next_player_try, desired_state.next_player_try
            );
        }
    }

    #[test]
    fn test_ignore_invalid_events_when_startup() {
        // ColorPressed
        // ResetRequested
        // SequencePlaybackCompleted

        let clean_seq = [None; 10];

        let cases = [
            ("color_pressed", Event::ColorPressed(Color::Green)),
            ("reset_requested", Event::ResetRequested),
            (
                "sequence_playback_completed",
                Event::SequencePlaybackCompleted,
            ),
        ];

        for (name, event) in cases {
            let initial_state = State {
                status: Status::Startup,
                sequence: clean_seq,
                current_sequence_size: 0,
                next_player_try: 0,
            };
            let desired_state = initial_state.clone();
            let resulting_state = state_machine(initial_state, event);
            assert_eq!(
                resulting_state.status, desired_state.status,
                "testing reset from state {:#?} state.status resulting: {:#?} desired: {:#?}",
                name, resulting_state.status, desired_state.status
            );
            assert_eq!(
                resulting_state.sequence, desired_state.sequence,
                "testing reset from state {:#?} state.sequence resulting: {:?} desired: {:?}",
                name, resulting_state.sequence, desired_state.sequence
            );
            assert_eq!(
                resulting_state.current_sequence_size, desired_state.current_sequence_size,
                "testing reset from state {:#?} state.current_sequence_size resulting: {} desired: {}",
                name, resulting_state.current_sequence_size, desired_state.current_sequence_size
            );
            assert_eq!(
                resulting_state.next_player_try, desired_state.next_player_try,
                "testing reset from state {:#?} state.next_player_try resulting: {} desired: {}",
                name, resulting_state.next_player_try, desired_state.next_player_try
            );
        }
    }

    #[test]
    fn test_ignore_invalid_events_when_playing_sequence() {
        // GameStarted
        // ColorGiven
        // ColorPressed
        let mut two_seq = [None; 10];
        two_seq[0] = core::prelude::v1::Some(Color::Green);
        two_seq[1] = core::prelude::v1::Some(Color::Red);

        let cases = [
            ("game_started", Event::GameStarted),
            ("color_given", Event::ColorGiven(Color::Blue)),
            ("color_pressed", Event::ColorPressed(Color::Blue)),
        ];

        for (name, event) in cases {
            let initial_state = State {
                status: Status::PlayingSequence,
                sequence: two_seq,
                current_sequence_size: 2,
                next_player_try: 0,
            };
            let desired_state = initial_state.clone();
            let resulting_state = state_machine(initial_state, event);
            assert_eq!(
                resulting_state.status, desired_state.status,
                "testing reset from state {:#?} state.status resulting: {:#?} desired: {:#?}",
                name, resulting_state.status, desired_state.status
            );
            assert_eq!(
                resulting_state.sequence, desired_state.sequence,
                "testing reset from state {:#?} state.sequence resulting: {:?} desired: {:?}",
                name, resulting_state.sequence, desired_state.sequence
            );
            assert_eq!(
                resulting_state.current_sequence_size, desired_state.current_sequence_size,
                "testing reset from state {:#?} state.current_sequence_size resulting: {} desired: {}",
                name, resulting_state.current_sequence_size, desired_state.current_sequence_size
            );
            assert_eq!(
                resulting_state.next_player_try, desired_state.next_player_try,
                "testing reset from state {:#?} state.next_player_try resulting: {} desired: {}",
                name, resulting_state.next_player_try, desired_state.next_player_try
            );
        }
    }

    #[test]
    fn test_ignore_invalid_events_when_awaiting_input() {
        // GameStarted
        // ColorGiven
        // SequencePlaybackCompleted
        let mut two_seq = [None; 10];
        two_seq[0] = core::prelude::v1::Some(Color::Green);
        two_seq[1] = core::prelude::v1::Some(Color::Red);

        let cases = [
            ("game_started", Event::GameStarted),
            ("color_given", Event::ColorGiven(Color::Blue)),
            (
                "sequence_playback_completed",
                Event::SequencePlaybackCompleted,
            ),
        ];

        for (name, event) in cases {
            let initial_state = State {
                status: Status::AwaitingInput,
                sequence: two_seq,
                current_sequence_size: 2,
                next_player_try: 0,
            };
            let desired_state = initial_state.clone();

            let resulting_state = state_machine(initial_state, event);
            assert_eq!(
                resulting_state.status, desired_state.status,
                "testing reset from state {:#?} state.status resulting: {:#?} desired: {:#?}",
                name, resulting_state.status, desired_state.status
            );
            assert_eq!(
                resulting_state.sequence, desired_state.sequence,
                "testing reset from state {:#?} state.sequence resulting: {:?} desired: {:?}",
                name, resulting_state.sequence, desired_state.sequence
            );
            assert_eq!(
                resulting_state.current_sequence_size, desired_state.current_sequence_size,
                "testing reset from state {:#?} state.current_sequence_size resulting: {} desired: {}",
                name, resulting_state.current_sequence_size, desired_state.current_sequence_size
            );
            assert_eq!(
                resulting_state.next_player_try, desired_state.next_player_try,
                "testing reset from state {:#?} state.next_player_try resulting: {} desired: {}",
                name, resulting_state.next_player_try, desired_state.next_player_try
            );
        }
    }

    #[test]
    fn test_ignore_invalid_events_when_round_success() {
        // ColorPressed
        // SequencePlaybackCompleted

        let mut two_seq = [None; 10];
        two_seq[0] = core::prelude::v1::Some(Color::Green);
        two_seq[1] = core::prelude::v1::Some(Color::Red);

        let cases = [
            ("color_pressed", Event::ColorPressed(Color::Blue)),
            (
                "sequence_playback_completed",
                Event::SequencePlaybackCompleted,
            ),
        ];

        for (name, event) in cases {
            let initial_state = State {
                status: Status::RoundSuccess,
                sequence: two_seq,
                current_sequence_size: 2,
                next_player_try: 1,
            };
            let desired_state = initial_state.clone();
            let resulting_state = state_machine(initial_state, event);
            assert_eq!(
                resulting_state.status, desired_state.status,
                "testing reset from state {:#?} state.status resulting: {:#?} desired: {:#?}",
                name, resulting_state.status, desired_state.status
            );
            assert_eq!(
                resulting_state.sequence, desired_state.sequence,
                "testing reset from state {:#?} state.sequence resulting: {:?} desired: {:?}",
                name, resulting_state.sequence, desired_state.sequence
            );
            assert_eq!(
                resulting_state.current_sequence_size, desired_state.current_sequence_size,
                "testing reset from state {:#?} state.current_sequence_size resulting: {} desired: {}",
                name, resulting_state.current_sequence_size, desired_state.current_sequence_size
            );
            assert_eq!(
                resulting_state.next_player_try, desired_state.next_player_try,
                "testing reset from state {:#?} state.next_player_try resulting: {} desired: {}",
                name, resulting_state.next_player_try, desired_state.next_player_try
            );
        }
    }

    #[test]
    fn test_ignore_invalid_events_when_game_over() {
        // GameStarted
        // ColorGiven
        // ColorPressed
        // SequencePlaybackCompleted
        let mut two_seq = [None; 10];
        two_seq[0] = core::prelude::v1::Some(Color::Green);
        two_seq[1] = core::prelude::v1::Some(Color::Red);

        let cases = [
            ("game_started", Event::GameStarted),
            ("color_given", Event::ColorGiven(Color::Blue)),
            ("color_pressed", Event::ColorPressed(Color::Blue)),
            (
                "sequence_playback_completed",
                Event::SequencePlaybackCompleted,
            ),
        ];

        for (name, event) in cases {
            let initial_state = State {
                status: Status::GameOver,
                sequence: two_seq,
                current_sequence_size: 2,
                next_player_try: 1,
            };
            let desired_state = initial_state.clone();
            let resulting_state = state_machine(initial_state, event);
            assert_eq!(
                resulting_state.status, desired_state.status,
                "testing reset from state {:#?} state.status resulting: {:#?} desired: {:#?}",
                name, resulting_state.status, desired_state.status
            );
            assert_eq!(
                resulting_state.sequence, desired_state.sequence,
                "testing reset from state {:#?} state.sequence resulting: {:?} desired: {:?}",
                name, resulting_state.sequence, desired_state.sequence
            );
            assert_eq!(
                resulting_state.current_sequence_size, desired_state.current_sequence_size,
                "testing reset from state {:#?} state.current_sequence_size resulting: {} desired: {}",
                name, resulting_state.current_sequence_size, desired_state.current_sequence_size
            );
            assert_eq!(
                resulting_state.next_player_try, desired_state.next_player_try,
                "testing reset from state {:#?} state.next_player_try resulting: {} desired: {}",
                name, resulting_state.next_player_try, desired_state.next_player_try
            );
        }
    }

    #[test]
    fn test_ignore_invalid_events_when_game_completed() {
        // GameStarted
        // ColorGiven
        // ColorPressed
        // SequencePlaybackCompleted
        let mut two_seq = [None; 10];
        two_seq[0] = core::prelude::v1::Some(Color::Green);
        two_seq[1] = core::prelude::v1::Some(Color::Red);

        let cases = [
            ("game_started", Event::GameStarted),
            ("color_given", Event::ColorGiven(Color::Blue)),
            ("color_pressed", Event::ColorPressed(Color::Blue)),
            (
                "sequence_playback_completed",
                Event::SequencePlaybackCompleted,
            ),
        ];

        for (name, event) in cases {
            let initial_state = State {
                status: Status::GameCompleted,
                sequence: two_seq,
                current_sequence_size: 2,
                next_player_try: 1,
            };
            let desired_state = initial_state.clone();
            let resulting_state = state_machine(initial_state, event);
            assert_eq!(
                resulting_state.status, desired_state.status,
                "testing reset from state {:#?} state.status resulting: {:#?} desired: {:#?}",
                name, resulting_state.status, desired_state.status
            );
            assert_eq!(
                resulting_state.sequence, desired_state.sequence,
                "testing reset from state {:#?} state.sequence resulting: {:?} desired: {:?}",
                name, resulting_state.sequence, desired_state.sequence
            );
            assert_eq!(
                resulting_state.current_sequence_size, desired_state.current_sequence_size,
                "testing reset from state {:#?} state.current_sequence_size resulting: {} desired: {}",
                name, resulting_state.current_sequence_size, desired_state.current_sequence_size
            );
            assert_eq!(
                resulting_state.next_player_try, desired_state.next_player_try,
                "testing reset from state {:#?} state.next_player_try resulting: {} desired: {}",
                name, resulting_state.next_player_try, desired_state.next_player_try
            );
        }
    }
}
