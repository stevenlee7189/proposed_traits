///
/// This represents a common set of syste, control  operation errors. Implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    ClockNotFound,
    ClockAlreadyEnabled,
    ClockAlreadyDisabled,
    InvalidClockFrequency,
    ClockConfigurationFailed,
    InvalidResetId,
    HardwareFailure,
    PermissionDenied,
    Timeout,
}



pub trait Error: core::fmt::Debug {
    /// Convert error to a generic error kind
    ///
    /// By using this method, errors freely defined by Algo implementations
    /// can be converted to a set of generic errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    /// Convert error to a generic Mac error kind.
    ///
    /// By using this method, Mac errors freely defined by Algo implementations
    /// can be converted to a set of generic I2C errors upon which generic
    /// code can act.    
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

pub trait ErrorType {
    /// Error type.
    type Error: Error;
}


/// Trait for clock control operations.
/// Abstracts enabling, disabling, and configuring clocks for peripherals or system components.
pub trait ClockControl: Send + Sync + ErrorType {
    /// Type for identifying a clock (e.g., peripheral ID, clock name, or register offset).
    type ClockId: Clone + PartialEq;
    /// Type for configuring a clock.
    type ClockConfig: PartialEq;

    /// Enables a clock for the specified clock ID.
    ///
    /// # Arguments
    ///
    /// * `clock_id` - A reference to the identifier of the clock to enable.
    ///
    /// # Returns
    ///
    /// * `Result<(), Self::Error>` - Ok if the operation is successful, or an error of type `Self::Error`.
    fn enable(&self, clock_id: &Self::ClockId) -> Result<(), Self::Error>;

    /// Disables a clock for the specified clock ID.
    ///
    /// # Arguments
    ///
    /// * `clock_id` - A reference to the identifier of the clock to disable.
    ///
    /// # Returns
    ///
    /// * `Result<(), Self::Error>` - Ok if the operation is successful, or an error of type `Self::Error`.
    fn disable(&self, clock_id: &Self::ClockId) -> Result<(), Self::Error>;

    /// Sets the frequency of a clock (in Hz).
    ///
    /// # Arguments
    ///
    /// * `clock_id` - A reference to the identifier of the clock to set the frequency for.
    /// * `frequency_hz` - The frequency to set, in Hertz.
    ///
    /// # Returns
    ///
    /// * `Result<(), Self::Error>` - Ok if the operation is successful, or an error of type `Self::Error`.
    fn set_frequency(
        &self,
        clock_id: &Self::ClockId,
        frequency_hz: u64,
    ) -> Result<(), Self::Error>;

    /// Gets the current frequency of a clock (in Hz).
    ///
    /// # Arguments
    ///
    /// * `clock_id` - A reference to the identifier of the clock to get the frequency for.
    ///
    /// # Returns
    ///
    /// * `Result<u64, Self::Error>` - Ok with the current frequency in Hertz, or an error of type `Self::Error`.
    fn get_frequency(&self, clock_id: &Self::ClockId) -> Result<u64, Self::Error>;

    /// Configures clock-specific parameters (e.g., divider, source).
    /// Vendor-specific parameters can be passed via `ClockConfig`.
    ///
    /// # Arguments
    ///
    /// * `clock_id` - A reference to the identifier of the clock to configure.
    /// * `config` - The configuration parameters for the clock.
    ///
    /// # Returns
    ///
    /// * `Result<(), Self::Error>` - Ok if the operation is successful, or an error of type `Self::Error`.
    fn configure(
        &self,
        clock_id: &Self::ClockId,
        config: Self::ClockConfig,
    ) -> Result<(), Self::Error>;

    /// Retrieves the current configuration of a clock.
    ///
    /// # Arguments
    ///
    /// * `clock_id` - A reference to the identifier of the clock to get the configuration for.
    ///
    /// # Returns
    ///
    /// * `Result<Self::ClockConfig, Self::Error>` - Ok with the current configuration, or an error of type `Self::Error`.
    fn get_config(&self, clock_id: &Self::ClockId) -> Result<Self::ClockConfig, Self::Error>;
}

/// Trait for reset control operations.
/// Abstracts asserting and deasserting reset signals for peripherals or system components.
pub trait ResetControl: Send + Sync + ErrorType {
    /// Type for identifying a reset line (e.g., peripheral ID, reset name, or register offset).
    type ResetId: Clone + PartialEq;

    /// Asserts the reset signal for the specified reset ID (holds the component in reset).
    ///
    /// # Arguments
    ///
    /// * `reset_id` - A reference to the identifier of the reset line to assert.
    ///
    /// # Returns
    ///
    /// * `Result<(), Self::Error>` - Ok if the operation is successful, or an error of type `Self::Error`.
    fn reset_assert(&self, reset_id: &Self::ResetId) -> Result<(), Self::Error>;

    /// Deasserts the reset signal for the specified reset ID (releases the component from reset).
    ///
    /// # Arguments
    ///
    /// * `reset_id` - A reference to the identifier of the reset line to deassert.
    ///
    /// # Returns
    ///
    /// * `Result<(), Self::Error>` - Ok if the operation is successful, or an error of type `Self::Error`.
    fn reset_deassert(&self, reset_id: &Self::ResetId) -> Result<(), Self::Error>;

    /// Performs a pulse reset (asserts then deasserts) with a specified duration (in microseconds).
    ///
    /// # Arguments
    ///
    /// * `reset_id` - A reference to the identifier of the reset line to pulse.
    /// * `duration_us` - The duration of the pulse in microseconds.
    ///
    /// # Returns
    ///
    /// * `Result<(), Self::Error>` - Ok if the operation is successful, or an error of type `Self::Error`.
    fn reset_pulse(&self, reset_id: &Self::ResetId, duration_us: u64) -> Result<(), Self::Error>;

    /// Checks if the reset signal is currently asserted for the specified reset ID.
    ///
    /// # Arguments
    ///
    /// * `reset_id` - A reference to the identifier of the reset line to check.
    ///
    /// # Returns
    ///
    /// * `Result<bool, Self::Error>` - Ok with a boolean indicating if the reset is asserted, or an error of type `Self::Error`.
    fn reset_is_asserted(&self, reset_id: &Self::ResetId) -> Result<bool, Self::Error>;
}
