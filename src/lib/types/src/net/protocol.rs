use std::str::FromStr;
use ::Error;

const PROTOCOL_TCP: &'static str = "TCP";
const PROTOCOL_UDP: &'static str = "UDP";
const PROTOCOL_UNIX: &'static str = "UNIX";

/// Generic `Protocol` enum allows to store data associated with each protocol type.
///
/// # Examples
///
/// ```
/// use std::net::SocketAddr;
/// use wrust_types::net::Protocol;
///
/// type NetAddress = Protocol<SocketAddr, SocketAddr, String>;
///
/// let tcp_addr: NetAddress = Protocol::Tcp("0.0.0.0:80".parse().unwrap());
/// let unix_addr: NetAddress = Protocol::Unix("/tmp/my.sock".to_string());
///
/// println!("TCP address {:?}", tcp_addr.tcp().unwrap());
/// println!("UNIX address {:?}", unix_addr.unix().unwrap());
/// ```

#[derive(Debug, Clone)]
pub enum Protocol<TcpData = (), UdpData = (), UnixData = ()> {
	/// `Tcp` protocol with associated data of `TcpData` type
	Tcp(TcpData),
	/// `Udp` protocol with associated data of `UdpData` type
	Udp(UdpData),
	/// `Unix` protocol with associated data of `UnixData` type
	Unix(UnixData),
}

impl<TcpData, UdpData, UnixData> Protocol<TcpData, UdpData, UnixData> {
	/// Test if protocol is TCP.
	pub fn is_tcp(&self) -> bool {
		match *self {
			Protocol::Tcp(_) => true,
			_ => false,
		}
	}

	/// Converts from `Protocol<TcpData, UdpData, UnixData>` to `Option<TcpData>`.
	pub fn tcp(self) -> Option<TcpData> {
		match self {
			Protocol::Tcp(data) => Some(data),
			_ => None,
		}
	}

	/// Converts from `Protocol<TcpData, UdpData, UnixData>` to `Option<&TcpData>`.
	pub fn tcp_ref(&self) -> Option<&TcpData> {
		match *self {
			Protocol::Tcp(ref data) => Some(data),
			_ => None,
		}
	}

	/// Execute a closure on holding data if protocol is TCP. Data is passed
	/// to the closure as *immutable* reference.
	///
	/// # Examples
	///
	/// ```
	/// use wrust_types::net::Protocol;
	///
	/// let proto: Protocol<i32, i32, i32> = Protocol::Tcp(0);
	/// proto.tcp_and_then(|data| println!("TCP counter is {}", data));
	/// ```
	pub fn tcp_and_then<Func>(&self, mut func: Func) -> &Self
		where Func : FnMut(&TcpData) {
		if let Protocol::Tcp(ref data) = *self {
			func(data);
		}

		self
	}

	/// Execute a closure on holding data if protocol is TCP. Data is passed
	/// to the closure as *mutable* reference.
	///
	/// # Examples
	///
	/// ```
	/// use wrust_types::net::Protocol;
	///
	/// let mut proto: Protocol<i32, i32, i32> = Protocol::Tcp(0);
	/// for i in 0..10 {
	///	    proto.tcp_and_then_mut(|data| *data = *data + 1);	
	/// }
	///
	/// proto.tcp_and_then(|data| println!("TCP counter is {}", data));
	/// ```
	pub fn tcp_and_then_mut<Func>(&mut self, mut func: Func) -> &mut Self
		where Func : FnMut(&mut TcpData) {
		if let Protocol::Tcp(ref mut data) = *self {
			func(data);
		}

		self
	}

	/// Test if protocol is UDP.
	pub fn is_udp(&self) -> bool {
		match *self {
			Protocol::Udp(_) => true,
			_ => false,
		}
	}

	/// Converts from `Protocol<TcpData, UdpData, UnixData>` to `Option<UdpData>`.
	pub fn udp(self) -> Option<UdpData> {
		match self {
			Protocol::Udp(data) => Some(data),
			_ => None,
		}
	}

	/// Converts from `Protocol<TcpData, UdpData, UnixData>` to `Option<UdpData>`.
	pub fn udp_ref(&self) -> Option<&UdpData> {
		match *self {
			Protocol::Udp(ref data) => Some(data),
			_ => None,
		}
	}

	/// Execute a closure on holding data if protocol is UDP. Data is passed
	/// to the closure by *immutable* reference.
	///
	/// # Examples
	///
	/// ```
	/// use wrust_types::net::Protocol;
	///
	/// let proto: Protocol<i32, i32, i32> = Protocol::Udp(0);
	/// proto.udp_and_then(|data| println!("UDP counter is {}", data));
	/// ```
	pub fn udp_and_then<Func>(&self, mut func: Func) -> &Self
		where Func : FnMut(&UdpData) {
		if let Protocol::Udp(ref data) = *self {
			func(data);
		}

		self
	}

	/// Execute a closure on holding data if protocol is UDP. Data is passed
	/// to the closure by *mutable* reference.
	///
	/// # Examples
	///
	/// ```
	/// use wrust_types::net::Protocol;
	///
	/// let mut proto: Protocol<i32, i32, i32> = Protocol::Udp(0);
	/// for i in 0..10 {
	///	    proto.udp_and_then_mut(|data| *data = *data + 1);	
	/// }
	///
	/// proto.udp_and_then(|data| println!("UDP counter is {}", data));
	/// ```
	pub fn udp_and_then_mut<Func>(&mut self, mut func: Func) -> &mut Self
		where Func : FnMut(&mut UdpData) {
		if let Protocol::Udp(ref mut data) = *self {
			func(data);
		}

		self
	}

	/// Test if protocol is UNIX.
	pub fn is_unix(&self) -> bool {
		match *self {
			Protocol::Unix(_) => true,
			_ => false,
		}
	}

	/// Converts from `Protocol<TcpData, UdpData, UnixData>` to `Option<UnixData>`.
	pub fn unix(self) -> Option<UnixData> {
		match self {
			Protocol::Unix(data) => Some(data),
			_ => None,
		}
	}

	/// Converts from `Protocol<TcpData, UdpData, UnixData>` to `Option<&UnixData>`.
	pub fn unix_ref(&self) -> Option<&UnixData> {
		match *self {
			Protocol::Unix(ref data) => Some(data),
			_ => None,
		}
	}

	/// Execute a closure on holding data if protocol is UNIX. Data is passed
	/// to the closure by *immutable* reference.
	///
	/// # Examples
	///
	/// ```
	/// use wrust_types::net::Protocol;
	///
	/// let proto: Protocol<i32, i32, i32> = Protocol::Unix(0);
	/// proto.unix_and_then(|data| println!("UNIX counter is {}", data));
	/// ```
	pub fn unix_and_then<Func>(&self, mut func: Func) -> &Self
		where Func : FnMut(&UnixData) {
		if let Protocol::Unix(ref data) = *self {
			func(data);
		}

		self
	}

	/// Execute a closure on holding data if protocol is UNIX. Data is passed
	/// to the closure by *mutable* reference.
	///
	/// # Examples
	///
	/// ```
	/// use wrust_types::net::Protocol;
	///
	/// let mut proto: Protocol<i32, i32, i32> = Protocol::Unix(0);
	/// for i in 0..10 {
	///	    proto.unix_and_then_mut(|data| *data = *data + 1);	
	/// }
	///
	/// proto.unix_and_then(|data| println!("UNIX counter is {}", data));
	/// ```
	pub fn unix_and_then_mut<Func>(&mut self, mut func: Func) -> &mut Self
		where Func : FnMut(&mut UnixData) {
		if let Protocol::Unix(ref mut data) = *self {
			func(data);
		}

		self
	}
}

impl FromStr for Protocol {
	type Err = Error;

	/// Parses a string `s` to return a `Protocol`.
	/// On success the function returns *Ok(Protocol)* otherwise it returns *Err(wrust_types::error::Error)*.
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let value = s.to_string();
		match value.to_uppercase().as_ref() {
			PROTOCOL_TCP => Ok(Protocol::Tcp(())),
			PROTOCOL_UDP => Ok(Protocol::Udp(())),
			PROTOCOL_UNIX => Ok(Protocol::Unix(())),
			_ => make_err!(format!("Invalid protocol name {}", value))
		}
	}
}


#[test]
fn test_to_protocol_enum() {
	// Test TCP
	let e: Protocol = "tCp".parse().unwrap();
	assert_eq!(e.is_tcp(), true);
	assert_eq!(e.is_udp(), false);
	assert_eq!(e.is_unix(), false);

	// Test UDP
	let e: Protocol = "udp".parse().unwrap();
	assert_eq!(e.is_tcp(), false);
	assert_eq!(e.is_udp(), true);
	assert_eq!(e.is_unix(), false);

	// Test UNIX
	let e: Protocol = "Unix".parse().unwrap();
	assert_eq!(e.is_tcp(), false);
	assert_eq!(e.is_udp(), false);
	assert_eq!(e.is_unix(), true);

	// Test invalid value
	let e = Protocol::from_str("invalid protocol name");
	assert_eq!(e.is_err(), true);
}

#[test]
fn test_closures() {
	// Test closures on TCP variant
	let mut tcp: Protocol<i32, i32, i32> = Protocol::Tcp(0);
	tcp.tcp_and_then_mut(|data| *data = *data + 1)
		.udp_and_then_mut(|data| *data = *data + 2)
		.unix_and_then_mut(|data| *data = *data + 3);
	assert_eq!(tcp.tcp().unwrap(), 1);

	// Test closures on UDP variant
	let mut udp: Protocol<i32, i32, i32> = Protocol::Udp(0);
	udp.tcp_and_then_mut(|data| *data = *data + 1)
		.udp_and_then_mut(|data| *data = *data + 2)
		.unix_and_then_mut(|data| *data = *data + 3);
	assert_eq!(udp.udp().unwrap(), 2);

	// Test closures on UNIX variant
	let mut unix: Protocol<i32, i32, i32> = Protocol::Unix(0);
	unix.tcp_and_then_mut(|data| *data = *data + 1)
		.udp_and_then_mut(|data| *data = *data + 2)
		.unix_and_then_mut(|data| *data = *data + 3);
	assert_eq!(unix.unix().unwrap(), 3);
}
