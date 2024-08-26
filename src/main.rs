use std::io::Write;
#[tokio::main]
async fn main() {
    let mut connection = Connection::new("COM1", 125).await.unwrap();
    connection.set_valve_mode(ValveMode::Off).await.unwrap();
}

struct Connection {

}
impl Connection {
    async fn new(serial_port: &str, device_address: u8) -> Result<Self, std::io::Error> {
        todo!()
    }

    async fn set_valve_mode(&mut self, mode: ValveMode) -> Result<(), std::io::Error> {
        todo!()
    }
}

async fn send_message<W: Write>(device: &mut W, device_address: u8, command: Command) -> Result<(), std::io::Error> {
    if device_address > 253 {
        panic!("Device address must be 253 or lower");
    }
    // Start of message characters
    device.write_all(b"@@@")?;

    // Write address
    write!(device, "{device_address:03}")?;

    // TODO Command oder Request schreiben
    device.write_all(&command.to_bytes())?;

    device.write_all(b";\r\n")?;

    // TODO Checksum schreiben
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub enum Gas {
    Acetone,
    Air,
    Nitrogen,
    Toluene,
}

impl Gas {
    fn symbol(&self) -> &'static str {
        match self {
            Self::Acetone => "C3H6O",
            Self::Air => "Air",
            Self::Nitrogen => "N2",
            Self::Toluene => "C7H8",
        }
    }
    fn code_number(&self) -> u8 {
        match self {
            Self::Acetone => 184,
            Self::Air => 8,
            Self::Nitrogen => 13,
            Self::Toluene => 181,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    ActivateProgrammedGas(Gas),
    SetFlowUnits(f32),
    SetValveMode(ValveMode),
}

impl Command {
    fn to_bytes(&self) -> Vec<u8> {
        let mut output = Vec::new();
        match self {
            Self::ActivateProgrammedGas(gas) => {
                output.write_all(b"PG!").unwrap();
                output.write_all(gas.symbol().as_bytes()).unwrap();
            },
            Self::SetFlowUnits(flow_rate) => {
                if *flow_rate < 0.0 || *flow_rate > 500_000.0 {
                    panic!("Invalid flow rate parameter")
                }
                write!(output, "SX!{flow_rate:.1}").unwrap();
            },
            Self::SetValveMode(mode) => {
                write!(output, "VO!{}", mode.message()).unwrap();
            }
        }
        output
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ValveMode {
    Normal,
    Purge,
    Off
}
impl ValveMode {
    fn message(&self) -> &'static str {
        match self {
            Self::Normal => "NORMAL",
            Self::Purge => "PURGE",
            Self::Off => "FLOW_OFF",
        }
    }
}
