use std::fmt::Debug;

use thiserror::Error;
use tokio::sync::watch::error::{RecvError, SendError};

#[derive(Error, Debug)]
pub enum Error<E>
where
    E: Debug,
{
    #[error("A System with this name was already registered: {0}!")]
    NameAlreadyRegistered(String),

    #[error("Dependency of the given system was not found: {0}!")]
    DependencyWasNotFound(String),

    #[error("Unable to start dispatching!")]
    DispatchSend(SendError<()>),

    #[error("Unable to wait for systems to finish!")]
    DispatchReceive(RecvError),

    #[error("Unable to start dispatching!")]
    SystemNotifySend(SendError<()>),

    #[error("Unable to wait for systems to finish")]
    SystemNotifyReceive(RecvError),

    #[error("System `async_run` returned an error.")]
    SystemRunError(E),
}
