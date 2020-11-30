// This code was autogenerated with `dbus-codegen-rust -m None --file /home/liushuyu/Development/ciel-rs/dbus-xml/org.freedesktop.machine1-machine.xml`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopDBusPeer {
    fn ping(&self) -> Result<(), dbus::Error>;
    fn get_machine_id(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusPeer for blocking::Proxy<'a, C> {

    fn ping(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
    }

    fn get_machine_id(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "GetMachineId", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait OrgFreedesktopDBusIntrospectable {
    fn introspect(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C> {

    fn introspect(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait OrgFreedesktopDBusProperties {
    fn get(&self, interface: &str, property: &str) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus::Error>;
    fn get_all(&self, interface: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error>;
    fn set(&self, interface: &str, property: &str, value: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusProperties for blocking::Proxy<'a, C> {

    fn get(&self, interface: &str, property: &str) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Get", (interface, property, ))
            .and_then(|r: (arg::Variant<Box<dyn arg::RefArg + 'static>>, )| Ok(r.0, ))
    }

    fn get_all(&self, interface: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "GetAll", (interface, ))
            .and_then(|r: (::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, )| Ok(r.0, ))
    }

    fn set(&self, interface: &str, property: &str, value: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Set", (interface, property, value, ))
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface: String,
    pub changed_properties: ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
            interface: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

pub trait OrgFreedesktopMachine1Machine {
    fn terminate(&self) -> Result<(), dbus::Error>;
    fn kill(&self, who: &str, signal: i32) -> Result<(), dbus::Error>;
    fn get_addresses(&self) -> Result<Vec<(i32, Vec<u8>)>, dbus::Error>;
    fn get_osrelease(&self) -> Result<::std::collections::HashMap<String, String>, dbus::Error>;
    fn get_uidshift(&self) -> Result<u32, dbus::Error>;
    fn open_pty(&self) -> Result<(arg::OwnedFd, String), dbus::Error>;
    fn open_login(&self) -> Result<(arg::OwnedFd, String), dbus::Error>;
    fn open_shell(&self, user: &str, path: &str, args: Vec<&str>, environment: Vec<&str>) -> Result<(arg::OwnedFd, String), dbus::Error>;
    fn bind_mount(&self, source: &str, destination: &str, read_only: bool, mkdir: bool) -> Result<(), dbus::Error>;
    fn copy_from(&self, source: &str, destination: &str) -> Result<(), dbus::Error>;
    fn copy_to(&self, source: &str, destination: &str) -> Result<(), dbus::Error>;
    fn open_root_directory(&self) -> Result<arg::OwnedFd, dbus::Error>;
    fn name(&self) -> Result<String, dbus::Error>;
    fn id(&self) -> Result<Vec<u8>, dbus::Error>;
    fn timestamp(&self) -> Result<u64, dbus::Error>;
    fn timestamp_monotonic(&self) -> Result<u64, dbus::Error>;
    fn service(&self) -> Result<String, dbus::Error>;
    fn unit(&self) -> Result<String, dbus::Error>;
    fn leader(&self) -> Result<u32, dbus::Error>;
    fn class(&self) -> Result<String, dbus::Error>;
    fn root_directory(&self) -> Result<String, dbus::Error>;
    fn network_interfaces(&self) -> Result<Vec<i32>, dbus::Error>;
    fn state(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopMachine1Machine for blocking::Proxy<'a, C> {

    fn terminate(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Machine", "Terminate", ())
    }

    fn kill(&self, who: &str, signal: i32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Machine", "Kill", (who, signal, ))
    }

    fn get_addresses(&self) -> Result<Vec<(i32, Vec<u8>)>, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Machine", "GetAddresses", ())
            .and_then(|r: (Vec<(i32, Vec<u8>)>, )| Ok(r.0, ))
    }

    fn get_osrelease(&self) -> Result<::std::collections::HashMap<String, String>, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Machine", "GetOSRelease", ())
            .and_then(|r: (::std::collections::HashMap<String, String>, )| Ok(r.0, ))
    }

    fn get_uidshift(&self) -> Result<u32, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Machine", "GetUIDShift", ())
            .and_then(|r: (u32, )| Ok(r.0, ))
    }

    fn open_pty(&self) -> Result<(arg::OwnedFd, String), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Machine", "OpenPTY", ())
    }

    fn open_login(&self) -> Result<(arg::OwnedFd, String), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Machine", "OpenLogin", ())
    }

    fn open_shell(&self, user: &str, path: &str, args: Vec<&str>, environment: Vec<&str>) -> Result<(arg::OwnedFd, String), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Machine", "OpenShell", (user, path, args, environment, ))
    }

    fn bind_mount(&self, source: &str, destination: &str, read_only: bool, mkdir: bool) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Machine", "BindMount", (source, destination, read_only, mkdir, ))
    }

    fn copy_from(&self, source: &str, destination: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Machine", "CopyFrom", (source, destination, ))
    }

    fn copy_to(&self, source: &str, destination: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Machine", "CopyTo", (source, destination, ))
    }

    fn open_root_directory(&self) -> Result<arg::OwnedFd, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Machine", "OpenRootDirectory", ())
            .and_then(|r: (arg::OwnedFd, )| Ok(r.0, ))
    }

    fn name(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.machine1.Machine", "Name")
    }

    fn id(&self) -> Result<Vec<u8>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.machine1.Machine", "Id")
    }

    fn timestamp(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.machine1.Machine", "Timestamp")
    }

    fn timestamp_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.machine1.Machine", "TimestampMonotonic")
    }

    fn service(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.machine1.Machine", "Service")
    }

    fn unit(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.machine1.Machine", "Unit")
    }

    fn leader(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.machine1.Machine", "Leader")
    }

    fn class(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.machine1.Machine", "Class")
    }

    fn root_directory(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.machine1.Machine", "RootDirectory")
    }

    fn network_interfaces(&self) -> Result<Vec<i32>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.machine1.Machine", "NetworkInterfaces")
    }

    fn state(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.machine1.Machine", "State")
    }
}