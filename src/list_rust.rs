extern crate ndarray;
use ndarray::{parallel::prelude::*, Axis};
use numpy::{PyArray2, PyReadonlyArray2};
use pyo3::wrap_pyfunction;
use pyo3::{
    prelude::*,
    types::{PyFloat, PyList},
};
use rayon::prelude::*;

#[pyfunction]
fn iterate_list_py<'py>(py: Python<'py>, a_list: PyReadonlyArray2<'_, f64>) -> f64 {
    // Idiomatic way
    //a_list.as_array().sum()

    // To comply with "Algorithm"
    let sum = a_list
        .as_array()
        .axis_iter(Axis(0))
        .map(|row| row.sum())
        .sum();
    println!("{}", sum);
    sum
}

#[pyfunction]
fn iterate_list_multi_py<'py>(py: Python<'py>, a_list: PyReadonlyArray2<'_, f64>) -> f64 {
    let sum = a_list
        .as_array()
        .axis_iter(Axis(0))
        .into_par_iter()
        .map(|row| row.sum())
        .sum();
    println!("{}", sum);
    sum
}

#[pyfunction]
fn make_list_py<'py>(py: Python<'py>, a_list: &'py PyArray2<f64>) -> &'py PyArray2<f64> {
    unsafe {
        // Idiomatic way
        // a_list.as_array_mut().fill(0.01);

        // Match the Algorithm description explicitly
        a_list
            .as_array_mut()
            .axis_iter_mut(Axis(0))
            .for_each(|mut row| row.fill(0.01))
    }
    a_list
}

#[pyfunction]
fn iterate_list(a_list: &PyList) -> f64 {
    let mut count = 0.0;
    for i in 0..a_list.len() {
        let list: &PyList = a_list.get_item(i as isize).cast_as().unwrap();

        for j in 0..list.len() {
            let value: &PyFloat = list.get_item(j as isize).cast_as().unwrap();
            count += value.value();
        }
    }
    println!("{}", count);
    count
}

#[pyfunction]
fn make_list<'py>(py: Python<'py>, a_list: &'py PyList) -> &'py PyList {
    let mut test: Vec<Vec<f64>> = Vec::new();
    for _i in 0..i64::pow(10, 4) {
        let mut new_list: Vec<f64> = Vec::new();
        for _j in 0..i64::pow(10, 4) {
            new_list.push(0.01);
        }
        test.push(new_list);
    }
    PyList::new(py, test)
}

#[pymodule]
fn list_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(iterate_list_py, m)?)
        .unwrap();
    m.add_function(wrap_pyfunction!(iterate_list_multi_py, m)?)
        .unwrap();
    m.add_function(wrap_pyfunction!(make_list_py, m)?).unwrap();
    m.add_function(wrap_pyfunction!(iterate_list, m)?).unwrap();
    m.add_function(wrap_pyfunction!(make_list, m)?).unwrap();

    Ok(())
}
