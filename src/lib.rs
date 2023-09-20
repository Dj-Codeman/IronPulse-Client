mod enviornment;
mod skel;
mod stream;

use enviornment::PROG;
use pretty::warn;
use skel::{Commands, Integrity, Payload, RegistrationId, Requests, Response, Types};
use std::net::TcpStream;
use stream::{connect_stream, phrasing_response, read_from_stream, write_to_stream};
use system::{create_hash, truncate};

pub fn new_channel(channel_name: &str) -> bool {
    // Sending
    // ! Creating the command
    let request: Commands = Commands::CreateChannel(
        truncate(
            &format!("{}-{}", channel_name, create_hash(&channel_name.to_owned())),
            20,
        )
        .to_string(),
    );
    let registration_id: RegistrationId = RegistrationId::Id(create_hash(&PROG.to_string()));
    let integrity_data: Integrity =
        Integrity::Hash(create_hash(&format!("{}{}", registration_id, request)));

    // ! Packaging the command
    let request: Requests = Requests::Command(request, registration_id, integrity_data);
    let mut connection: TcpStream = connect_stream();

    // ! Sending the command
    write_to_stream(&mut connection, request);

    // ! Collecting the results
    let response: Response = phrasing_response(read_from_stream(&mut connection));

    // ! Processing Response
    let response_code: String = match &response {
        Response::Code(data) => data.status.to_string(),
        Response::Data(data) => data.status.to_string(),
    };

    exit_status(response_code, 201)
}

pub fn reg_channel(channel_name: &str) -> bool {
    // Sending
    // ! Creating the command
    let request: Commands = Commands::RegisterChannel(
        truncate(
            &format!("{}-{}", channel_name, create_hash(&channel_name.to_owned())),
            20,
        )
        .to_string(),
    );
    let registration_id: RegistrationId = RegistrationId::Id(create_hash(&PROG.to_string())); // Make this a unique thing
    let integrity_data: Integrity =
        Integrity::Hash(create_hash(&format!("{}{}", registration_id, request)));

    // ! Packaging the command
    let request: Requests = Requests::Command(request, registration_id, integrity_data);
    let mut connection: TcpStream = connect_stream();

    // ! Sending the command
    write_to_stream(&mut connection, request);

    // ! Collecting the results
    let response: Response = phrasing_response(read_from_stream(&mut connection));

    // ! Processing Response
    let response_code: String = match &response {
        Response::Code(data) => data.status.to_string(),
        Response::Data(data) => data.status.to_string(),
    };

    exit_status(response_code, 201)
}

pub fn del_channel(channel_name: &str) -> bool {
    // Sending
    // ! Creating the command
    let request: Commands = Commands::DeleteChannel(
        truncate(
            &format!("{}-{}", channel_name, create_hash(&channel_name.to_owned())),
            20,
        )
        .to_string(),
    );
    let registration_id: RegistrationId = RegistrationId::Id(create_hash(&PROG.to_string()));
    let integrity_data: Integrity =
        Integrity::Hash(create_hash(&format!("{}{}", registration_id, request)));

    // ! Packaging the command
    let request: Requests = Requests::Command(request, registration_id, integrity_data);
    let mut connection: TcpStream = connect_stream();

    // ! Sending the command
    write_to_stream(&mut connection, request);

    // ! Collecting the results
    let response: Response = phrasing_response(read_from_stream(&mut connection));

    // ! Processing Response
    let response_code: String = match &response {
        Response::Code(data) => data.status.to_string(),
        Response::Data(data) => data.status.to_string(),
    };

    exit_status(response_code, 200)
}

pub fn store(channel: &str, data: Payload) -> bool {
    // Sending
    let channel_name: String = truncate(
        &format!("{}-{}", channel, create_hash(&channel.to_owned())),
        20,
    )
    .to_string();
    // ! Creating the command
    let request: Commands = Commands::Store(format!("{}_{}", channel_name, data));
    let registration_id: RegistrationId = RegistrationId::Id(create_hash(&PROG.to_string()));
    let integrity_data: Integrity =
        Integrity::Hash(create_hash(&format!("{}{}", registration_id, request)));

    // ! Packaging the command
    let request: Requests = Requests::Command(request, registration_id, integrity_data);
    let mut connection: TcpStream = connect_stream();
    write_to_stream(&mut connection, request);

    // ! Collecting the results
    let response: Response = phrasing_response(read_from_stream(&mut connection));

    // ! Processing Response
    let response_code: String = match &response {
        Response::Code(data) => data.status.to_string(),
        Response::Data(data) => data.status.to_string(),
    };

    exit_status(response_code, 201)
}

pub fn check(channel: &str) -> Option<String> {
    // Sending
    // ! Creating the command
    let request: Commands = Commands::Check(
        truncate(
            &format!("{}-{}", channel, create_hash(&channel.to_owned())),
            20,
        )
        .to_string(),
    );
    let registration_id: RegistrationId = RegistrationId::Id(create_hash(&PROG.to_string()));
    let integrity_data: Integrity =
        Integrity::Hash(create_hash(&format!("{}{}", registration_id, request)));

    // ! Packaging the command
    let request: Requests = Requests::Command(request, registration_id, integrity_data);
    let mut connection: TcpStream = connect_stream();
    write_to_stream(&mut connection, request);

    // ! Collecting the results
    let response: Response = phrasing_response(read_from_stream(&mut connection));

    // ! Processing Response
    let response_code: String = match &response {
        Response::Code(data) => data.status.to_string(),
        Response::Data(data) => data.status.to_string(),
    };

    if !exit_status(response_code, 202) {
        warn("Unexpected response");
    }


    let data: Option<String> = match &response {
        Response::Data(data) => Some(data.payload.to_string()),
        _ => None,
    };

    return data;
}

#[allow(unused)]
pub fn acknoledged() -> bool {
    // unnessacery
    // Sending
    // ! Creating the command
    let request: Commands = Commands::Store("none".to_string());
    let registration_id: RegistrationId = RegistrationId::Id(create_hash(&PROG.to_string()));
    let integrity_data: Integrity =
        Integrity::Hash(create_hash(&format!("{}{}", registration_id, request)));

    // ! Packaging the command
    let request: Requests = Requests::Command(request, registration_id, integrity_data);
    let mut connection: TcpStream = connect_stream();
    write_to_stream(&mut connection, request);

    // ! Collecting the results
    let response: Response = phrasing_response(read_from_stream(&mut connection));

    // ! Processing Response
    let response_code: String = match &response {
        Response::Code(data) => data.status.to_string(),
        Response::Data(data) => data.status.to_string(),
    };

    exit_status(response_code, 200)
}

#[allow(unused)]
pub fn make_payload(payload_type_raw: Types, payload_data_raw: String) -> Payload {
    let payload_type: Types = payload_type_raw;
    let payload_data: String = hex::encode(payload_data_raw);
    let payload_integrity: Integrity = Integrity::Hash(create_hash(&payload_data)); // creat a hash from the payload

    match payload_type {
        Types::Email => Payload::Email(payload_type, payload_data, payload_integrity),
    }
}

#[allow(unused)]
pub fn phrase_response(data: String) -> String {
    match hex::decode(&data) {
        Ok(data) => {
            let hex_decoded_data: String = match String::from_utf8(data) {
                Ok(string) => string,
                Err(e) => {
                    panic!("{}", e);
                }
            };
            format!("{}", hex_decoded_data)
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}

pub fn exit_status(response_given: String, expected: usize) -> bool {
    match response_given.parse::<usize>() {
        Ok(parsed_value) => parsed_value == expected,
        Err(_) => false, // Handle parsing error case
    }
}
