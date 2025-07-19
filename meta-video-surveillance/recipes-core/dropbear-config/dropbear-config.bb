DESCRIPTION = "Recipe for enabling SSH on boot"
LICENSE = "CLOSED"
PR = "r0"

inherit update-rc.d

SRC_URI = "file://dropbear"

do_install() {
    install -d ${D}${sysconfdir}/init.d/
    install -m 0755 ${WORKDIR}/dropbear ${D}${sysconfdir}/init.d/
    
    install -d {D}/etc/dropbear/

    if [ ! -f ${D}/etc/dropbear/dropbear_rsa_host_key ]; then
        dropbearkey -t rsa -f ${D}/etc/dropbear/dropbear_rsa_host_key
    fi
    if [ ! -f ${D}/etc/dropbear/dropbear_dss_host_key ]; then
        dropbearkey -t dss -f ${D}/etc/dropbear/dropbear_dss_host_key
    fi
}

FILES:${PN} += "/etc/dropbear"

INITSCRIPT_PACKAGES = "${PN}"
INITSCRIPT_NAME:${PN} = "dropbear"

INITSCRIPT_PARAMS = "defaults"
