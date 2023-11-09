use crate::axis_aligned_rectangle::AxisAlignedRectangle;
use crate::component::Component;
use crate::dividing::Dividing;
use crate::point::Point;
use crate::rectangle::{Rectangle, RectangleSize};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct JSRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[wasm_bindgen]
pub fn dividing(
    rect: JsValue,
    weights: &[f32],
    aspect_ratio: f32,
    vertical_first: bool,
    boustrophedron: bool,
) -> Result<JsValue, JsValue> {
    let Ok(rect) = serde_wasm_bindgen::from_value::<JSRect>(rect) else {
        return Err(JsValue::from_str("failed to parse rect"));
    };
    let rect =
        AxisAlignedRectangle::new(&Point::new(rect.x, rect.y), &Rectangle::new(rect.w, rect.h));
    let rects = match vertical_first {
        true => {
            rect.divide_vertical_then_horizontal_with_weights(weights, aspect_ratio, boustrophedron)
        }
        false => {
            rect.divide_horizontal_then_vertical_with_weights(weights, aspect_ratio, boustrophedron)
        }
    };

    let js_rects = rects
        .iter()
        .map(|rect| JSRect {
            x: rect.x(),
            y: rect.y(),
            w: rect.width(),
            h: rect.height(),
        })
        .collect::<Vec<_>>();

    serde_wasm_bindgen::to_value(&js_rects).map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    #[wasm_bindgen_test]
    fn test_basis() {
        let result = dividing(
            serde_wasm_bindgen::to_value(&JSRect {
                x: 0.0,
                y: 0.0,
                w: 100.0,
                h: 100.0,
            })
            .unwrap(),
            &[1.0, 1.0],
            1.0,
            true,
            false,
        )
        .unwrap();
        let result: Vec<JSRect> = serde_wasm_bindgen::from_value(result).unwrap();
        assert_eq!(
            result,
            vec![
                JSRect {
                    x: 0.0,
                    y: 0.0,
                    w: 100.0,
                    h: 50.0
                },
                JSRect {
                    x: 0.0,
                    y: 50.0,
                    w: 100.0,
                    h: 50.0
                }
            ]
        );
    }
}
