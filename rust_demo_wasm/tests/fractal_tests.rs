use wasm_bindgen_test::*;
use rust_demo_wasm::FractalEngine;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_fractal_generation() {
    let engine = FractalEngine::new();
    let result = engine.generate(1, -2.0, 1.0, -1.5, 1.5, 20);
    
    assert!(result.is_ok(), "Fractal generation returned an error");
    
    let array = result.unwrap();
    let expected_len = 800 * 600 * 3;
    assert_eq!(array.length(), expected_len, "Array length does not match expected dimensions");
    
    let mut buffer = vec![0.0; 3];
    array.copy_to(&mut buffer[..]);
    assert!(buffer[2] >= 0.0 && buffer[2] <= 255.0, "Intensity value out of range");
}

