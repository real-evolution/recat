use chrono::Duration;

/// Parses a string into a `chrono::Duration` in seconds.
///
/// # Arguments
///
/// * `arg` - The string to parse.
///
/// # Errors
///
/// Returns an error if the string cannot be parsed into a `i64`.
pub(super) fn parse_duration(arg: &str) -> anyhow::Result<Duration> {
    let seconds = arg.parse()?;

    Ok(Duration::seconds(seconds))
}
