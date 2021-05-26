use crate::state::State;
use std::path::Path;
use tokio::sync::watch::{channel, Receiver, Sender};

/// StateManger job is to pull for state changes from DeepNet Operator and make that available to the rest of the site as a [State].
///
/// * Have api that the services and managers can read state and get notified of state changes. TODO
/// * Pull DeepNet Operator for new state. TODO
///   * Pull 1 - 1.5 sec after successful response.
///   * Pull 1 - 1.5 minutes after unsuccessful response.
///   * Randomize wait spreed out load on cloud server.
/// * Save latest state to disk. Encrypted. TODO
/// * Load latest state from disk before asking Operator. TODO
/// * Have a way to set state when testing.
pub struct StateManger {
    _sender: Sender<State>,
    receiver: Receiver<State>,
}

impl Default for StateManger {
    fn default() -> Self {
        Self::new()
    }
}

impl StateManger {
    /// Create a new StateManger with default state.
    pub fn new() -> Self {
        let (s, r) = channel(Default::default());
        Self {
            _sender: s,
            receiver: r,
        }
    }

    /// Get a receiver that allows query of current state and awaiting state change.
    ///
    /// Other services and managers should only require a [Receiver<State>] and not access to the [StateManager] directly.
    pub fn watch(&self) -> Receiver<State> {
        self.receiver.clone()
    }

    /// Set the file to be used as local persistent store.
    ///
    /// Will load state from disk. And then enable saving the last state to disk. Encrypted using the provided key.
    ///
    /// TODO: [_key] need a better type.
    /// TODO: Create a better error type.
    pub async fn set_file(&mut self, _path: &Path, _key: &[u8]) -> Result<(), Error> {
        todo!()
    }

    /// Start pulling the operator API for changes.
    ///
    /// Will return as soon as it can be determined that the url and token is correct.
    /// Runs updates as a background task.
    /// TODO: _url and _token needs better types.
    /// TODO: Needs better Error type.
    /// TODO: Need to think of a different api/design. Rust will not like a &mut self here.
    pub async fn start_pull(&mut self, _url: &str, _token: &str) -> Result<(), Error> {
        todo!()
    }
}

///Placeholder for better error type.
pub struct Error;

#[cfg(test)]
mod tests {
    use tokio::sync::watch::error::SendError;

    use super::*;

    impl StateManger {
        /// Apply a new state to be used and source of truth.State
        ///
        /// Only to be used in unit testing.
        pub fn set_state(&self, state: State) -> Result<(), SendError<State>> {
            self._sender.send(state)
        }
    }

    #[test]
    fn start_with_defaults() {
        let state = StateManger::new();
        assert_eq!(*state.watch().borrow(), Default::default());
    }

    #[tokio::test]
    async fn notification_on_state_change() {
        let state = StateManger::new();
        let mut watch = state.watch();
        let alert = watch.changed();

        state.set_state(Default::default()).unwrap();
        
        alert.await.unwrap();
    }
}
