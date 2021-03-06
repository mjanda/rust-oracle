// Rust-oracle - Rust binding for Oracle database
//
// URL: https://github.com/kubo/rust-oracle
//
// ------------------------------------------------------
//
// Copyright 2017 Kubo Takehiro <kubo@jiubao.org>
//
// Redistribution and use in source and binary forms, with or without modification, are
// permitted provided that the following conditions are met:
//
//    1. Redistributions of source code must retain the above copyright notice, this list of
//       conditions and the following disclaimer.
//
//    2. Redistributions in binary form must reproduce the above copyright notice, this list
//       of conditions and the following disclaimer in the documentation and/or other materials
//       provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHORS ''AS IS'' AND ANY EXPRESS OR IMPLIED
// WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
// FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL <COPYRIGHT HOLDER> OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
// ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
// ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// The views and conclusions contained in the software and documentation are those of the
// authors and should not be interpreted as representing official policies, either expressed
// or implied, of the authors.

extern crate oracle;
#[macro_use]
mod common;
use oracle::*;

//
// Timestamp
//

#[test]
fn timestamp_from_sql() {
    let conn = common::connect().unwrap();
    let ts = Timestamp::new(2012, 3, 4, 0, 0, 0, 0);

    test_from_sql!(&conn,
                   "DATE '2012-03-04'",
                   &OracleType::Date, &ts);
    let ts = Timestamp::new(2012, 3, 4, 5, 6, 7, 0);
    test_from_sql!(&conn,
                   "TO_DATE('2012-03-04 05:06:07', 'YYYY-MM-DD HH24:MI:SS')",
                   &OracleType::Date, &ts);

    test_from_sql!(&conn,
                   "CAST(TO_DATE('2012-03-04 05:06:07', 'YYYY-MM-DD HH24:MI:SS') AS TIMESTAMP(0))",
                   &OracleType::Timestamp(0), &ts);
    let ts = ts.and_prec(1);
    test_from_sql!(&conn,
                   "CAST(TO_DATE('2012-03-04 05:06:07', 'YYYY-MM-DD HH24:MI:SS') AS TIMESTAMP(1))",
                   &OracleType::Timestamp(1), &ts);
    let ts = ts.and_prec(6);
    test_from_sql!(&conn,
                   "CAST(TO_DATE('2012-03-04 05:06:07', 'YYYY-MM-DD HH24:MI:SS') AS TIMESTAMP)",
                   &OracleType::Timestamp(6), &ts);
    let ts = ts.and_prec(9);
    test_from_sql!(&conn,
                   "CAST(TO_DATE('2012-03-04 05:06:07', 'YYYY-MM-DD HH24:MI:SS') AS TIMESTAMP(9))",
                   &OracleType::Timestamp(9), &ts);
    test_from_sql!(&conn,
                   "TO_TIMESTAMP('2012-03-04 05:06:07', 'YYYY-MM-DD HH24:MI:SS')",
                   &OracleType::Timestamp(9), &ts);
    let ts = Timestamp::new(2012, 3, 4, 5, 6, 7, 123456789);
    test_from_sql!(&conn,
                   "TO_TIMESTAMP('2012-03-04 05:06:07.123456789', 'YYYY-MM-DD HH24:MI:SS.FF')",
                   &OracleType::Timestamp(9), &ts);
    let ts = Timestamp::new(2012, 3, 4, 5, 6, 7, 123456000);
    test_from_sql!(&conn,
                   "TO_TIMESTAMP('2012-03-04 05:06:07.123456', 'YYYY-MM-DD HH24:MI:SS.FF')",
                   &OracleType::Timestamp(9), &ts);
    let ts = Timestamp::new(2012, 3, 4, 5, 6, 7, 123000000);
    test_from_sql!(&conn,
                   "TO_TIMESTAMP('2012-03-04 05:06:07.123', 'YYYY-MM-DD HH24:MI:SS.FF')",
                   &OracleType::Timestamp(9), &ts);

    let ts = Timestamp::new(2012, 3, 4, 5, 6, 7, 0).and_tz_offset(0);
    test_from_sql!(&conn,
                   "TO_TIMESTAMP_TZ('2012-03-04 05:06:07 +00:00', 'YYYY-MM-DD HH24:MI:SS TZH:TZM')",
                   &OracleType::TimestampTZ(9), &ts);
    let ts = Timestamp::new(2012, 3, 4, 5, 6, 7, 0).and_tz_hm_offset(8, 45);
    test_from_sql!(&conn,
                   "TO_TIMESTAMP_TZ('2012-03-04 05:06:07 +08:45', 'YYYY-MM-DD HH24:MI:SS TZH:TZM')",
                   &OracleType::TimestampTZ(9), &ts);
    let ts = Timestamp::new(2012, 3, 4, 5, 6, 7, 0).and_tz_hm_offset(-8, -45);
    test_from_sql!(&conn,
                   "TO_TIMESTAMP_TZ('2012-03-04 05:06:07 -08:45', 'YYYY-MM-DD HH24:MI:SS TZH:TZM')",
                   &OracleType::TimestampTZ(9), &ts);
}

#[test]
fn timestamp_to_sql() {
    let conn = common::connect().unwrap();
    let ts = Timestamp::new(2012, 3, 4, 0, 0, 0, 0);

    test_to_sql!(&conn, &ts,
                 "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS')",
                 "2012-03-04 00:00:00");

    let ts = Timestamp::new(2012, 3, 4, 5, 6, 7, 0);
    test_to_sql!(&conn, &ts,
                 "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS')",
                 "2012-03-04 05:06:07");

    test_to_sql!(&conn, &ts,
                 "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS')",
                 "2012-03-04 05:06:07");
    let ts = ts.and_prec(1);
    test_to_sql!(&conn, &ts,
                 "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS')",
                 "2012-03-04 05:06:07");
    let ts = ts.and_prec(6);
    test_to_sql!(&conn, &ts,
                 "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS')",
                 "2012-03-04 05:06:07");
    let ts = ts.and_prec(9);
    test_to_sql!(&conn, &ts,
                 "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS')",
                 "2012-03-04 05:06:07");
    test_to_sql!(&conn, &ts,
                 "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS')",
                 "2012-03-04 05:06:07");
    let ts = Timestamp::new(2012, 3, 4, 5, 6, 7, 123456789);
    test_to_sql!(&conn, &ts,
                 "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS.FF')",
                 "2012-03-04 05:06:07.123456789");
    let ts = Timestamp::new(2012, 3, 4, 5, 6, 7, 123456000);
    test_to_sql!(&conn, &ts,
                 "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS.FF6')",
                 "2012-03-04 05:06:07.123456");
    let ts = Timestamp::new(2012, 3, 4, 5, 6, 7, 123000000);
    test_to_sql!(&conn, &ts,
                 "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS.FF3')",
                 "2012-03-04 05:06:07.123");

    let ts = Timestamp::new(2012, 3, 4, 5, 6, 7, 0).and_tz_offset(0);
    test_to_sql!(&conn, &ts,
                 "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS TZH:TZM')",
                 "2012-03-04 05:06:07 +00:00");
    let ts = Timestamp::new(2012, 3, 4, 5, 6, 7, 0).and_tz_hm_offset(8, 45);
    test_to_sql!(&conn, &ts,
                 "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS TZH:TZM')",
                 "2012-03-04 05:06:07 +08:45");
    let ts = Timestamp::new(2012, 3, 4, 5, 6, 7, 0).and_tz_hm_offset(-8, -45);
    test_to_sql!(&conn, &ts,
                 "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS TZH:TZM')",
                 "2012-03-04 05:06:07 -08:45");
}

//
// IntervalDS
//

#[test]
fn interval_ds_from_sql() {
    let conn = common::connect().unwrap();

    let it = IntervalDS::new(1, 2, 3, 4, 0);
    test_from_sql!(&conn,
                   "INTERVAL '1 02:03:04' DAY TO SECOND",
                   &OracleType::IntervalDS(2, 6), &it);

    let it = IntervalDS::new(1, 2, 3, 4, 123456789);
    test_from_sql!(&conn,
                   "INTERVAL '+1 02:03:04.123456789' DAY TO SECOND(9)",
                   &OracleType::IntervalDS(2, 9), &it);

    let it = IntervalDS::new(123456789, 2, 3, 4, 123456789);
    test_from_sql!(&conn,
                   "INTERVAL '+123456789 02:03:04.123456789' DAY(9) TO SECOND(9)",
                   &OracleType::IntervalDS(9, 9), &it);

    let it = IntervalDS::new(-1, -2, -3, -4, 0);
    test_from_sql!(&conn,
                   "INTERVAL '-1 02:03:04' DAY TO SECOND",
                   &OracleType::IntervalDS(2, 6), &it);

    let it = IntervalDS::new(-1, -2, -3, -4, -123456789);
    test_from_sql!(&conn,
                   "INTERVAL '-1 02:03:04.123456789' DAY TO SECOND(9)",
                   &OracleType::IntervalDS(2, 9), &it);

    let it = IntervalDS::new(-123456789, -2, -3, -4, -123456789);
    test_from_sql!(&conn,
                   "INTERVAL '-123456789 02:03:04.123456789' DAY(9) TO SECOND(9)",
                   &OracleType::IntervalDS(9, 9), &it);
}

#[test]
fn interval_ds_to_sql() {
    let conn = common::connect().unwrap();

    let it = IntervalDS::new(1, 2, 3, 4, 0);
    test_to_sql!(&conn, &it,
                 "TO_CHAR(:1)",
                 "+000000001 02:03:04.000000000");

    let it = IntervalDS::new(1, 2, 3, 4, 123456789);
    test_to_sql!(&conn, &it,
                 "TO_CHAR(:1)",
                 "+000000001 02:03:04.123456789");

    let it = IntervalDS::new(123456789, 2, 3, 4, 123456789);
    test_to_sql!(&conn, &it,
                 "TO_CHAR(:1)",
                 "+123456789 02:03:04.123456789");

    let it = IntervalDS::new(-1, -2, -3, -4, 0);
    test_to_sql!(&conn, &it,
                 "TO_CHAR(:1)",
                 "-000000001 02:03:04.000000000");

    let it = IntervalDS::new(-1, -2, -3, -4, -123456789);
    test_to_sql!(&conn, &it,
                 "TO_CHAR(:1)",
                 "-000000001 02:03:04.123456789");

    let it = IntervalDS::new(-123456789, -2, -3, -4, -123456789);
    test_to_sql!(&conn, &it,
                 "TO_CHAR(:1)",
                 "-123456789 02:03:04.123456789");
}

//
// IntervalYM
//

#[test]
fn interval_ym_from_sql() {
    let conn = common::connect().unwrap();

    let it = IntervalYM::new(1, 2);
    test_from_sql!(&conn,
                   "INTERVAL '1-2' YEAR TO MONTH",
                   &OracleType::IntervalYM(2), &it);

    let it = IntervalYM::new(123456789, 2);
    test_from_sql!(&conn,
                   "INTERVAL '123456789-2' YEAR(9) TO MONTH",
                   &OracleType::IntervalYM(9), &it);

    let it = IntervalYM::new(-1, -2);
    test_from_sql!(&conn,
                   "INTERVAL '-1-2' YEAR TO MONTH",
                   &OracleType::IntervalYM(2), &it);

    let it = IntervalYM::new(-123456789, -2);
    test_from_sql!(&conn,
                   "INTERVAL '-123456789-2' YEAR(9) TO MONTH",
                   &OracleType::IntervalYM(9), &it);
}

#[test]
fn interval_ym_to_sql() {
    let conn = common::connect().unwrap();

    let it = IntervalYM::new(1, 2);
    test_to_sql!(&conn, &it,
                 "TO_CHAR(:1)",
                 "+000000001-02");
    let it = IntervalYM::new(123456789, 2);
    test_to_sql!(&conn, &it,
                 "TO_CHAR(:1)",
                 "+123456789-02");

    let it = IntervalYM::new(-1, -2);
    test_to_sql!(&conn, &it,
                 "TO_CHAR(:1)",
                 "-000000001-02");

    let it = IntervalYM::new(-123456789, -2);
    test_to_sql!(&conn, &it,
                 "TO_CHAR(:1)",
                 "-123456789-02");
}

#[cfg(feature = "chrono")]
mod chrono {
    extern crate chrono;
    use self::chrono::prelude::*;
    use self::chrono::Duration;
    use self::chrono::naive::NaiveDate;
    use common;
    use oracle::*;

    //
    // chrono::DateTime<Utc>
    // chrono::DateTime<Local>
    // chrono::DateTime<FixedOffset>
    //

    #[test]
    fn datetime_from_sql() {
        let conn = common::connect().unwrap();
        let fixed_utc = FixedOffset::east(0);
        let fixed_cet = FixedOffset::east(3600);

        // DATE -> DateTime<Utc>
        let dttm = Utc.ymd(2012, 3, 4).and_hms(5, 6, 7);
        test_from_sql!(&conn,
                       "TO_DATE('2012-03-04 05:06:07', 'YYYY-MM-DD HH24:MI:SS')",
                       &OracleType::Date, &dttm);

        // DATE -> DateTime<Local>
        let dttm = Local.ymd(2012, 3, 4).and_hms(5, 6, 7);
        test_from_sql!(&conn,
                       "TO_DATE('2012-03-04 05:06:07', 'YYYY-MM-DD HH24:MI:SS')",
                       &OracleType::Date, &dttm);

        // DATE -> DateTime<FixedOffset>  TZ is '+00:00'.
        let dttm = fixed_utc.ymd(2012, 3, 4).and_hms(5, 6, 7);
        test_from_sql!(&conn,
                       "TO_DATE('2012-03-04 05:06:07', 'YYYY-MM-DD HH24:MI:SS')",
                       &OracleType::Date, &dttm);

        // TIMESTAMP -> DateTime<Utc>
        let dttm = Utc.ymd(2012, 3, 4).and_hms_nano(5, 6, 7, 123456789);
        test_from_sql!(&conn,
                       "TO_TIMESTAMP('2012-03-04 05:06:07.123456789', 'YYYY-MM-DD HH24:MI:SS.FF9')",
                       &OracleType::Timestamp(9), &dttm);

        // TIMESTAMP -> DateTime<Local>
        let dttm = Local.ymd(2012, 3, 4).and_hms_nano(5, 6, 7, 123456789);
        test_from_sql!(&conn,
                       "TO_TIMESTAMP('2012-03-04 05:06:07.123456789', 'YYYY-MM-DD HH24:MI:SS.FF9')",
                       &OracleType::Timestamp(9), &dttm);

        // TIMESTAMP -> DateTime<Fixed_Utc>  TZ is '+00:00'.
        let dttm = fixed_utc.ymd(2012, 3, 4).and_hms_nano(5, 6, 7, 123456789);
        test_from_sql!(&conn,
                       "TO_TIMESTAMP('2012-03-04 05:06:07.123456789', 'YYYY-MM-DD HH24:MI:SS.FF9')",
                       &OracleType::Timestamp(9), &dttm);

        // TIMESTAMP WITH TIME ZONE -> DateTime<Utc>  TZ is ignored.
        let dttm = Utc.ymd(2012, 3, 4).and_hms_nano(5, 6, 7, 123456789);
        test_from_sql!(&conn,
                       "TO_TIMESTAMP_TZ('2012-03-04 05:06:07.123456789 +01:00', 'YYYY-MM-DD HH24:MI:SS.FF9 TZH:TZM')",
                       &OracleType::TimestampTZ(9), &dttm);

        // TIMESTAMP WITH TIME ZONE -> DateTime<Local>  TZ is ignored.
        let dttm = Local.ymd(2012, 3, 4).and_hms_nano(5, 6, 7, 123456789);
        test_from_sql!(&conn,
                       "TO_TIMESTAMP_TZ('2012-03-04 05:06:07.123456789 +01:00', 'YYYY-MM-DD HH24:MI:SS.FF9 TZH:TZM')",
                       &OracleType::TimestampTZ(9), &dttm);

        // TIMESTAMP WITH TIME ZONE -> DateTime<Fixed_Utc> TZ is set.
        let dttm = fixed_cet.ymd(2012, 3, 4).and_hms_nano(5, 6, 7, 123456789);
        test_from_sql!(&conn,
                       "TO_TIMESTAMP_TZ('2012-03-04 05:06:07.123456789 +01:00', 'YYYY-MM-DD HH24:MI:SS.FF9 TZH:TZM')",
                       &OracleType::TimestampTZ(9), &dttm);
    }

    #[test]
    fn datetime_to_sql() {
        let conn = common::connect().unwrap();
        let dttm_utc = Utc.ymd(2012, 3, 4).and_hms_nano(5, 6, 7, 123456789);
        let dttm_local = Local.ymd(2012, 3, 4).and_hms_nano(5, 6, 7, 123456789);
        let dttm_fixed_cet = FixedOffset::east(3600).ymd(2012, 3, 4).and_hms_nano(5, 6, 7, 123456789);

        // DateTime<Utc> -> TIMESTAMP WITH TIME ZONE
        test_to_sql!(&conn, &dttm_utc,
                     "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS.FF9 TZH:TZM')",
                     "2012-03-04 05:06:07.123456789 +00:00");

        // DateTime<Local> -> TIMESTAMP WITH TIME ZONE
        let tz_offset = dttm_local.offset().fix().local_minus_utc();
        let tz_sign = if tz_offset >= 0 { '+' } else { '-' };
        let tz_hour = tz_offset.abs() / 3600;
        let tz_min = tz_offset.abs() % 3600 / 60;
        test_to_sql!(&conn, &dttm_local,
                     "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS.FF9 TZH:TZM')",
                     &format!("2012-03-04 05:06:07.123456789 {}{:02}:{:02}", tz_sign, tz_hour, tz_min));

        // DateTime<FixedOffset> -> TIMESTAMP WITH TIME ZONE
        test_to_sql!(&conn, &dttm_fixed_cet,
                     "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS.FF9 TZH:TZM')",
                     "2012-03-04 05:06:07.123456789 +01:00");
    }

    //
    // chrono::Date<Utc>
    // chrono::Date<Local>
    // chrono::Date<FixedOffset>
    //

    #[test]
    fn date_from_sql() {
        let conn = common::connect().unwrap();
        let fixed_utc = FixedOffset::east(0);
        let fixed_cet = FixedOffset::east(3600);

        // DATE -> Date<Utc>
        let dttm = Utc.ymd(2012, 3, 4);
        test_from_sql!(&conn,
                       "TO_DATE('2012-03-04 05:06:07', 'YYYY-MM-DD HH24:MI:SS')",
                       &OracleType::Date, &dttm);

        // DATE -> Date<Local>
        let dttm = Local.ymd(2012, 3, 4);
        test_from_sql!(&conn,
                       "TO_DATE('2012-03-04 05:06:07', 'YYYY-MM-DD HH24:MI:SS')",
                       &OracleType::Date, &dttm);

        // DATE -> Date<FixedOffset>  TZ is '+00:00'.
        let dttm = fixed_utc.ymd(2012, 3, 4);
        test_from_sql!(&conn,
                       "TO_DATE('2012-03-04 05:06:07', 'YYYY-MM-DD HH24:MI:SS')",
                       &OracleType::Date, &dttm);

        // TIMESTAMP -> Date<Utc>
        let dttm = Utc.ymd(2012, 3, 4);
        test_from_sql!(&conn,
                       "TO_TIMESTAMP('2012-03-04 05:06:07.123456789', 'YYYY-MM-DD HH24:MI:SS.FF9')",
                       &OracleType::Timestamp(9), &dttm);

        // TIMESTAMP -> Date<Local>
        let dttm = Local.ymd(2012, 3, 4);
        test_from_sql!(&conn,
                       "TO_TIMESTAMP('2012-03-04 05:06:07.123456789', 'YYYY-MM-DD HH24:MI:SS.FF9')",
                       &OracleType::Timestamp(9), &dttm);

        // TIMESTAMP -> Date<Fixed_Utc>  TZ is '+00:00'.
        let dttm = fixed_utc.ymd(2012, 3, 4);
        test_from_sql!(&conn,
                       "TO_TIMESTAMP('2012-03-04 05:06:07.123456789', 'YYYY-MM-DD HH24:MI:SS.FF9')",
                       &OracleType::Timestamp(9), &dttm);

        // TIMESTAMP WITH TIME ZONE -> Date<Utc>  TZ is ignored.
        let dttm = Utc.ymd(2012, 3, 4);
        test_from_sql!(&conn,
                       "TO_TIMESTAMP_TZ('2012-03-04 05:06:07.123456789 +01:00', 'YYYY-MM-DD HH24:MI:SS.FF9 TZH:TZM')",
                       &OracleType::TimestampTZ(9), &dttm);

        // TIMESTAMP WITH TIME ZONE -> Date<Local>  TZ is ignored.
        let dttm = Local.ymd(2012, 3, 4);
        test_from_sql!(&conn,
                       "TO_TIMESTAMP_TZ('2012-03-04 05:06:07.123456789 +01:00', 'YYYY-MM-DD HH24:MI:SS.FF9 TZH:TZM')",
                       &OracleType::TimestampTZ(9), &dttm);

        // TIMESTAMP WITH TIME ZONE -> Date<Fixed_Utc> TZ is set.
        let dttm = fixed_cet.ymd(2012, 3, 4);
        test_from_sql!(&conn,
                       "TO_TIMESTAMP_TZ('2012-03-04 05:06:07.123456789 +01:00', 'YYYY-MM-DD HH24:MI:SS.FF9 TZH:TZM')",
                       &OracleType::TimestampTZ(9), &dttm);
    }

    #[test]
    fn date_to_sql() {
        let conn = common::connect().unwrap();
        let dttm_utc = Utc.ymd(2012, 3, 4);
        let dttm_local = Local.ymd(2012, 3, 4);
        let dttm_fixed_cet = FixedOffset::east(3600).ymd(2012, 3, 4);

        // Date<Utc> -> TIMESTAMP WITH TIME ZONE
        test_to_sql!(&conn, &dttm_utc,
                     "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS.FF9 TZH:TZM')",
                     "2012-03-04 00:00:00.000000000 +00:00");

        // Date<Local> -> TIMESTAMP WITH TIME ZONE
        let tz_offset = dttm_local.offset().fix().local_minus_utc();
        let tz_sign = if tz_offset >= 0 { '+' } else { '-' };
        let tz_hour = tz_offset.abs() / 3600;
        let tz_min = tz_offset.abs() % 3600 / 60;
        test_to_sql!(&conn, &dttm_local,
                     "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS.FF9 TZH:TZM')",
                     &format!("2012-03-04 00:00:00.000000000 {}{:02}:{:02}", tz_sign, tz_hour, tz_min));

        // Date<FixedOffset> -> TIMESTAMP WITH TIME ZONE
        test_to_sql!(&conn, &dttm_fixed_cet,
                     "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS.FF9 TZH:TZM')",
                     "2012-03-04 00:00:00.000000000 +01:00");
    }

    //
    // chrono::naive::NaiveDateTime
    //

    #[test]
    fn naive_datetime_from_sql() {
        let conn = common::connect().unwrap();

        // DATE -> NaiveDateTime
        let dttm = NaiveDate::from_ymd(2012, 3, 4).and_hms(5, 6, 7);
        test_from_sql!(&conn,
                       "TO_DATE('2012-03-04 05:06:07', 'YYYY-MM-DD HH24:MI:SS')",
                       &OracleType::Date, &dttm);

        // TIMESTAMP -> NaiveDateTime
        let dttm = NaiveDate::from_ymd(2012, 3, 4).and_hms_nano(5, 6, 7, 123456789);
        test_from_sql!(&conn,
                       "TO_TIMESTAMP('2012-03-04 05:06:07.123456789', 'YYYY-MM-DD HH24:MI:SS.FF9')",
                       &OracleType::Timestamp(9), &dttm);

        // TIMESTAMP WITH TIME ZONE -> NaiveDateTime (TZ is ignored.)
        let dttm = NaiveDate::from_ymd(2012, 3, 4).and_hms_nano(5, 6, 7, 123456789);
        test_from_sql!(&conn,
                       "TO_TIMESTAMP_TZ('2012-03-04 05:06:07.123456789 +01:00', 'YYYY-MM-DD HH24:MI:SS.FF9 TZH:TZM')",
                       &OracleType::TimestampTZ(9), &dttm);
    }

    #[test]
    fn naive_datetime_to_sql() {
        let conn = common::connect().unwrap();

        // NaiveDateTime -> TIMESTAMP
        let dttm = NaiveDate::from_ymd(2012, 3, 4).and_hms_nano(5, 6, 7, 123456789);
        test_to_sql!(&conn, &dttm,
                     "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS.FF9')",
                     "2012-03-04 05:06:07.123456789");
    }

    //
    // chrono::NaiveDate
    //

    #[test]
    fn naive_date_from_sql() {
        let conn = common::connect().unwrap();

        // DATE -> NaiveDate
        let dttm = NaiveDate::from_ymd(2012, 3, 4);
        test_from_sql!(&conn,
                       "TO_DATE('2012-03-04 05:06:07', 'YYYY-MM-DD HH24:MI:SS')",
                       &OracleType::Date, &dttm);

        // TIMESTAMP -> NaiveDate
        let dttm = NaiveDate::from_ymd(2012, 3, 4);
        test_from_sql!(&conn,
                       "TO_TIMESTAMP('2012-03-04 05:06:07.123456789', 'YYYY-MM-DD HH24:MI:SS.FF9')",
                       &OracleType::Timestamp(9), &dttm);

        // TIMESTAMP WITH TIME ZONE -> NaiveDate (TZ is ignored.)
        let dttm = NaiveDate::from_ymd(2012, 3, 4);
        test_from_sql!(&conn,
                       "TO_TIMESTAMP_TZ('2012-03-04 05:06:07.123456789 +01:00', 'YYYY-MM-DD HH24:MI:SS.FF9 TZH:TZM')",
                       &OracleType::TimestampTZ(9), &dttm);
    }

    #[test]
    fn naive_date_to_sql() {
        let conn = common::connect().unwrap();

        // NaiveDate -> TIMESTAMP
        let dttm = NaiveDate::from_ymd(2012, 3, 4);
        test_to_sql!(&conn, &dttm,
                     "TO_CHAR(:1, 'YYYY-MM-DD HH24:MI:SS.FF9')",
                     "2012-03-04 00:00:00.000000000");
    }

    //
    // chrono::Duration
    //

    #[test]
    fn duration_from_sql() {
        let conn = common::connect().unwrap();

        // INTERVAL DAY TO SECOND -> Duration
        let d = Duration::days(1) + Duration::hours(2) + Duration::minutes(3)
            + Duration::seconds(4) + Duration::nanoseconds(123456789);
        test_from_sql!(&conn,
                       "INTERVAL '+1 02:03:04.123456789' DAY TO SECOND(9)",
                       &OracleType::IntervalDS(2, 9), &d);
        let d = -d;
        test_from_sql!(&conn,
                       "INTERVAL '-1 02:03:04.123456789' DAY TO SECOND(9)",
                       &OracleType::IntervalDS(2, 9), &d);

        let d = Duration::days(999999999) + Duration::hours(23) + Duration::minutes(59)
            + Duration::seconds(59) + Duration::nanoseconds(999999999);
        test_from_sql!(&conn,
                       "INTERVAL '+999999999 23:59:59.999999999' DAY(9) TO SECOND(9)",
                       &OracleType::IntervalDS(9, 9), &d);

        let d = -d;
        test_from_sql!(&conn,
                       "INTERVAL '-999999999 23:59:59.999999999' DAY(9) TO SECOND(9)",
                       &OracleType::IntervalDS(9, 9), &d);
    }

    #[test]
    fn duration_to_sql() {
        let conn = common::connect().unwrap();

        // Duration -> INTERVAL DAY TO SECOND
        let d = Duration::days(1) + Duration::hours(2) + Duration::minutes(3)
            + Duration::seconds(4) + Duration::nanoseconds(123456789);
        test_to_sql!(&conn, &d,
                     "TO_CHAR(:1)",
                     "+000000001 02:03:04.123456789");

        let d = -d;
        test_to_sql!(&conn, &d,
                     "TO_CHAR(:1)",
                     "-000000001 02:03:04.123456789");

        let d = Duration::days(999999999) + Duration::hours(23) + Duration::minutes(59)
            + Duration::seconds(59) + Duration::nanoseconds(999999999);
        test_to_sql!(&conn, &d,
                     "TO_CHAR(:1)",
                     "+999999999 23:59:59.999999999");

        let d = -d;
        test_to_sql!(&conn, &d,
                     "TO_CHAR(:1)",
                     "-999999999 23:59:59.999999999");

        // Overflow
        let d = Duration::days(1000000000);
        let mut stmt = conn.prepare("begin :out := TO_CHAR(:1); end;").unwrap();
        let bind_result = stmt.bind(2, &d);
        if let Err(Error::Overflow(_, _)) = bind_result {
            ; /* OK */
        } else {
            panic!("Duration 1000000000 days should not be converted to interval day to second!");
        }
    }
}
