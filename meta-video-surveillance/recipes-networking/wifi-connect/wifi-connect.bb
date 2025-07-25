SUMMARY = "Wifi connect utility tool"
LICENSE = "CLOSED"

SRC_URI = "file://${VOLUME_WORKDIR}/tools/wifi-connect"
S = "${WORKDIR}/${VOLUME_WORKDIR}/tools/wifi-connect"

inherit cargo app-utils-deps

do_install() {
    install -d ${D}${bindir}
    install -m 0755 ${B}/target/${RUST_TARGET_SYS}/release/wifi-connect ${D}${bindir}/wifi-connect
}
