// Dumps every paper-space viewport's view fields from a DWG/DXF.
// cargo run --release --example inspect_viewports -- <path>
use acadrust::entities::EntityType;
use acadrust::io::dwg::DwgReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).expect("usage: inspect_viewports <file>");
    let doc = DwgReader::from_file(&path)?.read()?;
    println!("file: {}", path);
    println!("total entities: {}", doc.entities().count());
    let unit_name = match doc.header.insertion_units {
        0 => "Unitless",
        1 => "Inches",
        2 => "Feet",
        3 => "Miles",
        4 => "Millimeters",
        5 => "Centimeters",
        6 => "Meters",
        7 => "Kilometers",
        _ => "Other",
    };
    println!(
        "header.insertion_units = {} ({})",
        doc.header.insertion_units, unit_name
    );
    println!(
        "header.model_space_extents min=({:.3}, {:.3}, {:.3}) max=({:.3}, {:.3}, {:.3})",
        doc.header.model_space_extents_min.x,
        doc.header.model_space_extents_min.y,
        doc.header.model_space_extents_min.z,
        doc.header.model_space_extents_max.x,
        doc.header.model_space_extents_max.y,
        doc.header.model_space_extents_max.z,
    );
    println!();

    let viewports: Vec<_> = doc
        .entities()
        .filter_map(|e| {
            if let EntityType::Viewport(v) = e {
                Some(v.clone())
            } else {
                None
            }
        })
        .collect();
    println!("viewports: {}\n", viewports.len());

    for (i, vp) in viewports.iter().enumerate() {
        let scale_from_view_height = if vp.view_height.abs() > 1e-9 {
            vp.height / vp.view_height
        } else {
            f64::NAN
        };
        println!(
            "[{}] id={} handle={:?}",
            i, vp.id, vp.common.handle
        );
        println!(
            "    paper center=({:.3}, {:.3})  size={:.3} × {:.3}",
            vp.center.x, vp.center.y, vp.width, vp.height
        );
        println!(
            "    view_target=({:.3}, {:.3}, {:.3})",
            vp.view_target.x, vp.view_target.y, vp.view_target.z
        );
        println!(
            "    view_direction=({:.3}, {:.3}, {:.3})",
            vp.view_direction.x, vp.view_direction.y, vp.view_direction.z
        );
        println!(
            "    view_center=({:.3}, {:.3})",
            vp.view_center.x, vp.view_center.y
        );
        println!(
            "    view_height={:.6}   == vp.height? {}   ⇒ height/view_height = {:.6}",
            vp.view_height,
            (vp.view_height - vp.height).abs() < 1e-6,
            scale_from_view_height
        );
        println!(
            "    custom_scale={:.6}   twist={:.4}   lens={:.2}",
            vp.custom_scale, vp.twist_angle, vp.lens_length
        );
        println!(
            "    status: on={}, locked={}, perspective={}",
            vp.status.is_on, vp.status.locked, vp.status.perspective
        );
        println!();
    }

    // Summary stats
    let n_with_view_eq_height = viewports
        .iter()
        .filter(|v| (v.view_height - v.height).abs() < 1e-6)
        .count();
    let n_with_target_zero = viewports
        .iter()
        .filter(|v| {
            v.view_target.x.abs() < 1e-9
                && v.view_target.y.abs() < 1e-9
                && v.view_target.z.abs() < 1e-9
        })
        .count();
    let unique_view_heights: std::collections::BTreeSet<u64> = viewports
        .iter()
        .map(|v| v.view_height.to_bits())
        .collect();

    println!("─────── summary ───────");
    println!(
        "  view_height == vp.height  : {}/{}",
        n_with_view_eq_height,
        viewports.len()
    );
    println!(
        "  view_target == (0, 0, 0)  : {}/{}",
        n_with_target_zero,
        viewports.len()
    );
    println!(
        "  unique view_height values : {}",
        unique_view_heights.len()
    );
    Ok(())
}
