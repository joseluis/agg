#[test]
fn t20_outline_render() {
    use agigea::{DrawOutline, Pixfmt, Rgb8, Rgba8};
    use agigea::{RasterizerOutlineAA, RendererOutlineAA};
    let pix = Pixfmt::<Rgb8>::new(100, 100);
    let mut ren_base = agigea::RenderingBase::new(pix);
    ren_base.clear(Rgba8::new(255, 255, 255, 255));
    let mut ren = RendererOutlineAA::with_base(&mut ren_base);
    ren.color(agigea::Rgba8::new(0, 0, 0, 255));
    ren.width(20.0);

    let mut path = agigea::Path::new();
    path.move_to(10.0, 10.0);
    path.line_to(50.0, 90.0);
    path.line_to(90.0, 10.0);

    let mut ras = RasterizerOutlineAA::with_renderer(&mut ren);
    ras.round_cap(true);
    ras.add_path(&path);
    ren_base.to_file("tests/std/tmp/outline_aa.png").unwrap();

    assert!(agigea::ppm::img_diff("tests/std/tmp/outline_aa.png", "images/outline_aa.png").unwrap());
}
