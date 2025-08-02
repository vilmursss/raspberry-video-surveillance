SUMMARY = "Minimal Rust web server"
LICENSE = "CLOSED"

SRC_URI = "file://${VOLUME_WORKDIR}/web-server"
S = "${WORKDIR}/${VOLUME_WORKDIR}/web-server"

inherit cargo app-utils-deps

SRC_URI += " \
    crate://crates.io/ascii/1.1.0 \
    crate://crates.io/chunked_transfer/1.5.0 \
    crate://crates.io/httpdate/1.0.3 \
    crate://crates.io/log/0.4.27 \
    crate://crates.io/tiny_http/0.12.0 \
    crate://crates.io/os_pipe/1.2.2 \
"

SRC_URI[ascii-1.1.0.sha256sum] = "d92bec98840b8f03a5ff5413de5293bfcd8bf96467cf5452609f939ec6f5de16"
SRC_URI[chunked_transfer-1.5.0.sha256sum] = "6e4de3bc4ea267985becf712dc6d9eed8b04c953b3fcfb339ebc87acd9804901"
SRC_URI[httpdate-1.0.3.sha256sum] = "df3b46402a9d5adb4c86a0cf463f42e19994e3ee891101b1841f30a545cb49a9"
SRC_URI[log-0.4.27.sha256sum] = "13dc2df351e3202783a1fe0d44375f7295ffb4049267b0f3018346dc122a1d94"
SRC_URI[tiny_http-0.12.0.sha256sum] = "389915df6413a2e74fb181895f933386023c71110878cd0825588928e64cdc82"
SRC_URI[os_pipe-1.2.2.sha256sum] = "db335f4760b14ead6290116f2427bf33a14d4f0617d49f78a246de10c1831224"

# Dependencies for video streaming
RDEPENDS:${PN} += " \
    gstreamer1.0 \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    v4l-utils \
"

do_install() {
    install -d ${D}${bindir}
    install -m 0755 ${B}/target/${RUST_TARGET_SYS}/release/web-server ${D}${bindir}/web-server

    install -d ${D}${datadir}/web-server/html
    cp -r ${S}/html/* ${D}${datadir}/web-server/html/ || true
}

FILES:${PN} += "${datadir}/web-server/html"