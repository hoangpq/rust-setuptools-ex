#![crate_type = "dylib"]

#[macro_use]
extern crate cpython;
extern crate reqwest;

use cpython::{PyDict, PyList, PyObject, PyResult, PyString, PyTuple, Python};
use std::collections::HashMap;
use cpython::ToPyObject;

py_module_initializer!(helloworld, inithelloworld, PyInit_helloworld, |py, m| {
    try!(m.add(py, "__doc__", "Module documentation string"));
    try!(m.add(py, "run", py_fn!(py, run(*args, **kwargs))));
    try!(m.add(py, "val", py_fn!(py, val())));
    try!(m.add(py, "fetch_ip", py_fn!(py, fetch_ip())));
    Ok(())
});

fn fetch_ip_() -> Result<String, reqwest::Error> {
    let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")?.json()?;

    let res = resp.iter().map(|item| item.1).collect::<Vec<_>>();

    let ips = res[0]
        .split(',')
        .map(|s| s.trim_start())
        .collect::<Vec<_>>();

    Ok(ips[0].to_string())
}

fn fetch_ip(py: Python) -> PyResult<String> {
    Ok(fetch_ip_().unwrap())
}

fn run(py: Python, args: &PyTuple, kwargs: Option<&PyDict>) -> PyResult<PyObject> {
    println!("Rust says: Hello Python!");
    for arg in args.iter(py) {
        println!("Rust got {}", arg);
    }
    if let Some(kwargs) = kwargs {
        for (key, val) in kwargs.items(py) {
            println!("{} = {}", key, val);
        }
    }
    Ok(py.None())
}

fn val(_: Python) -> PyResult<i32> {
    // Ok(42)
    Ok(24)
}
