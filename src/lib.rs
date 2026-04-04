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
    #[cfg(feature = "pg_test")]
    use cities_json::CITIES;
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

    #[pg_test]
    fn test_tzf_tzname_with_cities_json_loaded_into_pg() {
        fn sql_literal(value: &str) -> String {
            format!("'{}'", value.replace('\'', "''"))
        }

        Spi::run(
            "
            DROP TABLE IF EXISTS test_cities_json;
            CREATE TEMP TABLE test_cities_json (
                name text NOT NULL,
                country text NOT NULL,
                lat float8 NOT NULL,
                lon float8 NOT NULL
            );
        ",
        )
        .unwrap();

        for chunk in CITIES.chunks(1000) {
            let mut sql =
                String::from("INSERT INTO test_cities_json(name, country, lat, lon) VALUES ");
            for (idx, city) in chunk.iter().enumerate() {
                if idx > 0 {
                    sql.push(',');
                }
                sql.push_str(&format!(
                    "({}, {}, {}, {})",
                    sql_literal(&city.name),
                    sql_literal(&city.country),
                    city.lat,
                    city.lng
                ));
            }
            Spi::run(&sql).unwrap();
        }

        let imported_count = Spi::get_one::<i64>("SELECT COUNT(*) FROM test_cities_json")
            .unwrap()
            .unwrap();
        assert_eq!(imported_count as usize, CITIES.len());

        let top5 = Spi::connect(|client| {
            let mut rows = vec![];
            let result = client
                .select(
                    "
                    SELECT timezone, city_count
                    FROM (
                        SELECT
                            tzf_tzname(lon, lat) AS timezone,
                            COUNT(*)::bigint AS city_count
                        FROM test_cities_json
                        GROUP BY 1
                    ) counts
                    ORDER BY city_count DESC, timezone ASC
                    LIMIT 5
                    ",
                    None,
                    &[],
                )
                .unwrap();

            for row in result {
                let timezone = row.get_by_name::<String, _>("timezone").unwrap().unwrap();
                let city_count = row.get_by_name::<i64, _>("city_count").unwrap().unwrap();
                rows.push((timezone, city_count));
            }
            rows
        });

        assert_eq!(top5.len(), 5);
        for pair in top5.windows(2) {
            assert!(pair[0].1 >= pair[1].1);
            if pair[0].1 == pair[1].1 {
                assert!(pair[0].0 <= pair[1].0);
            }
        }

        for (timezone, city_count) in &top5 {
            notice!("{} {}", timezone, city_count);
        }
    }
}
