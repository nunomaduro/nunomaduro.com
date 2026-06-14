#!/bin/sh
# nunomaduro.com/subscribe — follow Nuno across the internet in one command.
#
# Local-first: if a CLI is installed and authenticated, we follow right there.
# Otherwise we open your default browser to the follow page.
#
# Cautious? Read it before you run it:  curl -fsSL nunomaduro.com/subscribe
set -u

GITHUB_USER="nunomaduro"
MASTODON_ACCT="nunomaduro@mastodon.social"

# --- pretty output -----------------------------------------------------------
if [ -t 1 ]; then
  BOLD=$(printf '\033[1m');  DIM=$(printf '\033[2m');    RESET=$(printf '\033[0m')
  GREEN=$(printf '\033[32m'); BLUE=$(printf '\033[34m'); YELLOW=$(printf '\033[33m')
else
  BOLD=; DIM=; RESET=; GREEN=; BLUE=; YELLOW=
fi

ok()   { printf '%s  ✓ %s%s\n' "$GREEN"  "$1" "$RESET"; }
web()  { printf '%s  → %s%s\n' "$BLUE"   "$1" "$RESET"; }
warn() { printf '%s  ! %s%s\n' "$YELLOW" "$1" "$RESET"; }

# --- open a URL in the default browser (cross-platform) ----------------------
open_url() {
  if   command -v open          >/dev/null 2>&1; then open "$1"          >/dev/null 2>&1
  elif command -v xdg-open      >/dev/null 2>&1; then xdg-open "$1"      >/dev/null 2>&1
  elif command -v wslview       >/dev/null 2>&1; then wslview "$1"       >/dev/null 2>&1
  elif command -v powershell.exe >/dev/null 2>&1; then powershell.exe Start "$1" >/dev/null 2>&1
  else return 1
  fi
}

follow_web() { # label url
  if open_url "$2"; then
    web "$1 — opened in your browser"
  else
    warn "$1 — visit $2"
  fi
}

printf '\n%sFollowing Nuno Maduro…%s\n\n' "$BOLD" "$RESET"

# --- GitHub: console-only follow via the gh CLI ------------------------------
# Never opens the browser: if gh is missing or the follow fails, we just skip.
if command -v gh >/dev/null 2>&1 && gh auth status >/dev/null 2>&1 \
   && gh api -X PUT "user/following/$GITHUB_USER" >/dev/null 2>&1; then
  ok "GitHub — now following @$GITHUB_USER (via gh)"
else
  warn "GitHub — skipped (needs an authenticated 'gh' CLI)"
fi

# --- Mastodon: console-only follow via the toot CLI --------------------------
# Never opens the browser: if toot is missing or the follow fails, we just skip.
if command -v toot >/dev/null 2>&1 \
   && toot follow "$MASTODON_ACCT" >/dev/null 2>&1; then
  ok "Mastodon — now following $MASTODON_ACCT (via toot)"
else
  warn "Mastodon — skipped (needs an authenticated 'toot' CLI)"
fi

# --- the most important socials: open the follow / subscribe page ------------
# Ordered least → most important, so the top platform opens LAST (and ends up
# focused on top of the stack). Smaller accounts are intentionally left out to
# keep this to a handful of tabs.
follow_web "TikTok"      "https://tiktok.com/@enunomaduro"
follow_web "Twitch"      "https://twitch.tv/nunomaduro"
follow_web "X / Twitter" "https://twitter.com/intent/follow?screen_name=enunomaduro"
follow_web "Instagram"   "https://instagram.com/enunomaduro"
follow_web "YouTube"     "https://youtube.com/nunomaduro?sub_confirmation=1"

printf '\n%sThank you — see you out there.%s\n\n' "$BOLD" "$RESET"
