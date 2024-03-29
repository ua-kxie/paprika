use num_complex::Complex64;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
/// Struct known as vecvaluesall in Ngspice User's Manual
pub struct PkVecvaluesall{
    pub count: i32,
    pub index: i32,
    pub vecsa: Vec<Box<PkVecvalues>>,
}
#[derive(Clone, PartialEq, PartialOrd, Debug)]
/// Struct known as vecvalues in Ngspice User's Manual
pub struct PkVecvalues {
    pub name: String,
    pub creal: f64,
    pub cimag: f64,
    pub is_scale: bool,
    pub is_complex: bool,
}
#[derive(Clone, Debug)]
/// Struct known as vecinfoall in Ngspice User's Manual
pub struct PkVecinfoall{
    pub name: String,
    pub title: String,
    pub date: String,
    pub stype: String,
    pub count: i32,
    pub vecs: Vec<Box<PkVecinfo>>,
}
#[derive(Clone, Debug)]
#[allow(dead_code)]
/// Struct known as vecinfo in Ngspice User's Manual
pub struct PkVecinfo {
    pub number: i32,
    pub name: String,
    pub is_real: bool,
    pub pdvec: usize,  
    pub pdvecscale: usize,
}
#[derive(Copy, Clone, Debug)]
pub struct PkComplex {
    pub real: f64,
    pub imag: f64,
}
#[derive(Clone, Debug)]
pub struct PkVectorinfo {
    pub name: String,
    pub stype: i32,
    pub flag: i16,
    pub realdata: Option<Vec<f64>>,
    pub compdata: Option<Vec<Complex64>>,
    pub length: i32,
}