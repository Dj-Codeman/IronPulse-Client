use crate::skel::{Requests, Response, ResponseCode, ResponseData, ResponseStatus};
use std::{
    io::{Read, Write},
    net::TcpStream,
};
use system::create_hash;

pub fn connect_stream() -> TcpStream {
    let server_addr = "10.1.0.14:9518"; // Replace with the server's IP address and port
    let tcp_stream = match TcpStream::connect(server_addr) {
        Ok(tcp) => tcp,
        Err(e) => panic!("{}", e),
    };
    tcp_stream
}

pub fn write_to_stream(tcp_stream: &mut TcpStream, request: Requests) {
    tcp_stream
        .write(format!("{}", request).as_bytes())
        .expect("Failed at writing onto the unix stream");

    tcp_stream
        .shutdown(std::net::Shutdown::Write)
        .expect("Could not shutdown writing on the stream");
}

pub fn read_from_stream(tcp_stream: &mut TcpStream) -> String {
    let mut response = String::new();
    tcp_stream
        .read_to_string(&mut response)
        .expect("Error while reading from sock");
    response
}

fn unpack_payload(data: String) -> (Option<String>, Option<String>) {
    let data_vec: Vec<String> = data.split('/').map(|s| s.to_string()).collect();
    let data: String = data_vec[0].to_string();
    let intg: String = data_vec[1].to_string();

    return (Some(data), Some(intg));
}

// Return a struct with just a status code or a status code and data
pub fn phrasing_response(data: String) -> Response {
    let split_response: Vec<String> = data.split(',').map(|s| s.to_string()).collect();

    let response_code: String = split_response[0].to_string();

    let payload: Option<String> = if split_response.len() > 1 {
        Some(split_response[1].to_owned())
    } else {
        None
    };

    let (data, integrity): (Option<String>, Option<String>) = match payload {
        Some(data) => unpack_payload(data),
        _ => (None, None),
    };

    // Matching to pupulate the structs
    match data {
        Some(d) => {
            // ! The struct with the data
            // Running integrity checking
            let integrity_source: String = integrity.unwrap();
            let integrity_match: String = create_hash(&format!("{}", d));
            let integrity_check = if integrity_match == integrity_source {
                true
            } else {
                false
            };
            // Creating the struct
            let response_data: ResponseData = ResponseData {
                status: match response_code.parse::<usize>().expect("Server is speaking non sense") {
                    200 => ResponseStatus::AckOk,
                    201 => ResponseStatus::AckDr,
                    202 => ResponseStatus::AckDs,
                    400 => ResponseStatus::NoPer,
                    500 => ResponseStatus::NoHnd,
                    520 => ResponseStatus::SecFt,
                    _ => ResponseStatus::NoHnd,
                },
                payload: d,
                integrity: integrity_check,
            };
            return Response::Data(response_data);
        }
        _ => match response_code.parse::<usize>().expect("Server is speaking non sense") {
            200 => Response::Code(ResponseCode {
                status: ResponseStatus::AckOk,
            }),
            201 => Response::Code(ResponseCode {
                status: ResponseStatus::AckDr,
            }),
            202 => Response::Code(ResponseCode {
                status: ResponseStatus::AckDs,
            }),
            400 => Response::Code(ResponseCode {
                status: ResponseStatus::NoPer,
            }),
            500 => Response::Code(ResponseCode {
                status: ResponseStatus::NoHnd,
            }),
            520 => Response::Code(ResponseCode {
                status: ResponseStatus::SecFt,
            }),
            _ => Response::Code(ResponseCode {
                status: ResponseStatus::NoHnd,
            }),
        },
    }
}
