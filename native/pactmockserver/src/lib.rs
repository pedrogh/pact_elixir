// #[macro_use] extern crate rustler;
// #[macro_use] extern crate lazy_static;

// Disabled for now, but still
// wondering why rustler generated this in the first place
// #[macro_use] extern crate rustler_codegen;

use rustler::{NifResult, Term, Env, Encoder};
use rustler::types::Atom;
extern crate pact_mock_server;
extern crate libc;

// use pact_mock_server::create_mock_server;// as create_mock_server_a;
use pact_mock_server::create_mock_server;
use pact_mock_server::MockServerError; // unused because of the way the fuctions are being implemented.
use pact_mock_server::mock_server_mismatches;
use pact_mock_server::mock_server_matched;
use pact_mock_server::write_pact_file;
use pact_mock_server::WritePactFileErr; 
use pact_mock_server::shutdown_mock_server;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};


mod atoms {
    rustler::atoms! {
        ok,
        error,
        mock_server_failed_to_start,
        invalid_pact_json,
        io_error,
        no_mock_server_running_on_port,
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler::init!(
    "Elixir.PactElixir.RustPactMockServerFacade", 
    [
        create_mock_server_call,
        mock_server_mismatches_call,
        mock_server_matched_call,
        write_pact_file_call,
        cleanup_mock_server_call
    ]
);

#[rustler::nif(name = "create_mock_server")]
// fn create_mock_server_call<'a>(env: Env<'a>, pact_json:&str, port_arg:std::net::SocketAddr) -> NifResult<Term<'a>> {
fn create_mock_server_call<'a>(env: Env<'a>, pact_json:&str, port_arg:u16) -> NifResult<Term<'a>> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port_arg);
    // let addr = std::net::SocketAddr::from(port_arg);
    // let port = create_mock_server(pact_json, addr).context(format!("Unable to create server"), )?;
    let port = create_mock_server(pact_json, addr);
    // let mut addr = std::net::SocketAddr::into(port);
    // if let Err(err) = create_mock_server(pact_json, addr) {
    //     eprintln!("Error: {:?}", err);
    //     std::process::exit(1);
    // }

    match port {
        Ok(port) => {
            // Ok((atoms::ok(), port).encode(env))
            // let addr = std::net::SocketAddr::into(port);
            // (port).encode(env)
            Ok(((atoms::ok(), port)).encode(env))
        },
        Err(_e) => {
            Ok( (atoms::error(), atoms::invalid_pact_json()).encode(env) )
         }

        // Err(MockServerError::MockServerFailedToStart) =>
        //     Ok( (atoms::error(), atoms::mock_server_failed_to_start()).encode(env) ),
        // Err(MockServerError::InvalidPactJson) => 
        //     Ok( (atoms::error(), atoms::invalid_pact_json()).encode(env) )

        // Err(MockServerError::MockServerFailedToStart.into()) => 
        //     Ok( (atoms::error(), atoms::invalid_pact_json()).encode(env) )
        // Err(port) => Err(MockServerError::MockServerFailedToStart),

        // Err(_e) => Err(err) => match err.downcast_ref::<MockServerError>() {
        // }
        // Err(MockServerError::MockServerFailedToStart.into()) => 
        //     Ok( (atoms::error(), atoms::invalid_pact_json()).encode(env) )
        // Err(MockServerError::MockServerFailedToStart) =>
        //     Ok( (atoms::error(), atoms::mock_server_failed_to_start()).encode(env) ),
        // Err(MockServerError::InvalidPactJson) => 
        //     Ok( (atoms::error(), atoms::invalid_pact_json()).encode(env) )
    }
}

#[rustler::nif(name = "mock_server_mismatches")]
fn mock_server_mismatches_call(port:i32) -> NifResult<(Atom, Option<String>)> {
    Ok((atoms::ok(), mock_server_mismatches(port)))
}

#[rustler::nif(name = "mock_server_matched")]
fn mock_server_matched_call(port:i32) -> NifResult<(Atom,bool)> {
    Ok((atoms::ok(), mock_server_matched(port)))
}

#[rustler::nif(name = "write_pact_file")]
fn write_pact_file_call<'a>(env: Env<'a>, port:i32, dir_path:String) -> NifResult<Term<'a>> {
    match write_pact_file(port, Some(dir_path),true) {
        Ok(()) =>
            Ok((atoms::ok()).encode(env)),
        Err(WritePactFileErr::IOError) =>
            Ok( (atoms::error(), atoms::io_error()).encode(env) ),
        Err(WritePactFileErr::NoMockServer) =>
            Ok((atoms::error(), atoms::no_mock_server_running_on_port()).encode(env))
    }
}

#[rustler::nif(name = "cleanup_mock_server")]
fn cleanup_mock_server_call(port:i32) -> NifResult<(Atom,bool)> {
    Ok((atoms::ok(), shutdown_mock_server(port)))
}
