use futures::{stream, TryStreamExt};
use log::info;
use tokio::sync::watch::error::RecvError;

use super::{
    Error, LocalRun, LocalRunAsync, Receiver, Run, RunAsync, Sender, SharedWorld, ThreadRun,
    ThreadRunAsync,
};

/// Long running task of a `System` that is executed in a separate thread.
pub async fn execute_thread<E>(
    name: String,
    mut run: ThreadRun,
    sender: Sender,
    receivers: Vec<Receiver>,
    world: SharedWorld,
) -> Result<(), Error<E>>
where
    E: std::fmt::Debug,
{
    info!("System started: {}", &name);

    execute_inner(run.as_mut(), sender, receivers, world).await?;

    info!("System finished: {}", &name);

    Ok(())
}

/// Long running task of a `System` that is executed in the thread local
/// context.
pub async fn execute_local<E>(
    name: String,
    mut run: LocalRun,
    sender: Sender,
    receivers: Vec<Receiver>,
    world: SharedWorld,
) -> Result<(), Error<E>>
where
    E: std::fmt::Debug,
{
    info!("System started (local): {}", &name);

    execute_inner(run.as_mut(), sender, receivers, world).await?;

    info!("System finished (local): {}", &name);

    Ok(())
}

/// Long running task of a `System` that is executed in a separate thread.
pub async fn execute_thread_async<E>(
    name: String,
    mut run: ThreadRunAsync<E>,
    sender: Sender,
    receivers: Vec<Receiver>,
    world: SharedWorld,
) -> Result<(), Error<E>>
where
    E: std::fmt::Debug,
{
    info!("System started: {}", &name);

    execute_inner_async(run.as_mut(), sender, receivers, world).await?;

    info!("System finished: {}", &name);

    Ok(())
}

/// Long running task of a `System` that is executed in the thread local
/// context.
pub async fn execute_local_async<E>(
    name: String,
    mut run: LocalRunAsync<E>,
    sender: Sender,
    receivers: Vec<Receiver>,
    world: SharedWorld,
) -> Result<(), Error<E>>
where
    E: std::fmt::Debug,
{
    info!("System started (local): {}", &name);

    execute_inner_async(run.as_mut(), sender, receivers, world).await?;

    info!("System finished (local): {}", &name);

    Ok(())
}

/// Actual tasks that is running the system.
async fn execute_inner<R, E>(
    run: &mut R,
    sender: Sender,
    mut receivers: Vec<Receiver>,
    world: SharedWorld,
) -> Result<(), Error<E>>
where
    R: for<'a> Run<'a> + ?Sized,
    E: std::fmt::Debug,
{
    loop {
        stream::iter(receivers.iter_mut().map(Result::<_, RecvError>::Ok))
            .try_for_each(Receiver::changed)
            .await
            .map_err(Error::SystemNotifyReceive)?;

        run.run(&world);

        sender.send(()).map_err(Error::SystemNotifySend)?;
    }
}

/// Actual tasks that is running the system.
async fn execute_inner_async<R, E>(
    run: &mut R,
    sender: Sender,
    mut receivers: Vec<Receiver>,
    world: SharedWorld,
) -> Result<(), Error<E>>
where
    R: for<'a> RunAsync<'a, Error = E> + ?Sized,
    E: std::fmt::Debug,
{
    loop {
        stream::iter(receivers.iter_mut().map(Result::<_, RecvError>::Ok))
            .try_for_each(Receiver::changed)
            .await
            .map_err(Error::SystemNotifyReceive)?;

        run.run(&world).await.map_err(Error::SystemRunError)?;

        sender.send(()).map_err(Error::SystemNotifySend)?;
    }
}
