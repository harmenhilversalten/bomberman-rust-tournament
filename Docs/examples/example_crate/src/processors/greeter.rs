//! Greeter processor combining services.
use crate::error::Result;
use crate::providers::UuidProvider;
use crate::services::GreetingService;

/// Processor combining a [`GreetingService`] and [`UuidProvider`].
///
/// # Examples
///
/// ```
/// use example_crate::processors::GreeterProcessor;
/// use example_crate::services::DefaultGreetingService;
/// use example_crate::providers::DefaultUuidProvider;
/// let proc = GreeterProcessor::new(&DefaultGreetingService, &DefaultUuidProvider);
/// let out = proc.process("Bob").unwrap();
/// assert!(out.starts_with("Hello, Bob!"));
/// ```
pub struct GreeterProcessor<'a> {
    greeting: &'a dyn GreetingService,
    uuid: &'a dyn UuidProvider,
}

#[allow(clippy::elidable_lifetime_names)]
impl<'a> GreeterProcessor<'a> {
    /// Create a new processor.
    ///
    /// # Examples
    ///
    /// ```
    /// use example_crate::processors::GreeterProcessor;
    /// use example_crate::services::DefaultGreetingService;
    /// use example_crate::providers::DefaultUuidProvider;
    /// let _proc = GreeterProcessor::new(&DefaultGreetingService, &DefaultUuidProvider);
    /// ```
    #[must_use]
    pub fn new(greeting: &'a dyn GreetingService, uuid: &'a dyn UuidProvider) -> Self {
        Self { greeting, uuid }
    }

    /// Produce a greeting with a UUID suffix.
    ///
    /// # Examples
    ///
    /// ```
    /// use example_crate::processors::GreeterProcessor;
    /// use example_crate::services::DefaultGreetingService;
    /// use example_crate::providers::DefaultUuidProvider;
    /// let proc = GreeterProcessor::new(&DefaultGreetingService, &DefaultUuidProvider);
    /// let msg = proc.process("Bob").unwrap();
    /// assert!(msg.contains("Bob"));
    /// ```
    ///
    /// # Errors
    ///
    /// Propagates errors from the [`GreetingService`] or [`UuidProvider`].
    pub fn process(&self, name: &str) -> Result<String> {
        let msg = self.greeting.greeting(name)?;
        let id = self.uuid.uuid()?;
        let out = format!("{msg} ({id})");
        #[cfg(feature = "tracing")]
        tracing::info!(message = %out, "generated greeting");
        Ok(out)
    }
}
