// This file was generated by gir (https://github.com/gtk-rs/gir @ 1ae7210)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

extern crate gstreamer_sdp_sys;
extern crate shell_words;
extern crate tempdir;
use std::env;
use std::error::Error;
use std::path::Path;
use std::mem::{align_of, size_of};
use std::process::Command;
use std::str;
use gstreamer_sdp_sys::*;

static PACKAGES: &[&str] = &["gstreamer-sdp-1.0"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Compiler, Box<Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Compiler { args })
    }

    pub fn define<'a, V: Into<Option<&'a str>>>(&mut self, var: &str, val: V) {
        let arg = match val.into() {
            None => format!("-D{}", var),
            Some(val) => format!("-D{}={}", var, val),
        };
        self.args.push(arg);
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}",
                               &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let mut cmd = Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}",
                           &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
    /// Number of tests that failed to compile.
    failed_to_compile: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn record_failed_to_compile(&mut self) {
        self.failed += 1;
        self.failed_to_compile += 1;
    }
    fn summary(&self) -> String {
        format!(
            "{} passed; {} failed (compilation errors: {})",
            self.passed,
            self.failed,
            self.failed_to_compile)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let tmpdir = tempdir::TempDir::new("abi").expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!("1",
               get_c_value(tmpdir.path(), &cc, "1").expect("C constant"),
               "failed to obtain correct constant value for 1");

    let mut results : Results = Default::default();
    for (i, &(name, rust_value)) in RUST_CONSTANTS.iter().enumerate() {
        match get_c_value(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            },
            Ok(ref c_value) => {
                if rust_value == c_value {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!("Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                              name, rust_value, c_value);
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("constants ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let tmpdir = tempdir::TempDir::new("abi").expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!(Layout {size: 1, alignment: 1},
               get_c_layout(tmpdir.path(), &cc, "char").expect("C layout"),
               "failed to obtain correct layout for char type");

    let mut results : Results = Default::default();
    for (i, &(name, rust_layout)) in RUST_LAYOUTS.iter().enumerate() {
        match get_c_layout(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            },
            Ok(c_layout) => {
                if rust_layout == c_layout {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!("Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                              name, rust_layout, &c_layout);
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("layout    ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

fn get_c_layout(dir: &Path, cc: &Compiler, name: &str) -> Result<Layout, Box<Error>> {
    let exe = dir.join("layout");
    let mut cc = cc.clone();
    cc.define("ABI_TYPE_NAME", name);
    cc.compile(Path::new("tests/layout.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}",
                           &abi_cmd, &output).into());
    }

    let stdout = str::from_utf8(&output.stdout)?;
    let mut words = stdout.trim().split_whitespace();
    let size = words.next().unwrap().parse().unwrap();
    let alignment = words.next().unwrap().parse().unwrap();
    Ok(Layout {size, alignment})
}

fn get_c_value(dir: &Path, cc: &Compiler, name: &str) -> Result<String, Box<Error>> {
    let exe = dir.join("constant");
    let mut cc = cc.clone();
    cc.define("ABI_CONSTANT_NAME", name);
    cc.compile(Path::new("tests/constant.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}",
                           &abi_cmd, &output).into());
    }

    let output = str::from_utf8(&output.stdout)?.trim();
    if !output.starts_with("###gir test###") ||
       !output.ends_with("###gir test###") {
        return Err(format!("command {:?} return invalid output, {:?}",
                           &abi_cmd, &output).into());
    }

    Ok(String::from(&output[14..(output.len() - 14)]))
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    ("GstMIKEYCacheType", Layout {size: size_of::<GstMIKEYCacheType>(), alignment: align_of::<GstMIKEYCacheType>()}),
    ("GstMIKEYEncAlg", Layout {size: size_of::<GstMIKEYEncAlg>(), alignment: align_of::<GstMIKEYEncAlg>()}),
    ("GstMIKEYKVType", Layout {size: size_of::<GstMIKEYKVType>(), alignment: align_of::<GstMIKEYKVType>()}),
    ("GstMIKEYKeyDataType", Layout {size: size_of::<GstMIKEYKeyDataType>(), alignment: align_of::<GstMIKEYKeyDataType>()}),
    ("GstMIKEYMacAlg", Layout {size: size_of::<GstMIKEYMacAlg>(), alignment: align_of::<GstMIKEYMacAlg>()}),
    ("GstMIKEYMapSRTP", Layout {size: size_of::<GstMIKEYMapSRTP>(), alignment: align_of::<GstMIKEYMapSRTP>()}),
    ("GstMIKEYMapType", Layout {size: size_of::<GstMIKEYMapType>(), alignment: align_of::<GstMIKEYMapType>()}),
    ("GstMIKEYMessage", Layout {size: size_of::<GstMIKEYMessage>(), alignment: align_of::<GstMIKEYMessage>()}),
    ("GstMIKEYPRFFunc", Layout {size: size_of::<GstMIKEYPRFFunc>(), alignment: align_of::<GstMIKEYPRFFunc>()}),
    ("GstMIKEYPayload", Layout {size: size_of::<GstMIKEYPayload>(), alignment: align_of::<GstMIKEYPayload>()}),
    ("GstMIKEYPayloadKEMAC", Layout {size: size_of::<GstMIKEYPayloadKEMAC>(), alignment: align_of::<GstMIKEYPayloadKEMAC>()}),
    ("GstMIKEYPayloadKeyData", Layout {size: size_of::<GstMIKEYPayloadKeyData>(), alignment: align_of::<GstMIKEYPayloadKeyData>()}),
    ("GstMIKEYPayloadPKE", Layout {size: size_of::<GstMIKEYPayloadPKE>(), alignment: align_of::<GstMIKEYPayloadPKE>()}),
    ("GstMIKEYPayloadRAND", Layout {size: size_of::<GstMIKEYPayloadRAND>(), alignment: align_of::<GstMIKEYPayloadRAND>()}),
    ("GstMIKEYPayloadSP", Layout {size: size_of::<GstMIKEYPayloadSP>(), alignment: align_of::<GstMIKEYPayloadSP>()}),
    ("GstMIKEYPayloadSPParam", Layout {size: size_of::<GstMIKEYPayloadSPParam>(), alignment: align_of::<GstMIKEYPayloadSPParam>()}),
    ("GstMIKEYPayloadT", Layout {size: size_of::<GstMIKEYPayloadT>(), alignment: align_of::<GstMIKEYPayloadT>()}),
    ("GstMIKEYPayloadType", Layout {size: size_of::<GstMIKEYPayloadType>(), alignment: align_of::<GstMIKEYPayloadType>()}),
    ("GstMIKEYSecProto", Layout {size: size_of::<GstMIKEYSecProto>(), alignment: align_of::<GstMIKEYSecProto>()}),
    ("GstMIKEYSecSRTP", Layout {size: size_of::<GstMIKEYSecSRTP>(), alignment: align_of::<GstMIKEYSecSRTP>()}),
    ("GstMIKEYTSType", Layout {size: size_of::<GstMIKEYTSType>(), alignment: align_of::<GstMIKEYTSType>()}),
    ("GstMIKEYType", Layout {size: size_of::<GstMIKEYType>(), alignment: align_of::<GstMIKEYType>()}),
    ("GstSDPAttribute", Layout {size: size_of::<GstSDPAttribute>(), alignment: align_of::<GstSDPAttribute>()}),
    ("GstSDPBandwidth", Layout {size: size_of::<GstSDPBandwidth>(), alignment: align_of::<GstSDPBandwidth>()}),
    ("GstSDPConnection", Layout {size: size_of::<GstSDPConnection>(), alignment: align_of::<GstSDPConnection>()}),
    ("GstSDPKey", Layout {size: size_of::<GstSDPKey>(), alignment: align_of::<GstSDPKey>()}),
    ("GstSDPMedia", Layout {size: size_of::<GstSDPMedia>(), alignment: align_of::<GstSDPMedia>()}),
    ("GstSDPMessage", Layout {size: size_of::<GstSDPMessage>(), alignment: align_of::<GstSDPMessage>()}),
    ("GstSDPOrigin", Layout {size: size_of::<GstSDPOrigin>(), alignment: align_of::<GstSDPOrigin>()}),
    ("GstSDPResult", Layout {size: size_of::<GstSDPResult>(), alignment: align_of::<GstSDPResult>()}),
    ("GstSDPTime", Layout {size: size_of::<GstSDPTime>(), alignment: align_of::<GstSDPTime>()}),
    ("GstSDPZone", Layout {size: size_of::<GstSDPZone>(), alignment: align_of::<GstSDPZone>()}),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) GST_MIKEY_CACHE_ALWAYS", "1"),
    ("(gint) GST_MIKEY_CACHE_FOR_CSB", "2"),
    ("(gint) GST_MIKEY_CACHE_NONE", "0"),
    ("(gint) GST_MIKEY_ENC_AES_CM_128", "1"),
    ("(gint) GST_MIKEY_ENC_AES_KW_128", "2"),
    ("(gint) GST_MIKEY_ENC_NULL", "0"),
    ("(gint) GST_MIKEY_KD_TEK", "2"),
    ("(gint) GST_MIKEY_KD_TGK", "0"),
    ("(gint) GST_MIKEY_KV_INTERVAL", "2"),
    ("(gint) GST_MIKEY_KV_NULL", "0"),
    ("(gint) GST_MIKEY_KV_SPI", "1"),
    ("(gint) GST_MIKEY_MAC_HMAC_SHA_1_160", "1"),
    ("(gint) GST_MIKEY_MAC_NULL", "0"),
    ("(gint) GST_MIKEY_MAP_TYPE_SRTP", "0"),
    ("(gint) GST_MIKEY_PRF_MIKEY_1", "0"),
    ("(gint) GST_MIKEY_PT_CERT", "7"),
    ("(gint) GST_MIKEY_PT_CHASH", "8"),
    ("(gint) GST_MIKEY_PT_DH", "3"),
    ("(gint) GST_MIKEY_PT_ERR", "12"),
    ("(gint) GST_MIKEY_PT_GEN_EXT", "21"),
    ("(gint) GST_MIKEY_PT_ID", "6"),
    ("(gint) GST_MIKEY_PT_KEMAC", "1"),
    ("(gint) GST_MIKEY_PT_KEY_DATA", "20"),
    ("(gint) GST_MIKEY_PT_LAST", "0"),
    ("(gint) GST_MIKEY_PT_PKE", "2"),
    ("(gint) GST_MIKEY_PT_RAND", "11"),
    ("(gint) GST_MIKEY_PT_SIGN", "4"),
    ("(gint) GST_MIKEY_PT_SP", "10"),
    ("(gint) GST_MIKEY_PT_T", "5"),
    ("(gint) GST_MIKEY_PT_V", "9"),
    ("(gint) GST_MIKEY_SEC_PROTO_SRTP", "0"),
    ("(gint) GST_MIKEY_SP_SRTP_AUTH_ALG", "2"),
    ("(gint) GST_MIKEY_SP_SRTP_AUTH_KEY_LEN", "3"),
    ("(gint) GST_MIKEY_SP_SRTP_AUTH_TAG_LEN", "11"),
    ("(gint) GST_MIKEY_SP_SRTP_ENC_ALG", "0"),
    ("(gint) GST_MIKEY_SP_SRTP_ENC_KEY_LEN", "1"),
    ("(gint) GST_MIKEY_SP_SRTP_FEC_ORDER", "9"),
    ("(gint) GST_MIKEY_SP_SRTP_KEY_DERIV_RATE", "6"),
    ("(gint) GST_MIKEY_SP_SRTP_PRF", "5"),
    ("(gint) GST_MIKEY_SP_SRTP_SALT_KEY_LEN", "4"),
    ("(gint) GST_MIKEY_SP_SRTP_SRTCP_ENC", "8"),
    ("(gint) GST_MIKEY_SP_SRTP_SRTP_AUTH", "10"),
    ("(gint) GST_MIKEY_SP_SRTP_SRTP_ENC", "7"),
    ("(gint) GST_MIKEY_SP_SRTP_SRTP_PREFIX_LEN", "12"),
    ("(gint) GST_MIKEY_TS_TYPE_COUNTER", "2"),
    ("(gint) GST_MIKEY_TS_TYPE_NTP", "1"),
    ("(gint) GST_MIKEY_TS_TYPE_NTP_UTC", "0"),
    ("(gint) GST_MIKEY_TYPE_DH_INIT", "4"),
    ("(gint) GST_MIKEY_TYPE_DH_RESP", "5"),
    ("(gint) GST_MIKEY_TYPE_ERROR", "6"),
    ("(gint) GST_MIKEY_TYPE_INVALID", "-1"),
    ("(gint) GST_MIKEY_TYPE_PK_INIT", "2"),
    ("(gint) GST_MIKEY_TYPE_PK_VERIFY", "3"),
    ("(gint) GST_MIKEY_TYPE_PSK_INIT", "0"),
    ("(gint) GST_MIKEY_TYPE_PSK_VERIFY", "1"),
    ("GST_MIKEY_VERSION", "1"),
    ("GST_SDP_BWTYPE_AS", "AS"),
    ("GST_SDP_BWTYPE_CT", "CT"),
    ("GST_SDP_BWTYPE_EXT_PREFIX", "X-"),
    ("GST_SDP_BWTYPE_RR", "RR"),
    ("GST_SDP_BWTYPE_RS", "RS"),
    ("GST_SDP_BWTYPE_TIAS", "TIAS"),
    ("(gint) GST_SDP_EINVAL", "-1"),
    ("(gint) GST_SDP_OK", "0"),
];


