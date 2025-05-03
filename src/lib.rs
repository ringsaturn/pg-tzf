use lazy_static::lazy_static;
use pgrx::pg_sys::Point;
use pgrx::prelude::*;
use tzf_rs::DefaultFinder;

::pgrx::pg_module_magic!();

lazy_static! {
    static ref FINDER: DefaultFinder = DefaultFinder::default();
}

#[pg_extern(immutable, parallel_safe)]
fn tzf_tzname(lon: f64, lat: f64) -> String {
    FINDER.get_tz_name(lon, lat).to_string()
}

#[pg_extern(immutable, parallel_safe)]
fn tzf_tzname_point(point: Point) -> String {
    let lon = point.x;
    let lat = point.y;
    FINDER.get_tz_name(lon, lat).to_string()
}

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

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_tzf_tzname() {
        let result = Spi::get_one::<String>("SELECT tzf_tzname(-74.0060, 40.7128)")
            .unwrap()
            .unwrap();
        assert_eq!(result, "America/New_York");
    }

    #[pg_test]
    fn test_tzf_tzname_point() {
        let result = Spi::get_one::<String>("SELECT tzf_tzname_point(point(-74.0060, 40.7128))")
            .unwrap()
            .unwrap();
        assert_eq!(result, "America/New_York");
    }
}
