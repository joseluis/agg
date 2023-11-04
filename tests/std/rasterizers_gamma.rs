use agigea::Render;

fn rgb64(r: f64, g: f64, b: f64, a: f64) -> agigea::Rgba8 {
    agigea::Rgba8::new(
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
        (a * 255.0).round() as u8,
    )
}

#[test]
fn rasterizers_gamma() {
    let (w, h) = (500, 330);

    let m_x = [100. + 120., 369. + 120., 143. + 120.];
    let m_y = [60., 170., 310.0];

    let pixf = agigea::Pixfmt::<agigea::Rgb8>::new(w, h);
    let mut ren_base = agigea::RenderingBase::new(pixf);
    ren_base.clear(agigea::Rgba8::new(255, 255, 255, 255));

    let gamma = 1.0;
    let alpha = 0.5;

    let mut ras = agigea::RasterizerScanline::new();

    // Anti-Aliased
    {
        let mut ren_aa = agigea::RenderingScanlineAASolid::with_base(&mut ren_base);
        let mut path = agigea::Path::new();

        path.move_to(m_x[0], m_y[0]);
        path.line_to(m_x[1], m_y[1]);
        path.line_to(m_x[2], m_y[2]);
        path.close_polygon();
        ren_aa.color(rgb64(0.7, 0.5, 0.1, alpha));
        ras.add_path(&path);
        // Power Function
        ras.gamma(|v| (v.powf(gamma * 2.0)));
        agigea::render_scanlines(&mut ras, &mut ren_aa);
    }

    // Aliased
    {
        let mut ren_bin = agigea::RenderingScanlineBinSolid::with_base(&mut ren_base);
        let mut path = agigea::Path::new();

        path.move_to(m_x[0] - 200., m_y[0]);
        path.line_to(m_x[1] - 200., m_y[1]);
        path.line_to(m_x[2] - 200., m_y[2]);
        path.close_polygon();
        ren_bin.color(rgb64(0.1, 0.5, 0.7, alpha));
        ras.add_path(&path);
        // Threshold
        ras.gamma(|v| if v < gamma { 0.0 } else { 1.0 });
        agigea::render_scanlines(&mut ras, &mut ren_bin);
    }
    ren_base.to_file("tests/std/tmp/rasterizers_gamma.png").unwrap();
    assert_eq!(
        agigea::ppm::img_diff(
            "tests/std/tmp/rasterizers_gamma.png",
            "images/rasterizers_gamma.png"
        )
        .unwrap(),
        true
    );
}
