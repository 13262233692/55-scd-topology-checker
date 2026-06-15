use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use ahash::AHashMap;
use anyhow::{Context, Result};
use quick_xml::events::attributes::Attribute;
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::model::*;

struct ParseState {
    in_ied: bool,
    in_access_point: bool,
    in_ldevice: bool,
    in_ln: bool,
    in_inputs: bool,
    in_gse_control: bool,
    in_sv_control: bool,
    in_gse: bool,
    in_smv: bool,
    in_subnetwork: bool,
    in_header: bool,
    current_ied_name: Option<String>,
    current_ap_name: Option<String>,
    current_ld_inst: Option<String>,
    current_ln_class: Option<String>,
    current_ln_inst: Option<String>,
    current_gse_name: Option<String>,
    current_sv_name: Option<String>,
    current_subnetwork_name: Option<String>,
}

impl Default for ParseState {
    fn default() -> Self {
        Self {
            in_ied: false,
            in_access_point: false,
            in_ldevice: false,
            in_ln: false,
            in_inputs: false,
            in_gse_control: false,
            in_sv_control: false,
            in_gse: false,
            in_smv: false,
            in_subnetwork: false,
            in_header: false,
            current_ied_name: None,
            current_ap_name: None,
            current_ld_inst: None,
            current_ln_class: None,
            current_ln_inst: None,
            current_gse_name: None,
            current_sv_name: None,
            current_subnetwork_name: None,
        }
    }
}

pub struct SCDParser;

impl SCDParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<SCDModel<'static>> {
        let file = File::open(&path)
            .with_context(|| format!("Failed to open SCD file: {:?}", path.as_ref()))?;
        let reader = BufReader::with_capacity(8 * 1024 * 1024, file);
        Self::parse_from_reader(reader)
    }

    pub fn parse_from_reader<R: std::io::BufRead>(reader: R) -> Result<SCDModel<'static>> {
        let mut xml_reader = Reader::from_reader(reader);

        let mut buf = Vec::with_capacity(64 * 1024);
        let mut state = ParseState::default();
        let mut model = SCDModel::default();

        let mut owned_strings: Vec<String> = Vec::new();
        let decoder = xml_reader.decoder();

        loop {
            match xml_reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    let name_bytes = e.name();
                    let name = std::str::from_utf8(name_bytes.as_ref())?;
                    let attrs: Vec<Attribute> = e.attributes().filter_map(|a| a.ok()).collect();

                    match name {
                        "SCL" => {}
                        "Header" => {
                            state.in_header = true;
                            let mut header = HeaderInfo {
                                id: None,
                                version: None,
                                name_history: Vec::new(),
                            };
                            for attr in &attrs {
                                let key = std::str::from_utf8(attr.key.as_ref())?;
                                let value = attr.decode_and_unescape_value(decoder)?;
                                match key {
                                    "id" => {
                                        let s = value.into_owned();
                                        header.id = Some(unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        });
                                        owned_strings.push(s);
                                    }
                                    "version" => {
                                        let s = value.into_owned();
                                        header.version = Some(unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        });
                                        owned_strings.push(s);
                                    }
                                    _ => {}
                                }
                            }
                            model.header_info = Some(header);
                        }
                        "IED" => {
                            state.in_ied = true;
                            let mut ied_name = "";
                            let mut ied_type = None;
                            let mut manufacturer = None;

                            for attr in &attrs {
                                let key = std::str::from_utf8(attr.key.as_ref())?;
                                let value = attr.decode_and_unescape_value(decoder)?;
                                match key {
                                    "name" => {
                                        let s = value.into_owned();
                                        ied_name = unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        };
                                        owned_strings.push(s);
                                    }
                                    "type" => {
                                        let s = value.into_owned();
                                        ied_type = Some(unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        });
                                        owned_strings.push(s);
                                    }
                                    "manufacturer" => {
                                        let s = value.into_owned();
                                        manufacturer = Some(unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        });
                                        owned_strings.push(s);
                                    }
                                    _ => {}
                                }
                            }
                            state.current_ied_name = Some(ied_name.to_string());

                            let ied = IED {
                                name: ied_name,
                                ied_type,
                                manufacturer,
                                access_points: Vec::new(),
                            };
                            model.ieds.push(ied);
                        }
                        "AccessPoint" if state.in_ied => {
                            state.in_access_point = true;
                            let mut ap_name = "";

                            for attr in &attrs {
                                let key = std::str::from_utf8(attr.key.as_ref())?;
                                if key == "name" {
                                    let value = attr.decode_and_unescape_value(decoder)?;
                                    let s = value.into_owned();
                                    ap_name = unsafe {
                                        std::mem::transmute::<&str, &'static str>(s.as_str())
                                    };
                                    owned_strings.push(s);
                                    break;
                                }
                            }
                            state.current_ap_name = Some(ap_name.to_string());

                            if let Some(ied) = model.ieds.last_mut() {
                                let ap = AccessPoint {
                                    name: ap_name,
                                    ied_name: ied.name,
                                    goose_pubs: Vec::new(),
                                    sv_pubs: Vec::new(),
                                    goose_subs: Vec::new(),
                                    sv_subs: Vec::new(),
                                };
                                ied.access_points.push(ap);
                            }
                        }
                        "LDevice" if state.in_access_point => {
                            state.in_ldevice = true;
                            for attr in &attrs {
                                let key = std::str::from_utf8(attr.key.as_ref())?;
                                if key == "inst" {
                                    let value = attr.decode_and_unescape_value(decoder)?;
                                    state.current_ld_inst = Some(value.into_owned());
                                    break;
                                }
                            }
                        }
                        "LN0" | "LN" if state.in_ldevice => {
                            state.in_ln = true;
                            let mut ln_class = String::new();
                            let mut ln_inst = String::new();
                            for attr in &attrs {
                                let key = std::str::from_utf8(attr.key.as_ref())?;
                                let value = attr.decode_and_unescape_value(decoder)?;
                                match key {
                                    "lnClass" => ln_class = value.into_owned(),
                                    "inst" => ln_inst = value.into_owned(),
                                    _ => {}
                                }
                            }
                            state.current_ln_class = Some(ln_class);
                            state.current_ln_inst = Some(ln_inst);
                        }
                        "Inputs" if state.in_ln => {
                            state.in_inputs = true;
                        }
                        "ExtRef" if state.in_inputs => {
                            let mut ied_name = "";
                            let mut ap_name = "";
                            let mut ld_inst = "";
                            let mut cb_name = "";
                            let mut service_type = ServiceType::GOOSE;

                            for attr in &attrs {
                                let key = std::str::from_utf8(attr.key.as_ref())?;
                                let value = attr.decode_and_unescape_value(decoder)?;
                                match key {
                                    "iedName" => {
                                        let s = value.into_owned();
                                        ied_name = unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        };
                                        owned_strings.push(s);
                                    }
                                    "apRef" => {
                                        let s = value.into_owned();
                                        ap_name = unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        };
                                        owned_strings.push(s);
                                    }
                                    "ldInst" => {
                                        let s = value.into_owned();
                                        ld_inst = unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        };
                                        owned_strings.push(s);
                                    }
                                    "cbName" => {
                                        let s = value.into_owned();
                                        cb_name = unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        };
                                        owned_strings.push(s);
                                    }
                                    "serviceType" => {
                                        service_type = match value.as_ref() {
                                            "GOOSE" => ServiceType::GOOSE,
                                            "SMV" | "SV" => ServiceType::SV,
                                            _ => ServiceType::GOOSE,
                                        };
                                    }
                                    _ => {}
                                }
                            }

                            if !cb_name.is_empty() {
                                let vt = VirtualTerminal {
                                    ied_name,
                                    ap_name,
                                    ld_inst,
                                    cb_name,
                                    service_type,
                                    mac_address: None,
                                    app_id: None,
                                    vlan_id: None,
                                    vlan_priority: None,
                                };

                                if let Some(ied) = model.ieds.last_mut() {
                                    if let Some(ap) = ied.access_points.last_mut() {
                                        match service_type {
                                            ServiceType::GOOSE => ap.goose_subs.push(vt),
                                            ServiceType::SV => ap.sv_subs.push(vt),
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                        "GSEControl" if state.in_ln => {
                            state.in_gse_control = true;
                            for attr in &attrs {
                                let key = std::str::from_utf8(attr.key.as_ref())?;
                                if key == "name" {
                                    let value = attr.decode_and_unescape_value(decoder)?;
                                    state.current_gse_name = Some(value.into_owned());
                                    break;
                                }
                            }
                        }
                        "SampledValueControl" if state.in_ln => {
                            state.in_sv_control = true;
                            for attr in &attrs {
                                let key = std::str::from_utf8(attr.key.as_ref())?;
                                if key == "name" {
                                    let value = attr.decode_and_unescape_value(decoder)?;
                                    state.current_sv_name = Some(value.into_owned());
                                    break;
                                }
                            }
                        }
                        "GSE" | "GOOSE" if state.in_gse_control => {
                            state.in_gse = true;
                            let mut mac_addr = None;
                            let mut app_id = None;
                            let mut vlan_id = None;
                            let mut vlan_priority = None;

                            for attr in &attrs {
                                let key = std::str::from_utf8(attr.key.as_ref())?;
                                let value = attr.decode_and_unescape_value(decoder)?;
                                match key {
                                    "MACAddress" | "macAddress" | "DstAddress" => {
                                        let s = value.into_owned();
                                        mac_addr = Some(unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        });
                                        owned_strings.push(s);
                                    }
                                    "AppID" | "appId" => {
                                        let s = value.into_owned();
                                        app_id = Some(unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        });
                                        owned_strings.push(s);
                                    }
                                    "VLANID" | "vlanId" | "VlanId" => {
                                        let s = value.into_owned();
                                        vlan_id = Some(unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        });
                                        owned_strings.push(s);
                                    }
                                    "VLANPriority" | "vlanPriority" => {
                                        let s = value.into_owned();
                                        vlan_priority = Some(unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        });
                                        owned_strings.push(s);
                                    }
                                    _ => {}
                                }
                            }

                            if let Some(gse_name) = &state.current_gse_name {
                                let cb_name_s = gse_name.clone();
                                let cb_name: &'static str = unsafe {
                                    std::mem::transmute::<&str, &'static str>(cb_name_s.as_str())
                                };
                                owned_strings.push(cb_name_s);

                                let ld_inst_s = state.current_ld_inst.clone().unwrap_or_default();
                                let ld_inst: &'static str = unsafe {
                                    std::mem::transmute::<&str, &'static str>(ld_inst_s.as_str())
                                };
                                owned_strings.push(ld_inst_s);

                                if let Some(ied) = model.ieds.last_mut() {
                                    if let Some(ap) = ied.access_points.last_mut() {
                                        let vt = VirtualTerminal {
                                            ied_name: ied.name,
                                            ap_name: ap.name,
                                            ld_inst,
                                            cb_name,
                                            service_type: ServiceType::GOOSE,
                                            mac_address: mac_addr,
                                            app_id,
                                            vlan_id,
                                            vlan_priority,
                                        };
                                        ap.goose_pubs.push(vt);
                                    }
                                }
                            }
                        }
                        "SMV" | "SampledValue" if state.in_sv_control => {
                            state.in_smv = true;
                            let mut mac_addr = None;
                            let mut app_id = None;
                            let mut vlan_id = None;
                            let mut vlan_priority = None;

                            for attr in &attrs {
                                let key = std::str::from_utf8(attr.key.as_ref())?;
                                let value = attr.decode_and_unescape_value(decoder)?;
                                match key {
                                    "MACAddress" | "macAddress" | "DstAddress" => {
                                        let s = value.into_owned();
                                        mac_addr = Some(unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        });
                                        owned_strings.push(s);
                                    }
                                    "AppID" | "appId" => {
                                        let s = value.into_owned();
                                        app_id = Some(unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        });
                                        owned_strings.push(s);
                                    }
                                    "VLANID" | "vlanId" | "VlanId" => {
                                        let s = value.into_owned();
                                        vlan_id = Some(unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        });
                                        owned_strings.push(s);
                                    }
                                    "VLANPriority" | "vlanPriority" => {
                                        let s = value.into_owned();
                                        vlan_priority = Some(unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        });
                                        owned_strings.push(s);
                                    }
                                    _ => {}
                                }
                            }

                            if let Some(sv_name) = &state.current_sv_name {
                                let cb_name_s = sv_name.clone();
                                let cb_name: &'static str = unsafe {
                                    std::mem::transmute::<&str, &'static str>(cb_name_s.as_str())
                                };
                                owned_strings.push(cb_name_s);

                                let ld_inst_s = state.current_ld_inst.clone().unwrap_or_default();
                                let ld_inst: &'static str = unsafe {
                                    std::mem::transmute::<&str, &'static str>(ld_inst_s.as_str())
                                };
                                owned_strings.push(ld_inst_s);

                                if let Some(ied) = model.ieds.last_mut() {
                                    if let Some(ap) = ied.access_points.last_mut() {
                                        let vt = VirtualTerminal {
                                            ied_name: ied.name,
                                            ap_name: ap.name,
                                            ld_inst,
                                            cb_name,
                                            service_type: ServiceType::SV,
                                            mac_address: mac_addr,
                                            app_id,
                                            vlan_id,
                                            vlan_priority,
                                        };
                                        ap.sv_pubs.push(vt);
                                    }
                                }
                            }
                        }
                        "SubNetwork" => {
                            state.in_subnetwork = true;
                            let mut sub_name = "";
                            let mut sub_type = None;

                            for attr in &attrs {
                                let key = std::str::from_utf8(attr.key.as_ref())?;
                                let value = attr.decode_and_unescape_value(decoder)?;
                                match key {
                                    "name" => {
                                        let s = value.into_owned();
                                        sub_name = unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        };
                                        owned_strings.push(s);
                                    }
                                    "type" => {
                                        let s = value.into_owned();
                                        sub_type = Some(unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        });
                                        owned_strings.push(s);
                                    }
                                    _ => {}
                                }
                            }
                            state.current_subnetwork_name = Some(sub_name.to_string());

                            let sub = SubNetwork {
                                name: sub_name,
                                type_attr: sub_type,
                                access_points: Vec::new(),
                            };
                            model.sub_networks.push(sub);
                        }
                        "ConnectedAP" if state.in_subnetwork => {
                            let mut ied_name = "";
                            let mut ap_name = "";

                            for attr in &attrs {
                                let key = std::str::from_utf8(attr.key.as_ref())?;
                                let value = attr.decode_and_unescape_value(decoder)?;
                                match key {
                                    "iedName" => {
                                        let s = value.into_owned();
                                        ied_name = unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        };
                                        owned_strings.push(s);
                                    }
                                    "apName" => {
                                        let s = value.into_owned();
                                        ap_name = unsafe {
                                            std::mem::transmute::<&str, &'static str>(s.as_str())
                                        };
                                        owned_strings.push(s);
                                    }
                                    _ => {}
                                }
                            }

                            if let Some(sub) = model.sub_networks.last_mut() {
                                sub.access_points.push((ied_name, ap_name));
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::End(e)) => {
                    let name_bytes = e.name();
                    let name = std::str::from_utf8(name_bytes.as_ref())?;
                    match name {
                        "Header" => state.in_header = false,
                        "IED" => {
                            state.in_ied = false;
                            state.current_ied_name = None;
                        }
                        "AccessPoint" => {
                            state.in_access_point = false;
                            state.current_ap_name = None;
                        }
                        "LDevice" => {
                            state.in_ldevice = false;
                            state.current_ld_inst = None;
                        }
                        "LN0" | "LN" => {
                            state.in_ln = false;
                            state.current_ln_class = None;
                            state.current_ln_inst = None;
                        }
                        "Inputs" => state.in_inputs = false,
                        "GSEControl" => {
                            state.in_gse_control = false;
                            state.current_gse_name = None;
                        }
                        "SampledValueControl" => {
                            state.in_sv_control = false;
                            state.current_sv_name = None;
                        }
                        "GSE" | "GOOSE" => state.in_gse = false,
                        "SMV" | "SampledValue" => state.in_smv = false,
                        "SubNetwork" => {
                            state.in_subnetwork = false;
                            state.current_subnetwork_name = None;
                        }
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    eprintln!("XML parse error at position {}: {}", xml_reader.buffer_position(), e);
                    break;
                }
                _ => {}
            }
            buf.clear();
        }

        Self::build_connections(&mut model);

        std::mem::forget(owned_strings);

        Ok(model)
    }

    fn build_connections(model: &mut SCDModel<'_>) {
        let mut goose_pub_map: AHashMap<(&str, &str, &str), Vec<&VirtualTerminal>> = AHashMap::new();
        let mut sv_pub_map: AHashMap<(&str, &str, &str), Vec<&VirtualTerminal>> = AHashMap::new();

        for ied in &model.ieds {
            for ap in &ied.access_points {
                for vt in &ap.goose_pubs {
                    let key = (vt.ied_name, vt.ap_name, vt.cb_name);
                    goose_pub_map.entry(key).or_default().push(vt);
                }
                for vt in &ap.sv_pubs {
                    let key = (vt.ied_name, vt.ap_name, vt.cb_name);
                    sv_pub_map.entry(key).or_default().push(vt);
                }
            }
        }

        let mut goose_sub_map: AHashMap<(&str, &str, &str), Vec<VirtualTerminal>> = AHashMap::new();
        let mut sv_sub_map: AHashMap<(&str, &str, &str), Vec<VirtualTerminal>> = AHashMap::new();

        for ied in &model.ieds {
            for ap in &ied.access_points {
                for vt in &ap.goose_subs {
                    let key = (vt.ied_name, vt.ap_name, vt.cb_name);
                    goose_sub_map.entry(key).or_default().push(vt.clone());
                }
                for vt in &ap.sv_subs {
                    let key = (vt.ied_name, vt.ap_name, vt.cb_name);
                    sv_sub_map.entry(key).or_default().push(vt.clone());
                }
            }
        }

        for (key, pubs) in &goose_pub_map {
            if let Some(subs) = goose_sub_map.get(key) {
                for pub_vt in pubs {
                    model.goose_connections.push(((*pub_vt).clone(), subs.clone()));
                }
            }
        }

        for (key, pubs) in &sv_pub_map {
            if let Some(subs) = sv_sub_map.get(key) {
                for pub_vt in pubs {
                    model.sv_connections.push(((*pub_vt).clone(), subs.clone()));
                }
            }
        }
    }

    pub fn parse_parallel<P: AsRef<Path> + Sync>(paths: &[P]) -> Result<Vec<SCDModel<'static>>> {
        use rayon::prelude::*;
        paths
            .par_iter()
            .map(|p| Self::parse_file(p))
            .collect()
    }
}

impl Default for SCDParser {
    fn default() -> Self {
        Self::new()
    }
}
