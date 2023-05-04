/// A trait to be implemented by use cases.
pub trait UseCase {
    /// The input to the use case.
    type Input;

    /// The output of the use case.
    type Output;
}

/// A trait to be implemented by use case handlers.
///
/// # Parameters
///
/// * `U` - The type of the use case to be handled.
/// * `S` - The type of the state of the application.
#[async_trait::async_trait]
pub trait UseCaseHandler<U: UseCase, S> {
    /// The type of the error that can occur during the use case execution.
    type Error: Send + Sync;

    /// Asynchronously executes the use case.
    ///
    /// # Parameters
    /// - `input` - The input to the use case.
    /// - `state` - The state of the application.
    ///
    /// # Returns
    /// The output of the use case.
    async fn execute(
        input: U::Input,
        state: &S,
    ) -> Result<U::Output, Self::Error>;
}

#[cfg(test)]
mod tests {
    use crate::domain::UseCase;
    use reddd_macros::UseCase;

    #[derive(UseCase)]
    #[usecase(input = "String", output = "i32")]
    struct SampleUseCase;
}
