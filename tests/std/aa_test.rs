mod assets {
    use std::env;
    use std::path::PathBuf;
    // https://www.reddit.com/r/rust/comments/ahsz9q/psa_if_the_examples_for_your_crate_rely_on_media/

    pub fn is_base_dir(cwd: &PathBuf) -> Option<(PathBuf, PathBuf)> {
        if !cwd.join("Cargo.toml").is_file() {
            return None;
        }
        let mut images = cwd.clone();
        let mut test_tmp = cwd.clone();
        images.push("images");
        test_tmp.push("tests");
        test_tmp.push("std");
        test_tmp.push("tmp");
        if images.is_dir() && test_tmp.is_dir() {
            Some((images, test_tmp))
        } else {
            None
        }
    }

    pub fn find_assets() -> Option<(PathBuf, PathBuf)> {
        // First check currnet directory
        let cwd = env::current_dir().ok()?;
        if let Some(v) = is_base_dir(&cwd) {
            return Some(v);
        }
        // Search backwards from current executable path
        let mut exec = env::current_exe().ok()?;
        while let Some(dir) = exec.parent() {
            if let Some(v) = is_base_dir(&dir.to_path_buf()) {
                return Some(v);
            }
            exec = dir.to_path_buf();
        }
        // Could not find base directory
        None
    }
}

use agigea::Render;

fn path_from_slice(pts: &[f64]) -> agigea::Path {
    assert!(pts.len() % 2 == 0);
    assert!(pts.len() >= 4);
    let mut path = agigea::Path::new();
    path.move_to(pts[0] + 0.5, pts[1] + 0.5);
    for i in (2..pts.len()).step_by(2) {
        path.line_to(pts[i] + 0.5, pts[i + 1] + 0.5);
    }
    path
}

#[test]
fn t26_aa_test() {
    let (mut images, mut output) = assets::find_assets().unwrap();

    let (width, height) = (480, 350);
    let pix = agigea::Pixfmt::<agigea::Rgb8>::new(width, height);
    let mut ren_base = agigea::RenderingBase::new(pix);

    ren_base.clear(agigea::Rgba8::new(0, 0, 0, 255));

    // Radial Line Test
    let cx = width as f64 / 2.0;
    let cy = height as f64 / 2.0;
    let r = if cx < cy { cx } else { cy };

    let mut ras = agigea::RasterizerScanline::new();
    {
        let mut ren = agigea::RenderingScanlineAASolid::with_base(&mut ren_base);
        ren.color(agigea::Rgba8::new(255, 255, 255, 51));
        for i in (1..=180).rev() {
            ras.reset();
            let n = 2.0 * (i as f64) * core::f64::consts::PI / 180.0;
            let mut path = agigea::Path::new();
            path.move_to((cx + r * n.sin()) + 0.5, (cy + r * n.cos()) + 0.5);
            path.line_to(cx + 0.5, cy + 0.5);
            if i < 90 {
                let mut dash = agigea::Dash::new(path);
                dash.add_dash(i as f64, i as f64);
                let mut stroke = agigea::Stroke::new(dash);
                stroke.width(1.0);
                stroke.line_cap(agigea::LineCap::Round);
                ras.add_path(&stroke);
            } else {
                let mut stroke = agigea::Stroke::new(path);
                stroke.width(1.0);
                stroke.line_cap(agigea::LineCap::Round);
                ras.add_path(&stroke);
            }
            agigea::render_scanlines(&mut ras, &mut ren);
        }
    }

    for i in 1..=20 {
        let k = i as f64;
        let mut ren = agigea::RenderingScanlineAASolid::with_base(&mut ren_base);
        // Integral Point Sizes 1..=20
        ras.reset();
        ren.color(agigea::Rgb8::white());
        let ell = agigea::Ellipse::new(20.0 + k * (k + 1.0) + 0.5, 20.5, k / 2.0, k / 2.0, 8 + i);
        ras.add_path(&ell);
        agigea::render_scanlines(&mut ras, &mut ren);

        // Fractional Point Sizes 0..=2
        ras.reset();
        let ell = agigea::Ellipse::new(18. + (k * 4.0) + 0.5, 33. + 0.5, k / 20.0, k / 20.0, 8);
        ras.add_path(&ell);
        agigea::render_scanlines(&mut ras, &mut ren);

        // Fractional Point Positioning
        ras.reset();
        let ell = agigea::Ellipse::new(
            18. + (k * 4.0) + (k - 1.0) / 10.0 + 0.5,
            27. + (k - 1.0) / 10.0 + 0.5,
            0.5,
            0.5,
            8,
        );
        ras.reset();
        ras.add_path(&ell);
        agigea::render_scanlines(&mut ras, &mut ren);

        // Integral Line Widths 1..=20
        let gradient_colors = color_gradient(
            agigea::Rgb8::white(),
            agigea::Rgb8::new(
                ((255.) * (i % 2) as f64).round() as u8,
                ((255. / 2.) * (i % 3) as f64).round() as u8,
                ((255. / 4.) * (i % 5) as f64).round() as u8,
            ),
            256,
        );
        let x1 = 20.0 + k * (k + 1.0);
        let y1 = 40.5;
        let x2 = 20.0 + k * (k + 1.0) + ((k - 1.0) * 4.0);
        let y2 = 100.5;
        let gradient_mtx = calc_linear_gradient_transform(x1, y1, x2, y2);
        let span = agigea::SpanGradient::new(
            gradient_mtx,
            agigea::GradientX {},
            &gradient_colors,
            0.0,
            100.0,
        );
        let mut ren_grad = agigea::RenderingScanlineAA::new(&mut ren_base, span);
        let path = path_from_slice(&[x1, y1, x2, y2]);
        let mut stroke = agigea::Stroke::new(path);
        stroke.width(k);
        stroke.line_cap(agigea::LineCap::Round);
        ras.reset();
        ras.add_path(&stroke);
        agigea::render_scanlines(&mut ras, &mut ren_grad);

        // Fractional Line Lengths H (Red/Blue)
        let gradient_colors =
            color_gradient(agigea::Rgb8::new(255, 0, 0), agigea::Rgb8::new(0, 0, 255), 256);
        let x1 = 17.5 + (k * 4.0);
        let y1 = 107.;
        let x2 = 17.5 + (k * 4.0) + k / 6.66666667;
        let y2 = 107.;
        let gradient_mtx = calc_linear_gradient_transform(x1, y1, x2, y2);
        let span = agigea::SpanGradient::new(
            gradient_mtx,
            agigea::GradientX {},
            &gradient_colors,
            0.0,
            100.0,
        );
        let mut ren_grad = agigea::RenderingScanlineAA::new(&mut ren_base, span);
        let path = path_from_slice(&[x1, y1, x2, y2]);
        let mut stroke = agigea::Stroke::new(path);
        stroke.width(1.0);
        stroke.line_cap(agigea::LineCap::Round);
        ras.reset();
        ras.add_path(&stroke);
        agigea::render_scanlines(&mut ras, &mut ren_grad);

        // Fractional Line Lengths V (Red/Blue)
        let x1 = 18.0 + (k * 4.0);
        let y1 = 112.5;
        let x2 = 18.0 + (k * 4.0);
        let y2 = 112.5 + k / 6.66666667;
        let gradient_mtx = calc_linear_gradient_transform(x1, y1, x2, y2);
        let span = agigea::SpanGradient::new(
            gradient_mtx,
            agigea::GradientX {},
            &gradient_colors,
            0.0,
            100.0,
        );
        let mut ren_grad = agigea::RenderingScanlineAA::new(&mut ren_base, span);
        let path = path_from_slice(&[x1, y1, x2, y2]);
        let mut stroke = agigea::Stroke::new(path);
        stroke.width(1.0);
        stroke.line_cap(agigea::LineCap::Round);
        ras.reset();
        ras.add_path(&stroke);
        agigea::render_scanlines(&mut ras, &mut ren_grad);

        // Fractional Line Positioning (Red)
        let colors =
            color_gradient(agigea::Rgb8::new(255, 0, 0), agigea::Rgb8::new(255, 255, 255), 256);
        let x1 = 21.5;
        let y1 = 120.0 + (k - 1.0) * 3.1;
        let x2 = 52.5;
        let y2 = 120.0 + (k - 1.0) * 3.1;
        let gradient_mtx = calc_linear_gradient_transform(x1, y1, x2, y2);
        let span =
            agigea::SpanGradient::new(gradient_mtx, agigea::GradientX {}, &colors, 0.0, 100.0);
        let mut ren_grad = agigea::RenderingScanlineAA::new(&mut ren_base, span);
        let path = path_from_slice(&[x1, y1, x2, y2]);
        let mut stroke = agigea::Stroke::new(path);
        stroke.width(1.0);
        stroke.line_cap(agigea::LineCap::Round);
        ras.reset();
        ras.add_path(&stroke);
        agigea::render_scanlines(&mut ras, &mut ren_grad);

        // Fractional Line Widths 2..0 (Green)
        let colors =
            color_gradient(agigea::Rgb8::new(0, 255, 0), agigea::Rgb8::new(255, 255, 255), 256);
        let x1 = 52.5;
        let y1 = 118.0 + (k * 3.0);
        let x2 = 83.5;
        let y2 = 118.0 + (k * 3.0);
        let gradient_mtx = calc_linear_gradient_transform(x1, y1, x2, y2);
        let span =
            agigea::SpanGradient::new(gradient_mtx, agigea::GradientX {}, &colors, 0.0, 100.0);
        let mut ren_grad = agigea::RenderingScanlineAA::new(&mut ren_base, span);
        let path = path_from_slice(&[x1, y1, x2, y2]);
        let mut stroke = agigea::Stroke::new(path);
        stroke.width(2.0 - (k - 1.0) / 10.0);
        stroke.line_cap(agigea::LineCap::Round);
        ras.reset();
        ras.add_path(&stroke);
        agigea::render_scanlines(&mut ras, &mut ren_grad);

        // Stippled Fractional Width 2..0 (Blue)
        let colors =
            color_gradient(agigea::Rgb8::new(0, 0, 255), agigea::Rgb8::new(255, 255, 255), 256);
        let x1 = 83.5;
        let y1 = 119.0 + (k * 3.0);
        let x2 = 114.5;
        let y2 = 119.0 + (k * 3.0);
        let gradient_mtx = calc_linear_gradient_transform(x1, y1, x2, y2);
        let span =
            agigea::SpanGradient::new(gradient_mtx, agigea::GradientX {}, &colors, 0.0, 100.0);
        let mut ren_grad = agigea::RenderingScanlineAA::new(&mut ren_base, span);
        let path = path_from_slice(&[x1, y1, x2, y2]);
        let mut dash = agigea::Dash::new(path);
        dash.add_dash(3.0, 3.0);
        let mut stroke = agigea::Stroke::new(dash);
        stroke.width(2.0 - (k - 1.0) / 10.0);
        stroke.line_cap(agigea::LineCap::Round);
        ras.reset();
        ras.add_path(&stroke);
        agigea::render_scanlines(&mut ras, &mut ren_grad);

        let mut ren = agigea::RenderingScanlineAASolid::with_base(&mut ren_base);
        ren.color(agigea::Rgb8::new(255, 255, 255));
        if i <= 10 {
            // Integral line width, horz aligned (mipmap test)
            let path = path_from_slice(&[
                125.5,
                119.5 + (k + 2.0) * (k / 2.0),
                135.5,
                119.5 + (k + 2.0) * (k / 2.0),
            ]);
            let mut stroke = agigea::Stroke::new(path);
            stroke.width(k);
            stroke.line_cap(agigea::LineCap::Round);
            ras.reset();
            ras.add_path(&stroke);
            agigea::render_scanlines(&mut ras, &mut ren);
        }
        // Fractional line width 0..2, 1 px H
        let path = path_from_slice(&[17.5 + (k * 4.0), 192.0, 18.5 + (k * 4.0), 192.0]);
        let mut stroke = agigea::Stroke::new(path);
        stroke.width(k / 10.0);
        stroke.line_cap(agigea::LineCap::Round);
        ras.reset();
        ras.add_path(&stroke);
        agigea::render_scanlines(&mut ras, &mut ren);

        // Fractional line positioning, 1 px H
        let path = path_from_slice(&[
            17.5 + (k * 4.0) + (k - 1.0) / 10.0,
            186.0,
            18.5 + (k * 4.0) + (k - 1.0) / 10.0,
            186.0,
        ]);
        let mut stroke = agigea::Stroke::new(path);
        stroke.width(1.0);
        stroke.line_cap(agigea::LineCap::Round);
        ras.reset();
        ras.add_path(&stroke);
        agigea::render_scanlines(&mut ras, &mut ren);
    }

    let mut ren = agigea::RenderingScanlineAASolid::with_base(&mut ren_base);
    ren.color(agigea::Rgb8::white());
    for i in 1..=13 {
        let k = i as f64;
        ras.reset();
        let gradient_colors = color_gradient(
            agigea::Rgb8::white(),
            agigea::Rgb8::new(
                ((255.) * (i % 2) as f64).round() as u8,
                ((255. / 2.) * (i % 3) as f64).round() as u8,
                ((255. / 4.) * (i % 5) as f64).round() as u8,
            ),
            256,
        );
        let x1 = width as f64 - 150.;
        let y1 = height as f64 - 20. - k * (k + 1.5);
        let x2 = width as f64 - 20.;
        let y2 = height as f64 - 20. - k * (k + 1.0);

        let gradient_mtx = calc_linear_gradient_transform(x1, y1, x2, y2);
        let span = agigea::SpanGradient::new(
            gradient_mtx,
            agigea::GradientX {},
            &gradient_colors,
            0.0,
            100.0,
        );
        let mut ren_grad = agigea::RenderingScanlineAA::new(&mut ren_base, span);
        ras.move_to(width as f64 - 150., height as f64 - 20. - k * (k + 1.5));
        ras.line_to(width as f64 - 20., height as f64 - 20. - k * (k + 1.0));
        ras.line_to(width as f64 - 20., height as f64 - 20. - k * (k + 2.0));
        agigea::render_scanlines(&mut ras, &mut ren_grad);
    }

    output.push("aa_test.png");
    images.push("aa_test.png");
    ren_base.to_file(&output).unwrap();
    assert_eq!(agigea::ppm::img_diff(output, images).unwrap(), true);
}

fn calc_linear_gradient_transform(x1: f64, y1: f64, x2: f64, y2: f64) -> agigea::Transform {
    let gradient_d2 = 100.0;
    let dx = x2 - x1;
    let dy = y2 - y1;
    let mut mtx = agigea::Transform::new();
    let s = (dx * dx + dy * dy).sqrt() / gradient_d2;
    mtx = mtx * agigea::Transform::new_scale(s, s);
    mtx = mtx * agigea::Transform::new_rotate(dy.atan2(dx));
    mtx = mtx * agigea::Transform::new_translate(x1 + 0.5, y1 + 0.5);
    mtx.invert();

    // Above is equivalent to this
    // let mut mtx2 = agigea::Transform::new();
    // mtx2.scale(s,s);
    // mtx2.rotate(dy.atan2(dx));
    // mtx2.translate(x1+0.5, y1+0.5);
    // mtx2.invert();
    // assert!(mtx == mtx2);

    mtx
}

fn color_gradient(begin: agigea::Rgb8, end: agigea::Rgb8, len: usize) -> Vec<agigea::Rgb8> {
    let mut gradient_colors = vec![agigea::Rgb8::white(); len];
    fill_color_array(&mut gradient_colors, begin, end);
    gradient_colors
}

fn fill_color_array(array: &mut [agigea::Rgb8], begin: agigea::Rgb8, end: agigea::Rgb8) {
    let n = (array.len() - 1) as f64;
    for (i, v) in array.iter_mut().enumerate() {
        let a = ((i as f64 / n) * 255.0).round() as u8;
        v.r = agigea::math::lerp_u8(begin.r, end.r, a);
        v.g = agigea::math::lerp_u8(begin.g, end.g, a);
        v.b = agigea::math::lerp_u8(begin.b, end.b, a);
    }
}
