#!/bin/bash

printf '\033[1m ========== MAIDONO : INSTALL ==========\033[0m\n'

systemctl show --no-pager maidono.service | grep LoadState=loaded &>/dev/null
MAIDONO_SERVICE_ALREADY_SET_UP=$?
systemctl show --no-pager maidono.service | grep ActiveState=active &>/dev/null
MAIDONO_SERVICE_ALREADY_RUNNING=$?

MAIDONO_print_step() {
  printf '\033[1;94m %s\033[0m\n' "$1";
}

MAIDONO_print_ok() {
  printf '   OK\n';
}

MAIDONO_print_warning() {
  printf '   \033[7;93m[WARN]\033[27m %s\033[0m\n' "$1";
}

MAIDONO_print_error() {
  printf '   \033[7;91m[ERR]\033[27m %s\033[0m\n' "$1";
}

if [[ "$MAIDONO_SERVICE_ALREADY_RUNNING" == "0" ]]; then
  read -r -p ' A maidono server is already running. Press enter to shut it down and continue installing...'
  MAIDONO_print_step 'Stopping server...'
  if systemctl stop maidono; then
    MAIDONO_print_ok
  else
    MAIDONO_print_error 'Failed to stop server. Find a way to stop it yourself and run the install script again.'
    exit 1
  fi
fi

MAIDONO_print_step 'Installing executables...'
if cp ./bin/maidctl /usr/bin/maidctl && cp ./bin/maidono /usr/bin/maidono; then
  MAIDONO_print_ok
else
  MAIDONO_print_error 'Failed to copy some files to /usr/bin.'
  exit 1
fi
if ! command -v maidctl &>/dev/null
then
    MAIDONO_print_warning 'The command <maidctl> was installed, but does not seem to be on your PATH.'
fi

MAIDONO_print_step 'Installing web app...'
if mkdir -p /usr/share/maidono && rm -r /usr/share/maidono/web && cp -r ./web /usr/share/maidono/; then
  MAIDONO_print_ok
else
  MAIDONO_print_error 'Failed to copy some files to /usr/share/maidono.'
  exit 1
fi

if groups | grep -w maidono &>/dev/null; then
  MAIDONO_print_step 'Creating maidono group...'
  if addgroup --system maidono; then
    MAIDONO_print_ok
  else
    MAIDONO_print_error 'Failed to create the ''maidono'' group.'
    exit 1
  fi
fi
id maidono &>/dev/null
if [[ ! $? ]]; then
  MAIDONO_print_step 'Creating maidono user...'
  if adduser --system --no-create-home --ingroup maidono maidono; then
    MAIDONO_print_ok
  else
    MAIDONO_print_error 'Failed to create the ''maidono'' group.'
    exit 1
  fi
fi

if [[ ! -d /var/maidono ]]; then
  MAIDONO_print_step 'Creating runtime directory...'
  if mkdir /var/maidono && chown /var/maidono maidono; then
    MAIDONO_print_ok
  else
    MAIDONO_print_error 'Failed to create the directory /var/maidono.'
    exit 1
  fi
fi

IFS='' read -r -d '' MAIDONO_CONFIG_FILE <<'EOF'
# the IP address to bind to
# address: "127.0.0.1"

# the port to serve on
# port: 4471

# max level to log (debug / normal / critical)
# log_level: critical

# other Rocket options can be added here, see https://rocket.rs/guide/v0.5/configuration/
EOF

MAIDONO_print_step 'Adding config files...'
if [[ ! -d /etc/maidono ]]; then
  mkdir -p /etc/maidono && chown maidono /etc/maidono
  if [[ ! $? ]]; then
    MAIDONO_print_error 'Failed to create the directory /etc/maidono.'
    exit 1
  fi
fi
if [[ ! -f /etc/maidono/config.toml ]]; then
  echo "$MAIDONO_CONFIG_FILE" > /etc/maidono/config.toml && chown maidono /etc/maidono/config.toml
  if [[ ! $? ]]; then
    MAIDONO_print_error 'Failed to create the file /etc/maidono/config.toml.'
    exit 1
  fi
fi
if [[ ! -f /etc/maidono/enabled ]]; then
  touch /etc/maidono/enabled && chown maidono /etc/maidono/enabled
  if [[ ! $? ]]; then
    MAIDONO_print_error 'Failed to create the file /etc/maidono/enabled.'
    exit 1
  fi
fi
if [[ ! -d /etc/maidono/actions ]]; then
  mkdir -p /etc/maidono/actions && chown maidono /etc/maidono/actions
  if [[ ! $? ]]; then
    MAIDONO_print_error 'Failed to create the directory /etc/maidono/actions.'
    exit 1
  fi
fi
MAIDONO_print_ok

IFS='' read -r -d '' MAIDONO_UNIT_FILE <<'EOF'
[Unit]
Description=Maidono webhook server
After=network.target

[Service]
User=maidono
Group=maidono
WorkingDirectory=/var/maidono
ExecStart=/usr/bin/maidono
Restart=always
TimeoutStopSec=300

[Install]
WantedBy=multi-user.target
EOF

if [[ "$MAIDONO_SERVICE_ALREADY_SET_UP" != "0" ]]; then
  MAIDONO_print_step 'Setting up systemd unit...'
  echo "$MAIDONO_UNIT_FILE" > /etc/systemd/system/maidono.service
  # shellcheck disable=SC2320 # checks the file output, not the echo
  if [[ ! $? ]]; then
    MAIDONO_print_error 'Failed to write file /stc/systemd/system/maidono.service.'
    exit 1
  fi
  if systemctl daemon-reload; then
    MAIDONO_print_ok
  else
    MAIDONO_print_warning 'Failed to reload systemd units. You''ll need to start the server yourself.'
    exit 0 # don't try to start the server
  fi
fi

MAIDONO_print_step '(Re)Starting the server...'
if systemctl start maidono; then
  MAIDONO_print_ok
else
  MAIDONO_print_warning 'Everything was installed, but the server failed to start.'
fi