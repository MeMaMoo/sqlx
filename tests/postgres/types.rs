extern crate time_ as time;

use std::ops::Bound;

use sqlx_oldapi::postgres::types::{Oid, PgInterval, PgMoney, PgRange};
use sqlx_oldapi::postgres::Postgres;
use sqlx_test::{test_decode_type, test_prepared_type, test_type};
use std::str::FromStr;

test_type!(null<Option<i16>>(Postgres,
    "NULL::int2" == None::<i16>
));

test_type!(null_vec<Vec<Option<i16>>>(Postgres,
    "array[10,NULL,50]::int2[]" == vec![Some(10_i16), None, Some(50)],
));

test_type!(null_array<[Option<i16>; 3]>(Postgres,
    "array[10,NULL,50]::int2[]" == vec![Some(10_i16), None, Some(50)],
));

test_type!(bool<bool>(Postgres,
    "false::boolean" == false,
    "true::boolean" == true
));

test_type!(bool_vec<Vec<bool>>(Postgres,
    "array[true,false,true]::bool[]" == vec![true, false, true],
));

test_type!(bool_array<[bool; 3]>(Postgres,
    "array[true,false,true]::bool[]" == vec![true, false, true],
));

test_type!(byte_vec<Vec<u8>>(Postgres,
    "E'\\\\xDEADBEEF'::bytea"
        == vec![0xDE_u8, 0xAD, 0xBE, 0xEF],
    "E'\\\\x'::bytea"
        == Vec::<u8>::new(),
    "E'\\\\x0000000052'::bytea"
        == vec![0_u8, 0, 0, 0, 0x52]
));

// BYTEA cannot be decoded by-reference from a simple query as postgres sends it as hex
test_prepared_type!(byte_slice<&[u8]>(Postgres,
    "E'\\\\xDEADBEEF'::bytea"
        == &[0xDE_u8, 0xAD, 0xBE, 0xEF][..],
    "E'\\\\x0000000052'::bytea"
        == &[0_u8, 0, 0, 0, 0x52][..]
));

test_type!(byte_array_empty<[u8; 0]>(Postgres,
    "E'\\\\x'::bytea" == [0_u8; 0],
));

test_type!(byte_array<[u8; 4]>(Postgres,
    "E'\\\\xDEADBEEF'::bytea" == [0xDE_u8, 0xAD, 0xBE, 0xEF],
));

test_type!(str<&str>(Postgres,
    "'this is foo'" == "this is foo",
    "''" == "",
    "'identifier'::name" == "identifier",
    "'five'::char(4)" == "five",
    "'more text'::varchar" == "more text",
));

test_type!(string<String>(Postgres,
    "'this is foo'" == format!("this is foo"),
));

test_type!(string_vec<Vec<String>>(Postgres,
    "array['one','two','three']::text[]"
        == vec!["one","two","three"],

    "array['', '\"']::text[]"
        == vec!["", "\""],

    "array['Hello, World', '', 'Goodbye']::text[]"
        == vec!["Hello, World", "", "Goodbye"]
));

test_type!(string_array<[String; 3]>(Postgres,
    "array['one','two','three']::text[]" == ["one","two","three"],
));

test_type!(i8(
    Postgres,
    "0::\"char\"" == 0_i8,
    "120::\"char\"" == 120_i8,
));

test_type!(Oid(Postgres, "325235::oid" == Oid(325235),));

test_type!(i16(
    Postgres,
    "-2144::smallint" == -2144_i16,
    "821::smallint" == 821_i16,
));

test_type!(i32(
    Postgres,
    "94101::int" == 94101_i32,
    "-5101::int" == -5101_i32
));

test_type!(i32_vec<Vec<i32>>(Postgres,
    "'{5,10,50,100}'::int[]" == vec![5_i32, 10, 50, 100],
    "'{1050}'::int[]" == vec![1050_i32],
    "'{}'::int[]" == Vec::<i32>::new(),
    "'{1,3,-5}'::int[]" == vec![1_i32, 3, -5]
));

test_type!(i32_array_empty<[i32; 0]>(Postgres,
    "'{}'::int[]" == [0_i32; 0],
));

test_type!(i32_array<[i32; 4]>(Postgres,
    "'{5,10,50,100}'::int[]" == [5_i32, 10, 50, 100],
));

test_type!(i64(Postgres, "9358295312::bigint" == 9358295312_i64));

test_type!(f32(Postgres, "9419.122::real" == 9419.122_f32));

test_type!(f64(
    Postgres,
    "939399419.1225182::double precision" == 939399419.1225182_f64
));

test_type!(f64_vec<Vec<f64>>(Postgres,
    "'{939399419.1225182,-12.0}'::float8[]" == vec![939399419.1225182_f64, -12.0]
));

test_decode_type!(bool_tuple<(bool,)>(Postgres, "row(true)" == (true,)));

test_decode_type!(num_tuple<(i32, i64, f64,)>(Postgres, "row(10,515::int8,3.124::float8)" == (10,515,3.124)));

test_decode_type!(empty_tuple<()>(Postgres, "row()" == ()));

test_decode_type!(string_tuple<(String, String, String)>(Postgres,
    "row('one','two','three')"
        == ("one".to_string(), "two".to_string(), "three".to_string()),

    "row('', '\"', '\"\"\"\"\"\"')"
        == ("".to_string(), "\"".to_string(), "\"\"\"\"\"\"".to_string()),

    "row('Hello, World', '', 'Goodbye')"
        == ("Hello, World".to_string(), "".to_string(), "Goodbye".to_string())
));

#[cfg(feature = "uuid")]
test_type!(uuid<sqlx_oldapi::types::Uuid>(Postgres,
    "'b731678f-636f-4135-bc6f-19440c13bd19'::uuid"
        == sqlx_oldapi::types::Uuid::parse_str("b731678f-636f-4135-bc6f-19440c13bd19").unwrap(),
    "'00000000-0000-0000-0000-000000000000'::uuid"
        == sqlx_oldapi::types::Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()
));

#[cfg(feature = "uuid")]
test_type!(uuid_vec<Vec<sqlx_oldapi::types::Uuid>>(Postgres,
    "'{b731678f-636f-4135-bc6f-19440c13bd19,00000000-0000-0000-0000-000000000000}'::uuid[]"
        == vec![
           sqlx_oldapi::types::Uuid::parse_str("b731678f-636f-4135-bc6f-19440c13bd19").unwrap(),
           sqlx_oldapi::types::Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()
        ]
));

#[cfg(feature = "ipnetwork")]
test_type!(ipnetwork<sqlx_oldapi::types::ipnetwork::IpNetwork>(Postgres,
    "'127.0.0.1'::inet"
        == "127.0.0.1"
            .parse::<sqlx_oldapi::types::ipnetwork::IpNetwork>()
            .unwrap(),
    "'8.8.8.8/24'::inet"
        == "8.8.8.8/24"
            .parse::<sqlx_oldapi::types::ipnetwork::IpNetwork>()
            .unwrap(),
    "'::ffff:1.2.3.0'::inet"
        == "::ffff:1.2.3.0"
            .parse::<sqlx_oldapi::types::ipnetwork::IpNetwork>()
            .unwrap(),
    "'2001:4f8:3:ba::/64'::inet"
        == "2001:4f8:3:ba::/64"
            .parse::<sqlx_oldapi::types::ipnetwork::IpNetwork>()
            .unwrap(),
    "'192.168'::cidr"
        == "192.168.0.0/24"
            .parse::<sqlx_oldapi::types::ipnetwork::IpNetwork>()
            .unwrap(),
    "'::ffff:1.2.3.0/120'::cidr"
        == "::ffff:1.2.3.0/120"
            .parse::<sqlx_oldapi::types::ipnetwork::IpNetwork>()
            .unwrap(),
));

#[cfg(feature = "mac_address")]
test_type!(mac_address<sqlx_oldapi::types::mac_address::MacAddress>(Postgres,
    "'00:01:02:03:04:05'::macaddr"
        == "00:01:02:03:04:05"
            .parse::<sqlx_oldapi::types::mac_address::MacAddress>()
            .unwrap()
));

#[cfg(feature = "bit-vec")]
test_type!(bitvec<sqlx_oldapi::types::BitVec>(
    Postgres,
    // A full byte VARBIT
    "B'01101001'" == sqlx_oldapi::types::BitVec::from_bytes(&[0b0110_1001]),
    // A VARBIT value missing five bits from a byte
    "B'110'" == {
        let mut bit_vec = sqlx_oldapi::types::BitVec::with_capacity(4);
        bit_vec.push(true);
        bit_vec.push(true);
        bit_vec.push(false);
        bit_vec
    },
    // A BIT value
    "B'01101'::bit(5)" == {
        let mut bit_vec = sqlx_oldapi::types::BitVec::with_capacity(5);
        bit_vec.push(false);
        bit_vec.push(true);
        bit_vec.push(true);
        bit_vec.push(false);
        bit_vec.push(true);
        bit_vec
    },
));

#[cfg(feature = "ipnetwork")]
test_type!(ipnetwork_vec<Vec<sqlx_oldapi::types::ipnetwork::IpNetwork>>(Postgres,
    "'{127.0.0.1,8.8.8.8/24}'::inet[]"
        == vec![
           "127.0.0.1".parse::<sqlx_oldapi::types::ipnetwork::IpNetwork>().unwrap(),
           "8.8.8.8/24".parse::<sqlx_oldapi::types::ipnetwork::IpNetwork>().unwrap()
        ]
));

#[cfg(feature = "mac_address")]
test_type!(mac_address_vec<Vec<sqlx_oldapi::types::mac_address::MacAddress>>(Postgres,
    "'{01:02:03:04:05:06,FF:FF:FF:FF:FF:FF}'::macaddr[]"
        == vec![
           "01:02:03:04:05:06".parse::<sqlx_oldapi::types::mac_address::MacAddress>().unwrap(),
           "FF:FF:FF:FF:FF:FF".parse::<sqlx_oldapi::types::mac_address::MacAddress>().unwrap()
        ]
));

#[cfg(feature = "chrono")]
mod chrono {
    use super::*;
    use sqlx_oldapi::types::chrono::{
        DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc,
    };

    type PgTimeTz = sqlx_oldapi::postgres::types::PgTimeTz<NaiveTime, FixedOffset>;

    test_type!(chrono_date<NaiveDate>(Postgres,
        "DATE '2001-01-05'" == NaiveDate::from_ymd_opt(2001, 1, 5).unwrap(),
        "DATE '2050-11-23'" == NaiveDate::from_ymd_opt(2050, 11, 23).unwrap()
    ));

    test_type!(chrono_time<NaiveTime>(Postgres,
        "TIME '05:10:20.115100'" == NaiveTime::from_hms_micro_opt(5, 10, 20, 115100).unwrap()
    ));

    test_type!(chrono_date_time<NaiveDateTime>(Postgres,
        "'2019-01-02 05:10:20'::timestamp" == NaiveDate::from_ymd_opt(2019, 1, 2).unwrap().and_hms_opt(5, 10, 20).unwrap()
    ));

    test_type!(chrono_date_time_vec<Vec<NaiveDateTime>>(Postgres,
        "array['2019-01-02 05:10:20']::timestamp[]"
            == vec![NaiveDate::from_ymd_opt(2019, 1, 2).unwrap().and_hms_opt(5, 10, 20).unwrap()]
    ));

    test_type!(chrono_date_time_tz_utc<DateTime::<Utc>>(Postgres,
        "TIMESTAMPTZ '2019-01-02 05:10:20.115100'"
            == DateTime::<Utc>::from_utc(
                NaiveDate::from_ymd_opt(2019, 1, 2).unwrap().and_hms_micro_opt(5, 10, 20, 115100).unwrap(),
                Utc,
            )
    ));

    test_type!(chrono_date_time_tz<DateTime::<FixedOffset>>(Postgres,
        "TIMESTAMPTZ '2019-01-02 05:10:20.115100+06:30'"
            == FixedOffset::east_opt(60 * 60 * 6 + 1800).unwrap().ymd(2019, 1, 2).and_hms_micro_opt(5, 10, 20, 115100).unwrap()
    ));

    test_type!(chrono_date_time_tz_vec<Vec<DateTime::<Utc>>>(Postgres,
        "array['2019-01-02 05:10:20.115100']::timestamptz[]"
            == vec![
                DateTime::<Utc>::from_utc(
                    NaiveDate::from_ymd_opt(2019, 1, 2).unwrap().and_hms_micro_opt(5, 10, 20, 115100).unwrap(),
                    Utc,
                )
            ]
    ));

    test_type!(chrono_time_tz<PgTimeTz>(Postgres,
        "TIMETZ '05:10:20.115100+00'" == PgTimeTz { time: NaiveTime::from_hms_micro_opt(5, 10, 20, 115100).unwrap(), offset: FixedOffset::east_opt(0).unwrap() },
        "TIMETZ '05:10:20.115100+06:30'" == PgTimeTz { time: NaiveTime::from_hms_micro_opt(5, 10, 20, 115100).unwrap(), offset: FixedOffset::east_opt(60 * 60 * 6 + 1800).unwrap() },
        "TIMETZ '05:10:20.115100-05'" == PgTimeTz { time: NaiveTime::from_hms_micro_opt(5, 10, 20, 115100).unwrap(), offset: FixedOffset::west_opt(60 * 60 * 5).unwrap() },
        "TIMETZ '05:10:20+02'" == PgTimeTz { time: NaiveTime::from_hms(5, 10, 20), offset: FixedOffset::east_opt(60 * 60 * 2 ).unwrap()}
    ));
}

#[cfg(feature = "time")]
mod time_tests {
    use super::*;
    use sqlx_oldapi::types::time::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};
    use time::macros::{date, time};

    type PgTimeTz = sqlx_oldapi::postgres::types::PgTimeTz<Time, UtcOffset>;

    test_type!(time_date<Date>(
        Postgres,
        "DATE '2001-01-05'" == date!(2001 - 1 - 5),
        "DATE '2050-11-23'" == date!(2050 - 11 - 23)
    ));

    test_type!(time_time<Time>(
        Postgres,
        "TIME '05:10:20.115100'" == time!(5:10:20.115100)
    ));

    test_type!(time_date_time<PrimitiveDateTime>(
        Postgres,
        "TIMESTAMP '2019-01-02 05:10:20'" == date!(2019 - 1 - 2).with_time(time!(5:10:20)),
        "TIMESTAMP '2019-01-02 05:10:20.1151'" == date!(2019 - 1 - 2).with_time(time!(5:10:20.115100))
    ));

    test_type!(time_timestamp<OffsetDateTime>(
        Postgres,
        "TIMESTAMPTZ '2019-01-02 05:10:20.115100'"
            == date!(2019 - 1 - 2)
                .with_time(time!(5:10:20.115100))
                .assume_utc()
    ));

    test_prepared_type!(time_time_tz<PgTimeTz>(Postgres,
        "TIMETZ '05:10:20.115100+00'" == PgTimeTz { time: time!(5:10:20.115100), offset: UtcOffset::from_whole_seconds(0).unwrap() },
        "TIMETZ '05:10:20.115100+00'" == PgTimeTz { time: time!(5:10:20.115100), offset: UtcOffset::from_whole_seconds(0).unwrap() },
        "TIMETZ '05:10:20.115100+06:30'" == PgTimeTz { time: time!(5:10:20.115100), offset: UtcOffset::from_whole_seconds(60 * 60 * 6 + 1800).unwrap() },
        "TIMETZ '05:10:20.115100-05'" == PgTimeTz { time: time!(5:10:20.115100), offset: UtcOffset::from_whole_seconds(-(60 * 60 * 5)).unwrap() },
        "TIMETZ '05:10:20+02'" == PgTimeTz { time: time!(5:10:20), offset: UtcOffset::from_whole_seconds(60 * 60 * 2 ).unwrap() }
    ));
}

#[cfg(feature = "json")]
mod json {
    use super::*;
    use serde_json::value::RawValue as JsonRawValue;
    use serde_json::{json, Value as JsonValue};
    use sqlx_oldapi::postgres::PgRow;
    use sqlx_oldapi::types::Json;
    use sqlx_oldapi::{Executor, Row};
    use sqlx_test::new;

    // When testing JSON, coerce to JSONB for `=` comparison as `JSON = JSON` is not
    // supported in PostgreSQL

    test_type!(json<JsonValue>(
        Postgres,
        "SELECT ({0}::jsonb is not distinct from $1::jsonb)::int4, {0} as _2, $2 as _3",
        "'\"Hello, World\"'::json" == json!("Hello, World"),
        "'\"😎\"'::json" == json!("😎"),
        "'\"🙋‍♀️\"'::json" == json!("🙋‍♀️"),
        "'[\"Hello\", \"World!\"]'::json" == json!(["Hello", "World!"])
    ));

    test_type!(json_vec<Vec<JsonValue>>(
        Postgres,
        "SELECT ({0}::jsonb[] is not distinct from $1::jsonb[])::int4, {0} as _2, $2 as _3",
        "array['\"😎\"'::json, '\"🙋‍♀️\"'::json]::json[]" == vec![json!("😎"), json!("🙋‍♀️")],
    ));

    test_type!(json_array<[JsonValue; 2]>(
        Postgres,
        "SELECT ({0}::jsonb[] is not distinct from $1::jsonb[])::int4, {0} as _2, $2 as _3",
        "array['\"😎\"'::json, '\"🙋‍♀️\"'::json]::json[]" == [json!("😎"), json!("🙋‍♀️")],
    ));

    test_type!(jsonb<JsonValue>(
        Postgres,
        "'\"Hello, World\"'::jsonb" == json!("Hello, World"),
        "'\"😎\"'::jsonb" == json!("😎"),
        "'\"🙋‍♀️\"'::jsonb" == json!("🙋‍♀️"),
        "'[\"Hello\", \"World!\"]'::jsonb" == json!(["Hello", "World!"])
    ));

    test_type!(jsonb_array<Vec<JsonValue>>(
        Postgres,
        "array['\"😎\"'::jsonb, '\"🙋‍♀️\"'::jsonb]::jsonb[]" == vec![json!("😎"), json!("🙋‍♀️")],
    ));

    #[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
    struct Friend {
        name: String,
        age: u32,
    }

    test_type!(json_struct<Json<Friend>>(Postgres,
        "'{\"name\":\"Joe\",\"age\":33}'::jsonb" == Json(Friend { name: "Joe".to_string(), age: 33 })
    ));

    test_type!(json_struct_vec<Vec<Json<Friend>>>(Postgres,
        "array['{\"name\":\"Joe\",\"age\":33}','{\"name\":\"Bob\",\"age\":22}']::jsonb[]"
            == vec![
                Json(Friend { name: "Joe".to_string(), age: 33 }),
                Json(Friend { name: "Bob".to_string(), age: 22 }),
            ]
    ));

    #[sqlx_macros::test]
    async fn test_json_raw_value() -> anyhow::Result<()> {
        let mut conn = new::<Postgres>().await?;

        // unprepared, text API
        let row: PgRow = conn
            .fetch_one("SELECT '{\"hello\": \"world\"}'::jsonb")
            .await?;

        let value: &JsonRawValue = row.try_get(0)?;

        assert_eq!(value.get(), "{\"hello\": \"world\"}");

        // prepared, binary API
        let row: PgRow = conn
            .fetch_one(sqlx_oldapi::query("SELECT '{\"hello\": \"world\"}'::jsonb"))
            .await?;

        let value: &JsonRawValue = row.try_get(0)?;

        assert_eq!(value.get(), "{\"hello\": \"world\"}");

        Ok(())
    }
}

#[cfg(feature = "bigdecimal")]
test_type!(bigdecimal<sqlx_oldapi::types::BigDecimal>(Postgres,

    // https://github.com/launchbadge/sqlx/issues/283
    "0::numeric" == "0".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),

    "1::numeric" == "1".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),
    "10000::numeric" == "10000".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),
    "0.1::numeric" == "0.1".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),
    "0.01::numeric" == "0.01".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),
    "0.012::numeric" == "0.012".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),
    "0.0123::numeric" == "0.0123".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),
    "0.01234::numeric" == "0.01234".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),
    "0.012345::numeric" == "0.012345".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),
    "0.0123456::numeric" == "0.0123456".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),
    "0.01234567::numeric" == "0.01234567".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),
    "0.012345678::numeric" == "0.012345678".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),
    "0.0123456789::numeric" == "0.0123456789".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),
    "0.002::numeric" == "0.002".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),
    "0.0002::numeric" == "0.0002".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),
    "0.00002::numeric" == "0.00002".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),
    "0.000002::numeric" == "0.000002".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),
    "0.0000002::numeric" == "0.0000002".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),
    "0.00000002::numeric" == "0.00000002".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),
    "12.34::numeric" == "12.34".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),
    "12345.6789::numeric" == "12345.6789".parse::<sqlx_oldapi::types::BigDecimal>().unwrap(),
));

#[cfg(feature = "bigdecimal")]
test_type!(numrange_bigdecimal<PgRange<sqlx_oldapi::types::BigDecimal>>(Postgres,
    "'(1.3,2.4)'::numrange" == PgRange::from(
        (Bound::Excluded("1.3".parse::<sqlx_oldapi::types::BigDecimal>().unwrap()),
         Bound::Excluded("2.4".parse::<sqlx_oldapi::types::BigDecimal>().unwrap())))
));

#[cfg(feature = "decimal")]
test_type!(decimal<sqlx_oldapi::types::Decimal>(Postgres,
    "0::numeric" == sqlx_oldapi::types::Decimal::from_str("0").unwrap(),
    "1::numeric" == sqlx_oldapi::types::Decimal::from_str("1").unwrap(),
    "10000::numeric" == sqlx_oldapi::types::Decimal::from_str("10000").unwrap(),
    "0.1::numeric" == sqlx_oldapi::types::Decimal::from_str("0.1").unwrap(),
    "0.01234::numeric" == sqlx_oldapi::types::Decimal::from_str("0.01234").unwrap(),
    "12.34::numeric" == sqlx_oldapi::types::Decimal::from_str("12.34").unwrap(),
    "12345.6789::numeric" == sqlx_oldapi::types::Decimal::from_str("12345.6789").unwrap(),
));

#[cfg(feature = "decimal")]
test_type!(numrange_decimal<PgRange<sqlx_oldapi::types::Decimal>>(Postgres,
    "'(1.3,2.4)'::numrange" == PgRange::from(
        (Bound::Excluded(sqlx_oldapi::types::Decimal::from_str("1.3").unwrap()),
         Bound::Excluded(sqlx_oldapi::types::Decimal::from_str("2.4").unwrap()))),
));

const EXC2: Bound<i32> = Bound::Excluded(2);
const EXC3: Bound<i32> = Bound::Excluded(3);
const INC1: Bound<i32> = Bound::Included(1);
const INC2: Bound<i32> = Bound::Included(2);
const UNB: Bound<i32> = Bound::Unbounded;

test_type!(int4range<PgRange<i32>>(Postgres,
    "'(,)'::int4range" == PgRange::from((UNB, UNB)),
    "'(,]'::int4range" == PgRange::from((UNB, UNB)),
    "'(,2)'::int4range" == PgRange::from((UNB, EXC2)),
    "'(,2]'::int4range" == PgRange::from((UNB, EXC3)),
    "'(1,)'::int4range" == PgRange::from((INC2, UNB)),
    "'(1,]'::int4range" == PgRange::from((INC2, UNB)),
    "'(1,2]'::int4range" == PgRange::from((INC2, EXC3)),
    "'[,)'::int4range" == PgRange::from((UNB, UNB)),
    "'[,]'::int4range" == PgRange::from((UNB, UNB)),
    "'[,2)'::int4range" == PgRange::from((UNB, EXC2)),
    "'[,2]'::int4range" == PgRange::from((UNB, EXC3)),
    "'[1,)'::int4range" == PgRange::from((INC1, UNB)),
    "'[1,]'::int4range" == PgRange::from((INC1, UNB)),
    "'[1,2)'::int4range" == PgRange::from((INC1, EXC2)),
    "'[1,2]'::int4range" == PgRange::from((INC1, EXC3)),
));

test_prepared_type!(interval<PgInterval>(
    Postgres,
    "INTERVAL '1h'"
        == PgInterval {
            months: 0,
            days: 0,
            microseconds: 3_600_000_000
        },
    "INTERVAL '-1 hours'"
        == PgInterval {
            months: 0,
            days: 0,
            microseconds: -3_600_000_000
        },
    "INTERVAL '3 months 12 days 1h 15 minutes 10 second '"
        == PgInterval {
            months: 3,
            days: 12,
            microseconds: (3_600 + 15 * 60 + 10) * 1_000_000
        },
    "INTERVAL '03:10:20.116100'"
        == PgInterval {
            months: 0,
            days: 0,
            microseconds: (3 * 3_600 + 10 * 60 + 20) * 1_000_000 + 116100
        },
));

test_prepared_type!(money<PgMoney>(Postgres, "123.45::money" == PgMoney(12345)));

test_prepared_type!(money_vec<Vec<PgMoney>>(Postgres,
    "array[123.45,420.00,666.66]::money[]" == vec![PgMoney(12345), PgMoney(42000), PgMoney(66666)],
));

// FIXME: needed to disable `ltree` tests in version that don't have a binary format for it
// but `PgLTree` should just fall back to text format
#[cfg(postgres_14)]
test_type!(ltree<sqlx_oldapi::postgres::types::PgLTree>(Postgres,
    "'Foo.Bar.Baz.Quux'::ltree" == sqlx_oldapi::postgres::types::PgLTree::from_str("Foo.Bar.Baz.Quux").unwrap(),
    "'Alpha.Beta.Delta.Gamma'::ltree" == sqlx_oldapi::postgres::types::PgLTree::from_iter(["Alpha", "Beta", "Delta", "Gamma"]).unwrap(),
));

// FIXME: needed to disable `ltree` tests in version that don't have a binary format for it
// but `PgLTree` should just fall back to text format
#[cfg(postgres_14)]
test_type!(ltree_vec<Vec<sqlx_oldapi::postgres::types::PgLTree>>(Postgres,
    "array['Foo.Bar.Baz.Quux', 'Alpha.Beta.Delta.Gamma']::ltree[]" ==
        vec![
            sqlx_oldapi::postgres::types::PgLTree::from_str("Foo.Bar.Baz.Quux").unwrap(),
            sqlx_oldapi::postgres::types::PgLTree::from_iter(["Alpha", "Beta", "Delta", "Gamma"]).unwrap()
        ]
));
