extern crate dislog_hal_sm2;

use dislog_hal::{Bytes, DisLogPoint, Point, Scalar};
use dislog_hal_sm2::{NewU833, PointInner, ScalarInner};
use hex::{FromHex, ToHex};

fn get_sim_sm2(a: u8) -> Scalar<ScalarInner> {
    let mut array = [0u8; 32];
    array[0] = a;

    Scalar(ScalarInner::from_bytes(&array).unwrap())
}

#[test]
fn test_scalar() {
    let scalar_innera = ScalarInner::from_bytes(&[
        216, 154, 179, 139, 210, 121, 2, 71, 69, 99, 158, 216, 23, 173, 63, 100, 204, 0, 91, 50,
        219, 153, 57, 249, 28, 82, 31, 197, 100, 165, 192, 8,
    ])
    .unwrap();
    let scalar_innerb = ScalarInner::from_bytes(&[
        216, 154, 179, 139, 210, 121, 2, 71, 69, 99, 158, 216, 23, 173, 63, 100, 204, 0, 91, 50,
        219, 153, 57, 249, 28, 82, 31, 197, 100, 165, 192, 8,
    ])
    .unwrap();
    let scalar_innerc = ScalarInner::from_bytes(&[
        216, 154, 179, 139, 210, 121, 2, 71, 69, 99, 158, 216, 23, 173, 63, 100, 204, 0, 91, 50,
        219, 153, 57, 249, 28, 82, 31, 197, 100, 165, 192, 8, 216, 154, 179, 139, 210, 121, 2, 71,
        69, 99, 158, 216, 23, 173, 63, 100, 204, 0, 91, 50, 219, 153, 57, 249, 28, 82, 31, 197,
        100, 165, 192, 8,
    ])
    .unwrap();

    let scalar_a = Scalar(scalar_innera);
    let scalar_b = Scalar(scalar_innerb);
    let scalar_c = Scalar(scalar_innerc);

    assert_eq!(
        scalar_c.to_bytes(),
        [
            146, 109, 196, 67, 110, 119, 111, 132, 195, 220, 71, 234, 11, 117, 88, 23, 27, 240, 58,
            174, 166, 96, 201, 216, 31, 186, 225, 73, 156, 208, 31, 137
        ]
    );

    assert_eq!(scalar_a.clone(), scalar_b.clone());
    assert_eq!(
        scalar_a.clone() + scalar_a.clone() + scalar_a.clone(),
        scalar_b.clone() * get_sim_sm2(3)
    );

    assert_eq!(
        scalar_a.clone() * get_sim_sm2(2),
        scalar_a.clone() + scalar_b.clone()
    );

    assert_eq!(
        scalar_a.clone() * &get_sim_sm2(2),
        scalar_a.clone() + &scalar_b
    );

    // assert_eq!(&scalar_a * get_sim_sm2(2), &scalar_a + scalar_b.clone());

    assert_eq!(&scalar_a * &get_sim_sm2(2), &scalar_a + &scalar_b);

    //assert_eq!(get_sim_sm2(0), scalar_a - scalar_b);

    //assert_eq!(get_sim_sm2(0), &scalar_a - scalar_b);

    //assert_eq!(get_sim_sm2(0), scalar_a - &scalar_b);

    assert_eq!(get_sim_sm2(0), &scalar_a - &scalar_b);

    let inv_a = scalar_a.inv();

    assert_eq!(inv_a * scalar_a, get_sim_sm2(1));

    println!("inv_a:{:?}\n", Scalar::<ScalarInner>::order());

    println!("inv_a:{:?}\n", Scalar::<ScalarInner>::zero());

    println!("inv_a:{:?}\n", Scalar::<ScalarInner>::one());
}

#[test]
fn test_point() {
    let point_innerone = PointInner::one();
    let point_innerzero = PointInner::zero();

    let point_innera = PointInner::from_bytes(&[
        2, 50, 196, 174, 44, 31, 25, 129, 25, 95, 153, 4, 70, 106, 57, 201, 148, 143, 227, 11, 191,
        242, 102, 11, 225, 113, 90, 69, 137, 51, 76, 116, 199,
    ])
    .unwrap();
    assert_eq!(point_innera, point_innerone);

    assert_eq!(
        PointInner::from_bytes(&[
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ])
        .unwrap(),
        point_innerzero
    );

    let point_innerb = PointInner::one();

    assert_eq!(
        point_innerone,
        PointInner::from_bytes(&point_innerone.to_bytes().0).unwrap()
    );

    let point_a = Point(point_innera);
    let point_b = Point(point_innerb);

    let point_one = Point(point_innerone);
    let point_zero = Point(point_innerzero);

    assert_eq!(
        Point::<PointInner>::one() + Point::<PointInner>::one() + Point::<PointInner>::one(),
        &point_a * &get_sim_sm2(3)
    );
    let point_last = point_b * (Scalar::<ScalarInner>::order() + (-get_sim_sm2(1)));
    println!("let: {:?}", &point_last);

    assert_eq!(point_one.clone() + point_last.clone(), point_zero.clone());

    assert_eq!(&point_one + point_last.clone(), point_zero.clone());

    assert_eq!(point_one.clone() + &point_last, point_zero.clone());

    assert_eq!(&point_one + &point_last, point_zero.clone());

    assert_eq!(&point_last - &point_last, point_zero.clone());

    //assert_eq!(&point_last - point_last, point_last - point_last);

    //assert_eq!(point_last - &point_last, point_last - point_last);

    //assert_eq!(&point_last - &point_last, point_last - point_last);

    assert_eq!(
        Point(
            PointInner::from_bytes(&[
                2, 169, 127, 124, 212, 179, 201, 147, 180, 190, 45, 170, 140, 219, 65, 226, 76,
                161, 63, 107, 217, 69, 48, 34, 68, 226, 105, 24, 241, 208, 80, 158, 191
            ])
            .unwrap()
        ),
        point_one.clone() * get_sim_sm2(3)
    );

    //assert_eq!(&point_one * get_sim_sm2(3), point_one * get_sim_sm2(3));

    assert_eq!(
        point_one.clone() * &get_sim_sm2(3),
        point_one.clone() * get_sim_sm2(3)
    );

    assert_eq!(
        &point_one * &get_sim_sm2(3),
        point_one.clone() * get_sim_sm2(3)
    );

    assert_eq!(
        Point(
            PointInner::from_bytes(&[
                2, 169, 127, 124, 212, 179, 201, 147, 180, 190, 45, 170, 140, 219, 65, 226, 76,
                161, 63, 107, 217, 69, 48, 34, 68, 226, 105, 24, 241, 208, 80, 158, 191
            ])
            .unwrap()
        ),
        get_sim_sm2(3) * point_one.clone()
    );

    assert_eq!(
        &get_sim_sm2(3) * point_one.clone(),
        point_one.clone() * get_sim_sm2(3)
    );

    assert_eq!(
        get_sim_sm2(3) * &point_one,
        point_one.clone() * get_sim_sm2(3)
    );

    assert_eq!(
        &get_sim_sm2(3) * &point_one,
        point_one.clone() * get_sim_sm2(3)
    );

    // 4493907448824000747700850167940867464579944529806937181821189941592931634714
    let scalar_ax = Scalar(
        ScalarInner::from_bytes(&[
            0x1a, 0x0e, 0x97, 0x8a, 0x90, 0xf6, 0x62, 0x2d, 0x37, 0x47, 0x02, 0x3f, 0x8a, 0xd8,
            0x26, 0x4d, 0xa7, 0x58, 0xaa, 0x1b, 0x88, 0xe0, 0x40, 0xd1, 0x58, 0x9e, 0x7b, 0x7f,
            0x23, 0x76, 0xef, 0x09,
        ])
        .unwrap(),
    );

    assert_eq!(
        scalar_ax.clone() * get_sim_sm2(5) * get_sim_sm2(3),
        scalar_ax.clone() * get_sim_sm2(15)
    );

    let hex_str = &point_one.to_bytes().encode_hex::<String>();
    assert_eq!(
        String::from("0232c4ae2c1f1981195f9904466a39c9948fe30bbff2660be1715a4589334c74c7"),
        *hex_str
    );

    let point_hex = Point(PointInner::from_bytes(&NewU833::from_hex(hex_str).unwrap().0).unwrap());

    assert_eq!(&point_one, &point_hex);
}

#[test]
fn test_point_json_serial() {
    let point_one = Point::<PointInner>::one();
    let json_str = serde_json::to_string(&point_one).unwrap();
    assert_eq!(
        String::from("\"0232C4AE2C1F1981195F9904466A39C9948FE30BBFF2660BE1715A4589334C74C7\""),
        *json_str
    );

    let point_json: Point<PointInner> = serde_json::from_str(&json_str[..]).unwrap();
    assert_eq!(&point_one, &point_json);

    println!("json_str: {:?}", json_str);
}

#[test]
fn test_fix_point_clone() {
    let point_a = Point::<PointInner>::from_bytes(&[
        2, 0, 48, 132, 111, 149, 100, 80, 34, 87, 192, 147, 221, 103, 158, 116, 106, 72, 195, 49,
        84, 69, 111, 79, 193, 123, 71, 208, 170, 251, 172, 159, 166,
    ])
    .unwrap();

    println!("{:?} ", point_a);
    println!("{:?} ", point_a.to_bytes());
    println!("{:?}", point_a.get_x().to_bytes());
}
