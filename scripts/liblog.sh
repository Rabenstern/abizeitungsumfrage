BCK_FG="\033[30m"
BCK_BG="\033[40m"
RED_FG="\033[31m"
RED_BG="\033[41m"
GRN_FG="\033[32m"
GRN_BG="\033[42m"
BRN_FG="\033[33m"
BRN_BG="\033[43m"
BLU_FG="\033[34m"
BLU_BG="\033[44m"
PPL_FG="\033[35m"
PPL_BG="\033[45m"
CYN_FG="\033[36m"
CYN_BG="\033[46m"
GRY_FG="\033[37m"
GRY_BG="\033[47m"

NORM="\033[00m"
BOLD="\033[01m"
ULIN="\033[04m"
BLNK="\033[05m"
RVRS="\033[07m"

err () {
    echo -e "$GRY_FG[$BIN_NAME @ $(date --rfc-3339=ns)] $RED_FG ERROR: $NORM$1"
}

info () {
    echo -e "$GRY_FG[$BIN_NAME @ $(date --rfc-3339=ns)] $CYN_FG INFO: $NORM$1"
}

debug () {
    echo -e "$GRY_FG[$BIN_NAME @ $(date --rfc-3339=ns)] $BLU_FG DEBUG: $NORM$1"
}
