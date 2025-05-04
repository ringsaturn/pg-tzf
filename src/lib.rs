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

#[pg_extern(immutable, parallel_safe)]
fn tzf_tzname_batch(lons: Vec<f64>, lats: Vec<f64>) -> Vec<String> {
    if lons.len() != lats.len() {
        error!("array lengths of lons and lats must match");
    }

    lons.into_iter()
        .zip(lats.into_iter())
        .map(|(lon, lat)| FINDER.get_tz_name(lon, lat).to_string())
        .collect()
}

#[pg_extern(immutable, parallel_safe)]
fn tzf_tzname_batch_points(points: Vec<Point>) -> Vec<String> {
    points
        .into_iter()
        .map(|point| FINDER.get_tz_name(point.x, point.y).to_string())
        .collect()
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

    #[pg_test]
    fn test_tzf_tzname_batch() {
        let result = Spi::get_one::<Vec<String>>(
            "SELECT tzf_tzname_batch(
                ARRAY[-74.0060, -118.2437, 139.6917],
                ARRAY[40.7128, 34.0522, 35.6895]
            )",
        )
        .unwrap()
        .unwrap();

        assert_eq!(
            result,
            vec![
                "America/New_York".to_string(),
                "America/Los_Angeles".to_string(),
                "Asia/Tokyo".to_string()
            ]
        );
    }

    #[pg_test]
    fn test_tzf_tzname_batch_empty() {
        let result = Spi::get_one::<Vec<String>>(
            "SELECT tzf_tzname_batch(ARRAY[]::float8[], ARRAY[]::float8[])",
        )
        .unwrap()
        .unwrap();
        assert_eq!(result, Vec::<String>::new());
    }

    #[pg_test]
    #[should_panic]
    fn test_tzf_tzname_batch_length_mismatch() {
        Spi::get_one::<i64>(
            "
            SELECT COUNT(*) FROM tzf_tzname_batch(ARRAY[1.0], ARRAY[1.0, 2.0])
        ",
        )
        .unwrap()
        .unwrap();
    }

    #[pg_test]
    fn test_tzf_tzname_batch_points() {
        let result = Spi::get_one::<Vec<String>>(
            "SELECT tzf_tzname_batch_points(ARRAY[
                point(-74.0060, 40.7128),
                point(-118.2437, 34.0522),
                point(139.6917, 35.6895)
            ])",
        )
        .unwrap()
        .unwrap();

        assert_eq!(
            result,
            vec![
                "America/New_York".to_string(),
                "America/Los_Angeles".to_string(),
                "Asia/Tokyo".to_string()
            ]
        );
    }

    #[pg_test]
    fn test_tzf_tzname_batch_points_empty() {
        let result =
            Spi::get_one::<Vec<String>>("SELECT tzf_tzname_batch_points(ARRAY[]::point[])")
                .unwrap()
                .unwrap();
        assert_eq!(result, Vec::<String>::new());
    }
}
