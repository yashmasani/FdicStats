use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;


pub fn draw(canvas_id: &str, x: &[f32], y: &[f32]) -> DrawResult<impl Fn((i32, i32)) -> Option<(f32, f32)>> {
     
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();
    let power = 2; 
    root.fill(&WHITE)?;
    let x_max = x.iter().max_by(|a,b| a.partial_cmp(b).unwrap()).unwrap_or(&2000f32);
    let y_max = y.iter().max_by(|a,b| a.partial_cmp(b).unwrap()).unwrap_or(&2000f32);
    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .caption(format!("y=x^{}", power), font)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(1f32..*x_max, 1.2f32..*y_max)?;

    chart.configure_mesh().x_labels(20).y_labels(20).draw()?;

    chart.draw_series(LineSeries::new(
        (0..x.len())
            .map(|i| (x[i],y[i])),
        &RED,
    ))?;

    root.present()?;
    Ok(chart.into_coord_trans())
}

/// Type used on the JS side to convert screen coordinates to chart
/// coordinates.
#[wasm_bindgen]
pub struct Chart {
    convert: Box<dyn Fn((i32, i32)) -> Option<(f64, f64)>>,
}

#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Chart {
    /// Draw provided power function on the canvas element using it's id.
    /// Return `Chart` struct suitable for coordinate conversion.
    pub fn draw(canvas_id: &str, x: &[f32], y: &[f32]) -> Result<Chart, JsValue> {
        let map_coord = draw(canvas_id, x, y).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }
    /// This function can be used to convert screen coordinates to
    /// chart coordinates.
    pub fn coord(&self, x: i32, y: i32) -> Option<Point> {
        (self.convert)((x, y)).map(|(x, y)| Point { x, y })
    }
}
