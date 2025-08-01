# Suppress 32bit-time QA warnings for all v4l-utils packages
INSANE_SKIP:append = " 32bit-time"
INSANE_SKIP:${PN}:append = " 32bit-time"
INSANE_SKIP:libv4l:append = " 32bit-time"
INSANE_SKIP:libv4l-dev:append = " 32bit-time"
INSANE_SKIP:v4l-utils-dev:append = " 32bit-time"