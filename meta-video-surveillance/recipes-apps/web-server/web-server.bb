SUMMARY = "Minimal Rust web server"
LICENSE = "CLOSED"

inherit cargo

SRC_URI = "file://${VOLUME_WORKDIR}/web-server"
S = "${WORKDIR}/${VOLUME_WORKDIR}/web-server"

SRC_URI += " \
    crate://crates.io/ascii/1.1.0 \
    crate://crates.io/c_linked_list/1.1.1 \
    crate://crates.io/chunked_transfer/1.5.0 \
    crate://crates.io/gcc/0.3.55 \
    crate://crates.io/get_if_addrs-sys/0.1.1 \
    crate://crates.io/get_if_addrs/0.5.3 \
    crate://crates.io/httpdate/1.0.3 \
    crate://crates.io/libc/0.2.174 \
    crate://crates.io/log/0.4.27 \
    crate://crates.io/tiny_http/0.12.0 \
    crate://crates.io/winapi/0.2.8 \
"

SRC_URI[ascii-1.1.0.sha256sum] = "d92bec98840b8f03a5ff5413de5293bfcd8bf96467cf5452609f939ec6f5de16"
SRC_URI[c_linked_list-1.1.1.sha256sum] = "4964518bd3b4a8190e832886cdc0da9794f12e8e6c1613a9e90ff331c4c8724b"
SRC_URI[chunked_transfer-1.5.0.sha256sum] = "6e4de3bc4ea267985becf712dc6d9eed8b04c953b3fcfb339ebc87acd9804901"
SRC_URI[gcc-0.3.55.sha256sum] = "8f5f3913fa0bfe7ee1fd8248b6b9f42a5af4b9d65ec2dd2c3c26132b950ecfc2"
SRC_URI[get_if_addrs-sys-0.1.1.sha256sum] = "0d04f9fb746cf36b191c00f3ede8bde9c8e64f9f4b05ae2694a9ccf5e3f5ab48"
SRC_URI[get_if_addrs-0.5.3.sha256sum] = "abddb55a898d32925f3148bd281174a68eeb68bbfd9a5938a57b18f506ee4ef7"
SRC_URI[httpdate-1.0.3.sha256sum] = "df3b46402a9d5adb4c86a0cf463f42e19994e3ee891101b1841f30a545cb49a9"
SRC_URI[libc-0.2.174.sha256sum] = "1171693293099992e19cddea4e8b849964e9846f4acee11b3948bcc337be8776"
SRC_URI[log-0.4.27.sha256sum] = "13dc2df351e3202783a1fe0d44375f7295ffb4049267b0f3018346dc122a1d94"
SRC_URI[tiny_http-0.12.0.sha256sum] = "389915df6413a2e74fb181895f933386023c71110878cd0825588928e64cdc82"
SRC_URI[winapi-0.2.8.sha256sum] = "167dc9d6949a9b857f3451275e911c3f44255842c1f7a76f33c55103a909087a"

do_install() {
    install -d ${D}${bindir}
    install -m 0755 ${B}/target/${RUST_TARGET_SYS}/release/web-server ${D}${bindir}/web-server

    install -d ${D}${datadir}/web-server/html
    cp -r ${S}/html/* ${D}${datadir}/web-server/html/ || true
}

FILES:${PN} += "${datadir}/web-server/html"
