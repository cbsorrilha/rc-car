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
pub enum Status {
    Startup,
    PlayingSequence,
    AwaitingInput,
    RoundSuccess,
    RoundError,
    GameOver,
    GameCompleted
}
#[derive(PartialEq)]
pub enum Event {
  GameStarted,
  ColorGiven,
  // ColorPressed,
  // SequencePlaybackCompleted,
  // ResetRequested
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
    Event::GameStarted => {
      State {
        status: Status::Startup,
        sequence: [0; 10],
        current_sequence_size: 0,
        next_player_try: 0,
      }
    }
    Event::ColorGiven => {
      let i = state.current_sequence_size;
      state.sequence[i] = color;
      State {
        status: Status::PlayingSequence,
        sequence: state.sequence,
        current_sequence_size: i + 1,
        next_player_try: state.next_player_try
      }
    }
  }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_event() {
        let mut state = State {
          status: Status::Startup,
          sequence: [0; 10],
          current_sequence_size: 0,
          next_player_try: 0,
        };

        let resulting_state = state_machine(&mut state, Event::GameStarted, 0);
        assert_eq!(resulting_state.status, state.status);
        assert_eq!(resulting_state.sequence, state.sequence);
        assert_eq!(resulting_state.current_sequence_size, state.current_sequence_size);
        assert_eq!(resulting_state.next_player_try, state.next_player_try);
    }

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
          status: Status::PlayingSequence,
          sequence: seq,
          current_sequence_size: 1,
          next_player_try: 0,
        };

        let resulting_state = state_machine(&mut initial_state, Event::ColorGiven, b'G');
        assert_eq!(resulting_state.status, desired_state.status);
        assert_eq!(resulting_state.sequence, desired_state.sequence);
        assert_eq!(resulting_state.current_sequence_size, desired_state.current_sequence_size);
        assert_eq!(resulting_state.next_player_try, desired_state.next_player_try);
    }
}
