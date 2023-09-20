use serde::{Deserialize, Serialize};
use std::fmt;

// THIS WILL BE SOMEWHERE ELSE
pub enum Requests {
    // command( command , RegisterID, Hash)
    Command(Commands, RegistrationId, Integrity),

    // Retriving data from the server
    // command( Check, 2yU2ttnm0g23ht024gt42hyg0t2, 3cedee34d55)
    // recive(ack,202) acknolodged standby for data
    // command(ack, 2yU2ttnm0g23ht024gt42hyg0t2, 3cedee34d55)
    // recive(payload(data, integrity),200)
    // command(ack, 2yU2ttnm0g23ht024gt42hyg0t2, 3cedee34d55)
}

pub enum Response {
    Code(ResponseCode),
    Data(ResponseData),
}

pub struct ResponseData {
    pub status: ResponseStatus,
    pub payload: String,
    pub integrity: bool,
}

#[derive(Debug)]
pub struct ResponseCode {
    pub status: ResponseStatus,
}

#[derive(Debug)]
pub enum ResponseStatus {
    AckOk, // Data Recived OK
    AckDr, // Data Recived
    AckDs, // Data Data sent
    NoHnd, // Resource not foure or err occoured
    NoPer, // Invalid permission or registration
    SecFt, // Integrity check failed
}

pub enum Commands {
    CreateChannel(String),   // Created a channel with the given name
    RegisterChannel(String), // Registers ID to channel
    DeleteChannel(String),   // Deletes a channel and permissions
    Store(String),           // Given the associated id stores a messege in a channel
    Check(String),           // Given ID retrives stored data in json format
    Ack(String),             // Allows data to be deleted
}

pub enum Payload {
    Email(Types, String, Integrity),
}

// ? Descriptors
#[derive(Debug)]
pub enum Types {
    Email,
}

pub enum Integrity {
    Hash(String),
}

// ? Diffrent types of messages
#[derive(Serialize, Deserialize, Debug)]
pub struct Email {
    pub to: String,
    pub subject: String,
    pub body: String,
}
pub enum RegistrationId {
    Id(String),
}

// ? Implementations for enums
impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Response::Code(data) => write!(f, "{}", data),
            Response::Data(data) => write!(f, "{}", data),
        }
    }
}

impl fmt::Display for ResponseData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.status, self.payload, self.integrity)
    }
}

impl fmt::Display for ResponseCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.status)
    }
}

impl fmt::Display for ResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResponseStatus::AckOk => write!(f, "200"), // ok
            ResponseStatus::AckDr => write!(f, "201"), // ack recived data
            ResponseStatus::AckDs => write!(f, "202"), // ack data in response
            ResponseStatus::NoPer => write!(f, "400"), // client messed up
            ResponseStatus::NoHnd => write!(f, "500"), // i messed up
            ResponseStatus::SecFt => write!(f, "520"), // i refuse, security fault
                                                        // ResponseStatus::AckOk => write!(f, "Data Received OK"),
                                                        // ResponseStatus::AckDr => write!(f, "Data Received"),
                                                        // ResponseStatus::AckDs => write!(f, "Data Sent"),
                                                        // ResponseStatus::NoHnd => write!(f, "Resource not found or error occurred"),
                                                        // ResponseStatus::NoPer => write!(f, "Invalid permission or registration"),
                                                        // ResponseStatus::SecFt => write!(f, "Integrity check failed"),
        }
    }
}

impl fmt::Display for Payload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Payload::Email(types, data, sec) => write!(f, "{}_{}_{}", types, data, sec),
        }
    }
}

impl fmt::Display for Integrity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Integrity::Hash(data) => write!(f, "{}", data),
        }
    }
}

impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Types::Email => write!(f, "Email"),
        }
    }
}

impl fmt::Display for RegistrationId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RegistrationId::Id(data) => write!(f, "{}", data),
        }
    }
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Commands::CreateChannel(data) => write!(f, "CreateChannel/{}", data),
            Commands::RegisterChannel(data) => write!(f, "RegisterChannel/{}", data),
            Commands::DeleteChannel(data) => write!(f, "DeleteChannel/{}", data),
            Commands::Store(data) => write!(f, "Store/{}", data),
            Commands::Check(data) => write!(f, "Check/{}", data),
            Commands::Ack(data) => write!(f, "Ack/{}", data),
        }
    }
}

impl fmt::Display for Requests {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Requests::Command(x, y, z) => {
                write!(f, "{},{},{}", x, y, z)
            }
        }
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "To: {}\nSubject: {}\n\n{}",
            self.to, self.subject, self.body
        )
    }
}
