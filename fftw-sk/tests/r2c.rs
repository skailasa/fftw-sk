use fftw_sk::plan::*;
use fftw_sk::types::*;

/// Check successive forward and backward transform equals to the identity
#[test]
fn c2r2c_identity() {
    let n = 32;
    let mut a = vec![c64::default(); n / 2 + 1];
    let mut b = vec![0.0; n];
    let c2r: C2RPlan64 = C2RPlan::new(&[n], &mut a, &mut b, Flag::MEASURE).unwrap();
    let r2c: R2CPlan64 = R2CPlan::new(&[n], &mut b, &mut a, Flag::MEASURE).unwrap();
    for i in 0..(n / 2 + 1) {
        a[i] = [1.0, 0.0];
    }
    c2r.c2r(&mut a, &mut b).unwrap();
    r2c.r2c(&mut b, &mut a).unwrap();
    
    for v in a.iter() {
        let ans = [n as f64, 0.0];
        let dif = ((v[0] - ans[0]).powf(2.) + (v[1]-ans[1]).powf(2.0)).powf(0.5);
        if dif > 1e-7 {
            panic!("Large difference: v={:?}, dif={}", v, dif);
        }
    }
}