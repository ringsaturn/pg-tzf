SELECT unnest(
    tzf_tzname_batch_points(
    ARRAY[
            point(-74.0060, 40.7128),
            point(-118.2437, 34.0522),
            point(139.6917, 35.6895)
    ]
    )
) AS timezones;