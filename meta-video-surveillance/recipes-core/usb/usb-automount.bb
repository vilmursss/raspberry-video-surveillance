SUMMARY = "Automatic USB mount helper (udev rule and script)"
DESCRIPTION = "Installs a udev rule and script to auto-mount USB drives"
LICENSE = "CLOSED"

SRC_URI += "file://usb-automount.sh \
            file://99-usb-automount.rules"

do_install() {
    install -d ${D}${sysconfdir}/udev/rules.d
    install -m 0644 ${WORKDIR}/99-usb-automount.rules ${D}${sysconfdir}/udev/rules.d/

    install -d ${D}/lib/udev/scripts
    install -m 0755 ${WORKDIR}/usb-automount.sh ${D}/lib/udev/scripts/
}
