#!/bin/bash

# Set up logging
LOG_FILE="$HOME/Desktop/SSM_splitscreen.log"
echo "=== SSM Splitscreen Minecraft - $(date) ===" > "$LOG_FILE"
exec > >(tee -a "$LOG_FILE") 2>&1  # Redirect stdout and stderr to the log file AND terminal


# Constants
CONFIG_FILE="$HOME/.ssm/config.conf"
CONFIG_DIR="$HOME/.ssm"
REAL_PATH=$(realpath "$0")
SCRIPT_DIR=$(dirname "$REAL_PATH")
ICON_PATH="$SCRIPT_DIR/icon.ico"
LAUNCHED_FROM_STEAM=$SteamDeck
LAUNCHED_FROM_GAMEMODE=$SteamGamepadUI


# Setup config
mkdir -p "$CONFIG_DIR"
if [ ! -f "$CONFIG_FILE" ]; then
    # Default config (empty or with defaults)
    echo "Steam=false" >> "$CONFIG_FILE"
fi
get_config_value() {
    grep "^$1=" "$CONFIG_FILE" | cut -d'=' -f2
}
set_config_value() {
    if grep -q "^$1=" "$CONFIG_FILE"; then
        # Update existing key
        sed -i "s/^$1=.*/$1=$2/" "$CONFIG_FILE"
    else
        # Add new key
        echo "$1=$2" >> "$CONFIG_FILE"
    fi
}


# Check if PolyMC is installed via flatpak
if ! flatpak list | grep -q "org.polymc.PolyMC"; then
    if kdialog --title "PolyMC Not Installed" --yesno "PolyMC is not installed.\n\nWould you like to open Discover to install it now?"; then
        # Open Discover to PolyMC page
        plasma-discover --application appstream:org.polymc.PolyMC >/dev/null 2>&1 &
    fi
    echo "Please install PolyMC from Discover, and restart this script"
    exit 1
else
    POLYMC_DIR=$(flatpak info -l org.polymc.PolyMC)
    POLYMC_FILES_DIR="${POLYMC_DIR}/files"
    echo "PolyMC is installed in: $POLYMC_DIR"

    # PolyMC Flatpak config directory
    POLYMC_CONFIG_DIR="$HOME/.var/app/org.polymc.PolyMC/data/PolyMC"
    PROFILES_DIR="$POLYMC_CONFIG_DIR/profiles"
    INSTANCES_DIR="$POLYMC_CONFIG_DIR/instances"

    # Create config directory if it doesn't exist
    mkdir -p "$PROFILES_DIR"
    mkdir -p "$INSTANCES_DIR"

    # Define player profile names
    PLAYER_PROFILES=("SSM1" "SSM2" "SSM3" "SSM4")
fi


# Check Minecraft version
MINECRAFT_VERSION=$(get_config_value "MinecraftVersion")
if [ -z $MINECRAFT_VERSION ]; then
    # Prompt user for Minecraft version using kdialog
    MINECRAFT_VERSION=$(kdialog --title "Minecraft Version" --inputbox "Enter the Minecraft version to use for all profiles" "1.21.11")

    # Check if the user canceled the dialog
    if [ $? -ne 0 ] || [ -z "$MINECRAFT_VERSION" ]; then
        echo "No version specified. Using default: 1.21.11"
        MINECRAFT_VERSION="1.21.11"
	set_config_value "MinecraftVersion" $MINECRAFT_VERSION
    fi

fi
echo "Using Minecraft version: $MINECRAFT_VERSION"


# Function to create a PolyMC profile
create_polymc_profile() {
    local PROFILE_NAME="$1"
    local PROFILE_DIR="$PROFILES_DIR/$PROFILE_NAME"

    if [ ! -d "$PROFILE_DIR" ]; then
        echo "Creating PolyMC profile: $PROFILE_NAME (Version: $MINECRAFT_VERSION, $PROFILE_DIR)"
        mkdir -p "$PROFILE_DIR"
        cat > "$PROFILE_DIR/profile.json" <<EOF
{
  "gameDir": "$INSTANCES_DIR/$PROFILE_NAME",
  "java": {
    "custom": false,
    "javaPath": "",
    "majorVersion": 21
  },
  "lastVersionId": "$MINECRAFT_VERSION",
  "name": "$PROFILE_NAME",
  "resolution": {
    "fullscreen": false,
    "height": 720,
    "width": 1280
  },
  "type": "vanilla"
}
EOF
        mkdir -p "$INSTANCES_DIR/$PROFILE_NAME"
    else
        echo "PolyMC profile '$PROFILE_NAME' already exists." >> "$LOG_FILE"
    fi
}


# Create all player profiles
for PROFILE in "${PLAYER_PROFILES[@]}"; do
    create_polymc_profile "$PROFILE"
done


# Check if the user has already answered the Steam prompt
if [ "$(get_config_value "SteamAddQuestionAnswered")" != "true" ] && [ "$SteamDeck" != "1" ]; then

    echo "Created a desktop entry"
    DESKTOP_ENTRY="$HOME/Desktop/SSM.desktop"
    if [ ! -f "$DESKTOP_ENTRY" ]; then
        cat > "$DESKTOP_ENTRY" <<EOF
[Desktop Entry]
Name=SSM Splitscreen Minecraft
Exec="$(realpath "$0")"
Icon=$ICON_PATH
Type=Application
Categories=Game;
Terminal=true
StartupWMClass=konsole
EOF
    chmod +x $DESKTOP_ENTRY
    update-desktop-database -q
    fi

    # Prompt the user
    if kdialog --title "Add to Steam instructions?" --yesno "Would you like instructions on how to add this launcher to Steam as a non-Steam game?"; then

        # Open Steam's "Add Non-Steam Game" dialog
        kdialog --title "Instructions on how to add to Steam" --msgbox "A desktop entry for this launcher has been created.

1. Open Steam.
2. Click 'Games' > 'Add a Non-Steam Game to My Library'.
3. Browse to \"SSM Splitscreen Minecraft\" and select it.
4. Click 'Add Selected Programs'.

The launcher will now appear in your Steam library."
	exit 0
    fi
    set_config_value "SteamAddQuestionAnswered" "true"
fi


# Check current resolution
RESOLUTION=$(xrandr --current | grep '*' | awk '{print $1}' | cut -d 'x' -f1,2)
WIDTH=$(echo $RESOLUTION | cut -d 'x' -f1)
HALF_WIDTH=$((WIDTH / 2))
HEIGHT=$(echo $RESOLUTION | cut -d 'x' -f2)
HALF_HEIGHT=$((HEIGHT / 2))

echo "Current resolution: $WIDTH x $HEIGHT"


# Determine number of players based on connected controllers
CONTROLLERS=$(grep -i -A 1 "pad\|joystick" /proc/bus/input/devices | grep -c "N: Name=")
BUILT_IN_ENABLED=$([ $(grep -c "Vendor=28de.*Product=1205" /proc/bus/input/devices) -gt 0 ] && echo 1 || echo 0)
PLAYERS=$CONTROLLERS
OFFSET=0

if [ "$BUILT_IN_ENABLED" -eq 1 ] && [ "$CONTROLLERS" -gt 1 ]; then
    PLAYERS=$((CONTROLLERS - 1)) # Skip built-in, external controllers present
    OFFSET=1
fi

PLAYERS=$((PLAYERS > 4 ? 4 : PLAYERS))
CONTROLLER_ARGS=(
    "--controller=$((0 + OFFSET))"  # js0 or js1
    "--controller=$((1 + OFFSET))"  # js1 or js2
    "--controller=$((2 + OFFSET))"  # js2 or js3
    "--controller=$((3 + OFFSET))"  # js3 or js4
)
echo "Controllers: $CONTROLLERS (built-in enabled: $BUILT_IN_ENABLED), players: $PLAYERS"


# Setup window coordinates and controllers for all players
declare -a WINDOW_ARGS
case $PLAYERS in
    1)
        # Fullscreen
        WINDOW_ARGS=("--fullscreen")
        ;;
    2)
        # Top/Bottom split
        WINDOW_ARGS=(
            "--width=$WIDTH --height=$HALF_HEIGHT --x=0 --y=0"
            "--width=$WIDTH --height=$HALF_HEIGHT --x=0 --y=$HALF_HEIGHT"
        )
        ;;
    3)
        # 3 Players: TOP_LEFT, TOP_RIGHT, BOTTOM_LEFT
        WINDOW_ARGS=(
            "--width=$HALF_WIDTH --height=$HALF_HEIGHT --x=0 --y=0"
            "--width=$HALF_WIDTH --height=$HALF_HEIGHT --x=$HALF_WIDTH --y=0"
            "--width=$HALF_WIDTH --height=$HALF_HEIGHT --x=0 --y=$HALF_HEIGHT"
        )
        ;;
    4)
        # 4 Players: 2x2 Grid
        WINDOW_ARGS=(
            "--width=$HALF_WIDTH --height=$HALF_HEIGHT --x=0 --y=0"
            "--width=$HALF_WIDTH --height=$HALF_HEIGHT --x=$HALF_WIDTH --y=0"
            "--width=$HALF_WIDTH --height=$HALF_HEIGHT --x=0 --y=$HALF_HEIGHT"
            "--width=$HALF_WIDTH --height=$HALF_HEIGHT --x=$HALF_WIDTH --y=$HALF_HEIGHT"
        )
        ;;
    *)
        echo "Currently only 1-4 players are supported. Requested ($PLAYERS), exiting.."
        exit 1
        ;;
esac

# Print arguments for each player (for debugging)
for i in "${!WINDOW_ARGS[@]}"; do
    echo "Player $((i+1)): ${WINDOW_ARGS[$i]} ${CONTROLLER_ARGS[$i]}"
done

exit 0
