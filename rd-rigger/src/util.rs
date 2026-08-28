use std::time::Duration;

use crate::CieColor;

fn parse_floats(s: &str) -> Vec<f32> {
    s.split('}')
        .filter_map(|row| {
            let row = row.trim_start_matches('{').trim();
            if row.is_empty() { None } else { Some(row) }
        })
        .flat_map(|row| {
            row.split(',').filter_map(|num| {
                let num = num.trim();
                if num.is_empty() { None } else { num.parse::<f32>().ok() }
            })
        })
        .collect()
}

pub(crate) fn parse_cie_color_array(s: &str) -> Vec<CieColor> {
    parse_floats(s).chunks(3).map(|chunk| CieColor::new(chunk[0], chunk[1], chunk[2])).collect()
}

pub(crate) fn parse_vec2(s: &str) -> glam::Vec2 {
    let v = parse_floats(s);
    assert_eq!(v.len(), 2, "expected 2 floats, got {}: {:?}", v.len(), s);
    glam::Vec2::new(v[0], v[1])
}

pub(crate) fn parse_vec3(s: &str) -> glam::Vec3A {
    let v = parse_floats(s);
    assert_eq!(v.len(), 3, "expected 3 floats, got {}: {:?}", v.len(), s);
    glam::Vec3A::new(v[0], v[1], v[2])
}

pub(crate) fn parse_mat3(s: &str) -> glam::Mat3 {
    let v = parse_floats(s);
    assert_eq!(v.len(), 9, "expected 9 floats, got {}: {:?}", v.len(), s);
    glam::Mat3::from_cols_array(&[v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8]])
}

pub(crate) fn parse_affine3a(s: &str) -> glam::Affine3A {
    let rows: Vec<Vec<f32>> = s
        .split('}')
        .filter_map(|row| {
            let row = row.trim_start_matches('{').trim();
            if row.is_empty() { None } else { Some(parse_floats(row)) }
        })
        .collect();

    assert_eq!(rows.len(), 4, "expected 4 row groups, got {}: {:?}", rows.len(), s);
    assert!(rows.iter().all(|r| r.len() == 3), "each row must have 3 floats: {:?}", s);

    #[rustfmt::skip]
    let cols = [
        rows[0][0], rows[0][1], rows[0][2],
        rows[1][0], rows[1][1], rows[1][2],
        rows[2][0], rows[2][1], rows[2][2],
        rows[3][0] / 1000.0, rows[3][1] / 1000.0, rows[3][2] / 1000.0,
    ];

    glam::Affine3A::from_cols_array(&cols)
}

pub(crate) fn parse_affine3a_or_identity(s: Option<&str>) -> glam::Affine3A {
    s.map(parse_affine3a).unwrap_or(glam::Affine3A::IDENTITY)
}

pub(crate) fn parse_affine3a_from_mat4(s: &str) -> glam::Affine3A {
    let v = parse_floats(s);
    assert_eq!(v.len(), 16, "expected 16 floats for mat4, got {}: {:?}", v.len(), s);
    #[rustfmt::skip]
    let cols = [
        v[0], v[1], v[2],
        v[4], v[5], v[6],
        v[8], v[9], v[10],
        v[12] / 1000.0, v[13] / 1000.0, v[14] / 1000.0,
    ];
    glam::Affine3A::from_cols_array(&cols)
}

pub(crate) fn parse_possibly_negative_duration(duration: f32) -> Duration {
    match duration {
        v if v >= 0.0 => Duration::from_secs_f32(v),
        _ => {
            eprintln!("Negative duration found");
            Duration::default()
        }
    }
}
