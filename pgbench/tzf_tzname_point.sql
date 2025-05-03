\set lon random(-180, 180)
\set lat random(-90, 90)
SELECT tzf_tzname_point(point(:lon, :lat)); 