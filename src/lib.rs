use lazy_static::lazy_static;
use pgrx::prelude::*;
use tzf_rs::DefaultFinder;

::pgrx::pg_module_magic!();

lazy_static! {
    static ref FINDER: DefaultFinder = DefaultFinder::default();
}

/// Returns the IANA timezone name for the given longitude and latitude coordinates.
///
/// # Arguments
/// * `lon` - Longitude in degrees (-180 to +180)
/// * `lat` - Latitude in degrees (-90 to +90)
#[pg_extern]
fn tzf_tzname(lon: f64, lat: f64) -> String {
    FINDER.get_tz_name(lon, lat).to_string()
}

#[cfg(any(test, feature = "default"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_tzf_tzname() {
        assert_eq!("America/New_York", crate::tzf_tzname(74.006, 40.7128));
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
