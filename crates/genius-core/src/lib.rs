#![no_std]

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
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Status {
    Startup,
    PlayingSequence,
    AwaitingInput,
    RoundSuccess,
    GameOver,
    GameCompleted
}
#[derive(PartialEq)]
pub enum Event {
  GameStarted,
  ColorGiven,
  ColorPressed,
  ResetRequested,
  SequencePlaybackCompleted,
}

#[derive(Debug)]
pub struct State {
  status: Status,
  sequence: [u8; 10],
  current_sequence_size: usize,
  next_player_try: usize,
}

pub fn state_machine(state: &mut State, event: Event, color: u8) -> State {
  match event {
    Event::ColorGiven => {
      let i = state.current_sequence_size;
      state.sequence[i] = color;
      State {
        status: state.status.clone(),
        sequence: state.sequence,
        current_sequence_size: i + 1,
        next_player_try: state.next_player_try
      }
    }

    Event::GameStarted => {
      State {
        status: Status::PlayingSequence,
        sequence: state.sequence,
        current_sequence_size: state.current_sequence_size,
        next_player_try: state.next_player_try,
      }
    }

    Event::SequencePlaybackCompleted => {
      State {
        status: Status::AwaitingInput,
        sequence: state.sequence,
        current_sequence_size: state.current_sequence_size,
        next_player_try: state.next_player_try
      }
    }

    Event::ResetRequested => {
      State {
        status: Status::Startup,
        sequence: [0; 10],
        current_sequence_size: 0,
        next_player_try: 0,
      }
    }
    
    Event::ColorPressed => {
      if state.status == Status::PlayingSequence {
        return State {
          status: state.status.clone(),
          sequence: state.sequence,
          current_sequence_size: state.current_sequence_size,
          next_player_try: state.next_player_try,
        }
      }

      if state.sequence[state.next_player_try] != color {
        return State {
          status: Status::GameOver,
          sequence: state.sequence,
          current_sequence_size: state.current_sequence_size,
          next_player_try: state.next_player_try,
        }
      }

      // o tamanho total do array de sequencia
      if state.next_player_try == 9 {
        return State {
          status: Status::GameCompleted,
          sequence: state.sequence,
          current_sequence_size: state.current_sequence_size,
          next_player_try: state.next_player_try,
        }
      }


      State {
        status: Status::RoundSuccess,
        sequence: state.sequence,
        current_sequence_size: state.current_sequence_size,
        next_player_try: state.next_player_try + 1,
      }
    }
  }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_given_event() {
        let mut initial_state = State {
          status: Status::Startup,
          sequence: [0; 10],
          current_sequence_size: 0,
          next_player_try: 0,
        };

        let mut seq = [0; 10];
        seq[0] = b'G';

        let desired_state = State {
          status: Status::Startup,
          sequence: seq,
          current_sequence_size: 1,
          next_player_try: 0,
        };

        let resulting_state = state_machine(&mut initial_state, Event::ColorGiven, b'G');
        assert_eq!(resulting_state.status, desired_state.status, "testing state.status resulting: {:#?} desired: {:#?}", resulting_state.status, desired_state.status);
        assert_eq!(resulting_state.sequence, desired_state.sequence,"testing state.sequence resulting: {:?} desired: {:?}", resulting_state.sequence, desired_state.sequence);
        assert_eq!(resulting_state.current_sequence_size, desired_state.current_sequence_size,"testing state.current_sequence_size resulting: {} desired: {}", resulting_state.current_sequence_size, desired_state.current_sequence_size);
        assert_eq!(resulting_state.next_player_try, desired_state.next_player_try,"testing state.next_player_try resulting: {} desired: {}", resulting_state.next_player_try, desired_state.next_player_try);
    }

    #[test]
    fn test_start_event() {
        let mut seq = [0; 10];
        seq[0] = b'G';

        let mut initial_state = State {
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

        let resulting_state = state_machine(&mut initial_state, Event::GameStarted, 0);
        assert_eq!(resulting_state.status, desired_state.status, "testing state.status resulting: {:#?} desired: {:#?}", resulting_state.status, desired_state.status);
        assert_eq!(resulting_state.sequence, desired_state.sequence,"testing state.sequence resulting: {:?} desired: {:?}", resulting_state.sequence, desired_state.sequence);
        assert_eq!(resulting_state.current_sequence_size, desired_state.current_sequence_size,"testing state.current_sequence_size resulting: {} desired: {}", resulting_state.current_sequence_size, desired_state.current_sequence_size);
        assert_eq!(resulting_state.next_player_try, desired_state.next_player_try,"testing state.next_player_try resulting: {} desired: {}", resulting_state.next_player_try, desired_state.next_player_try);
    }

    #[test]
    fn test_sequence_playback_completed() {
        let mut seq = [0; 10];
        seq[0] = b'G';

        let mut initial_state = State {
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

        let resulting_state = state_machine(&mut initial_state, Event::SequencePlaybackCompleted, 0);
        assert_eq!(resulting_state.status, desired_state.status, "testing state.status resulting: {:#?} desired: {:#?}", resulting_state.status, desired_state.status);
        assert_eq!(resulting_state.sequence, desired_state.sequence,"testing state.sequence resulting: {:?} desired: {:?}", resulting_state.sequence, desired_state.sequence);
        assert_eq!(resulting_state.current_sequence_size, desired_state.current_sequence_size,"testing state.current_sequence_size resulting: {} desired: {}", resulting_state.current_sequence_size, desired_state.current_sequence_size);
        assert_eq!(resulting_state.next_player_try, desired_state.next_player_try,"testing state.next_player_try resulting: {} desired: {}", resulting_state.next_player_try, desired_state.next_player_try);
    }

    #[test]
    fn test_color_pressed_event_round_fail() {
        let mut seq = [0; 10];
        seq[0] = b'G';

        let mut initial_state = State {
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

        let resulting_state = state_machine(&mut initial_state, Event::ColorPressed, b'R');
        assert_eq!(resulting_state.status, desired_state.status, "testing state.status resulting: {:#?} desired: {:#?}", resulting_state.status, desired_state.status);
        assert_eq!(resulting_state.sequence, desired_state.sequence,"testing state.sequence resulting: {:?} desired: {:?}", resulting_state.sequence, desired_state.sequence);
        assert_eq!(resulting_state.current_sequence_size, desired_state.current_sequence_size,"testing state.current_sequence_size resulting: {} desired: {}", resulting_state.current_sequence_size, desired_state.current_sequence_size);
        assert_eq!(resulting_state.next_player_try, desired_state.next_player_try,"testing state.next_player_try resulting: {} desired: {}", resulting_state.next_player_try, desired_state.next_player_try);
    }

    #[test]
    fn test_color_pressed_event_round_success() {
        let mut seq = [0; 10];
        seq[0] = b'G';

        let mut initial_state = State {
          status: Status::AwaitingInput,
          sequence: seq,
          current_sequence_size: 1,
          next_player_try: 0,
        };

        let desired_state = State {
          status: Status::RoundSuccess,
          sequence: seq,
          current_sequence_size: 1,
          next_player_try: 1,
        };

        let resulting_state = state_machine(&mut initial_state, Event::ColorPressed, b'G');
        assert_eq!(resulting_state.status, desired_state.status, "testing state.status resulting: {:#?} desired: {:#?}", resulting_state.status, desired_state.status);
        assert_eq!(resulting_state.sequence, desired_state.sequence,"testing state.sequence resulting: {:?} desired: {:?}", resulting_state.sequence, desired_state.sequence);
        assert_eq!(resulting_state.current_sequence_size, desired_state.current_sequence_size,"testing state.current_sequence_size resulting: {} desired: {}", resulting_state.current_sequence_size, desired_state.current_sequence_size);
        assert_eq!(resulting_state.next_player_try, desired_state.next_player_try,"testing state.next_player_try resulting: {} desired: {}", resulting_state.next_player_try, desired_state.next_player_try);
    }

    #[test]
    fn test_color_pressed_event_game_success() {
        let mut seq = [0; 10];
        seq[9] = b'G';//Last in sequence

        let mut initial_state = State {
          status: Status::AwaitingInput,
          sequence: seq,
          current_sequence_size: 10,
          next_player_try: 9,
        };

        let desired_state = State {
          status: Status::GameCompleted,
          sequence: seq,
          current_sequence_size: 10,
          next_player_try: 9,
        };

        let resulting_state = state_machine(&mut initial_state, Event::ColorPressed, b'G');
        assert_eq!(resulting_state.status, desired_state.status, "testing state.status resulting: {:#?} desired: {:#?}", resulting_state.status, desired_state.status);
        assert_eq!(resulting_state.sequence, desired_state.sequence,"testing state.sequence resulting: {:?} desired: {:?}", resulting_state.sequence, desired_state.sequence);
        assert_eq!(resulting_state.current_sequence_size, desired_state.current_sequence_size,"testing state.current_sequence_size resulting: {} desired: {}", resulting_state.current_sequence_size, desired_state.current_sequence_size);
        assert_eq!(resulting_state.next_player_try, desired_state.next_player_try,"testing state.next_player_try resulting: {} desired: {}", resulting_state.next_player_try, desired_state.next_player_try);
    }

    #[test]
    fn test_color_pressed_ignore_playing_sequence() {
        let mut seq = [0; 10];
        seq[0] = b'G';//Last in sequence

        let mut desired_state = State {
          status: Status::PlayingSequence,
          sequence: seq,
          current_sequence_size: 1,
          next_player_try: 0,
        };

        let resulting_state = state_machine(&mut desired_state, Event::ColorPressed, b'G');
        assert_eq!(resulting_state.status, desired_state.status, "testing state.status resulting: {:#?} desired: {:#?}", resulting_state.status, desired_state.status);
        assert_eq!(resulting_state.sequence, desired_state.sequence,"testing state.sequence resulting: {:?} desired: {:?}", resulting_state.sequence, desired_state.sequence);
        assert_eq!(resulting_state.current_sequence_size, desired_state.current_sequence_size,"testing state.current_sequence_size resulting: {} desired: {}", resulting_state.current_sequence_size, desired_state.current_sequence_size);
        assert_eq!(resulting_state.next_player_try, desired_state.next_player_try,"testing state.next_player_try resulting: {} desired: {}", resulting_state.next_player_try, desired_state.next_player_try);
    }

    #[test]
    fn test_reset_requested_from_playing_sequence() {
      let mut seq = [0; 10];
        seq[0] = b'G';

        let mut initial_state = State {
          status: Status::PlayingSequence,
          sequence: seq,
          current_sequence_size: 1,
          next_player_try: 0,
        };

        let desired_state = State {
          status: Status::Startup,
          sequence: [0; 10],
          current_sequence_size: 0,
          next_player_try: 0,
        };

        let resulting_state = state_machine(&mut initial_state, Event::ResetRequested, b'G');
        assert_eq!(resulting_state.status, desired_state.status, "testing state.status resulting: {:#?} desired: {:#?}", resulting_state.status, desired_state.status);
        assert_eq!(resulting_state.sequence, desired_state.sequence,"testing state.sequence resulting: {:?} desired: {:?}", resulting_state.sequence, desired_state.sequence);
        assert_eq!(resulting_state.current_sequence_size, desired_state.current_sequence_size,"testing state.current_sequence_size resulting: {} desired: {}", resulting_state.current_sequence_size, desired_state.current_sequence_size);
        assert_eq!(resulting_state.next_player_try, desired_state.next_player_try,"testing state.next_player_try resulting: {} desired: {}", resulting_state.next_player_try, desired_state.next_player_try);
    }
}
