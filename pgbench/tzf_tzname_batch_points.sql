\set size 1000
\set lon random(-180, 180)
\set lat random(-90, 90)

SELECT unnest(
    tzf_tzname_batch_points(
        array_agg(point(:lon, :lat))
    )
)
FROM generate_series(1, :size); 