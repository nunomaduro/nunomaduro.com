#!/bin/sh
# nunomaduro.com/subscribe — follow Nuno across the internet in one command.
#
# Automatic first: every account that *can* be followed from the terminal is,
# using a CLI you already have installed and signed in. Everything else — most
# platforms publish no usable follow API — falls back to opening that platform's
# own follow page in your browser.
#
#   curl -fsSL nunomaduro.com/subscribe | sh                     follow everywhere
#   curl -fsSL nunomaduro.com/subscribe | sh -s -- --dry-run     show, touch nothing
#   curl -fsSL nunomaduro.com/subscribe | sh -s -- --no-browser  terminal only
#
# Cautious? Read it before you run it:  curl -fsSL nunomaduro.com/subscribe
set -u

GITHUB_USER="nunomaduro"
MASTODON_ACCT="nunomaduro@mastodon.social"

DRY_RUN=0
NO_BROWSER=0

usage() {
  cat <<'USAGE'
Usage: curl -fsSL nunomaduro.com/subscribe | sh -s -- [options]

  -n, --dry-run     print what would happen; follow nothing, open nothing
      --no-browser  only the automatic terminal follows, never open a tab
  -h, --help        show this help

USAGE
}

for arg in "$@"; do
  case "$arg" in
    -n|--dry-run) DRY_RUN=1 ;;
    --no-browser) NO_BROWSER=1 ;;
    -h|--help)    usage; exit 0 ;;
    *)            printf 'subscribe: unknown option %s\n\n' "$arg" >&2; usage >&2; exit 2 ;;
  esac
done

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
note() { printf '%s    %s%s\n' "$DIM"    "$1" "$RESET"; }

# --- open a URL in the default browser (cross-platform) ----------------------
open_url() {
  if   command -v open           >/dev/null 2>&1; then open "$1"           >/dev/null 2>&1
  elif command -v xdg-open       >/dev/null 2>&1; then xdg-open "$1"       >/dev/null 2>&1
  elif command -v wslview        >/dev/null 2>&1; then wslview "$1"        >/dev/null 2>&1
  elif command -v powershell.exe >/dev/null 2>&1; then powershell.exe Start "$1" >/dev/null 2>&1
  else return 1
  fi
}

# Give the browser a beat between tabs: fired back-to-back, some browsers drop
# tabs outright, and the ordering below only holds if each one lands.
stagger() { sleep 0.2 2>/dev/null || :; }

# --- automatic followers -----------------------------------------------------
# Each prints its own success line and returns 0, or sets REASON and returns
# non-zero so follow() can say why and fall back to the browser.
#
# Only GitHub and Mastodon are here, and not for want of trying: X and YouTube
# gate following behind an OAuth app of your own, Twitch dropped follow/unfollow
# from its API entirely, and TikTok, Instagram, Threads, LinkedIn and Kick
# publish no follow endpoint at all. Those can only ever be a browser tab.

github_auto() {
  command -v gh >/dev/null 2>&1  || { REASON="no 'gh' CLI installed"; return 1; }
  gh auth status >/dev/null 2>&1 || { REASON="'gh' is not signed in";  return 1; }

  if gh api "user/following/$GITHUB_USER" >/dev/null 2>&1; then
    ok "GitHub — already following @$GITHUB_USER"
    return 0
  fi
  if gh api -X PUT "user/following/$GITHUB_USER" >/dev/null 2>&1; then
    ok "GitHub — now following @$GITHUB_USER (via gh)"
    return 0
  fi

  if [ "$(gh api user --jq .login 2>/dev/null || :)" = "$GITHUB_USER" ]; then
    ok "GitHub — that's you, nothing to do"
    return 0
  fi

  # GitHub answers a bare 404 — not a 403 — when the token can read but not
  # follow, so a missing scope is by far the likeliest cause. Name the fix.
  REASON="'gh' needs the user scope: gh auth refresh -h github.com -s user"
  return 1
}

mastodon_auto() {
  command -v toot >/dev/null 2>&1 || { REASON="no 'toot' CLI installed"; return 1; }
  toot follow "$MASTODON_ACCT" >/dev/null 2>&1 \
    || { REASON="'toot' is not signed in, or the follow failed"; return 1; }
  ok "Mastodon — now following $MASTODON_ACCT (via toot)"
}

# --- follow <label> <url> [automatic-follower] [cli it needs] ----------------
AUTO=0; OPENED=0; MANUAL=0

follow() {
  _label=$1; _url=$2; _auto=${3:-}; _cli=${4:-}
  REASON=

  if [ "$DRY_RUN" = 1 ]; then
    if [ -n "$_auto" ]; then
      note "$_label — would follow with $_cli if available, else open $_url"
    else
      note "$_label — would open $_url"
    fi
    return 0
  fi

  # 1. automatically, right here in the terminal
  if [ -n "$_auto" ] && "$_auto"; then
    AUTO=$((AUTO + 1))
    return 0
  fi
  if [ -n "$REASON" ]; then
    if [ "$NO_BROWSER" = 1 ]; then
      note "$_label — $REASON"
    else
      note "$_label — $REASON; falling back to the browser"
    fi
  fi

  # 2. the platform's own follow page
  if [ "$NO_BROWSER" = 1 ]; then
    warn "$_label — skipped (--no-browser), follow at $_url"
    MANUAL=$((MANUAL + 1))
  elif open_url "$_url"; then
    web "$_label — opened in your browser"
    OPENED=$((OPENED + 1))
    stagger
  else
    warn "$_label — no browser found, follow at $_url"
    MANUAL=$((MANUAL + 1))
  fi
}

if [ "$DRY_RUN" = 1 ]; then
  printf '\n%sDry run — nothing will be followed or opened.%s\n\n' "$BOLD" "$RESET"
else
  printf '\n%sFollowing Nuno Maduro…%s\n\n' "$BOLD" "$RESET"
fi

# --- the accounts ------------------------------------------------------------
# Ordered least → most followed, so the biggest platform is handled LAST and
# ends up focused on top of the browser's tab stack. Keep this list in sync with
# the socials on nunomaduro.com/about — scripts/update-social.js sorts that list
# by follower count, so mirroring its order keeps the two identical.
follow "TikTok · @nunomaduro_extra"    "https://tiktok.com/@nunomaduro_extra"
follow "YouTube · @nunomaduro_extra2"  "https://youtube.com/@nunomaduro_extra2?sub_confirmation=1"
follow "YouTube · @nunomaduro_vods"    "https://youtube.com/@nunomaduro_vods?sub_confirmation=1"
follow "Instagram · @nunomaduro_extra" "https://instagram.com/nunomaduro_extra"
follow "Kick"                          "https://kick.com/nunomaduro"
follow "YouTube · @nunomaduro_extra"   "https://youtube.com/@nunomaduro_extra?sub_confirmation=1"
follow "Mastodon"                      "https://mastodon.social/@nunomaduro"                        mastodon_auto toot
follow "Threads"                       "https://threads.com/@enunomaduro"
follow "Twitch"                        "https://twitch.tv/nunomaduro"
follow "Instagram"                     "https://instagram.com/enunomaduro"
follow "TikTok"                        "https://tiktok.com/@enunomaduro"
follow "GitHub"                        "https://github.com/nunomaduro"                              github_auto gh
follow "LinkedIn"                      "https://www.linkedin.com/in/nunomaduro/"
follow "YouTube"                       "https://youtube.com/nunomaduro?sub_confirmation=1"
follow "X / Twitter"                   "https://twitter.com/intent/follow?screen_name=enunomaduro"

# --- wrap up -----------------------------------------------------------------
if [ "$DRY_RUN" = 1 ]; then
  printf '\n%sRun it without --dry-run to actually follow.%s\n\n' "$DIM" "$RESET"
  exit 0
fi

printf '\n%s%s followed from the terminal' "$BOLD" "$AUTO"
[ "$OPENED" -gt 0 ] && printf ', %s opened in your browser' "$OPENED"
[ "$MANUAL" -gt 0 ] && printf ', %s left for you' "$MANUAL"
printf '.%s\n' "$RESET"
printf '%sThank you — see you out there.%s\n\n' "$BOLD" "$RESET"
