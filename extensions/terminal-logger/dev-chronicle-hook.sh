#!/bin/bash
# DevChronicle Terminal Logger Hook
# This script hooks into bash/zsh to capture terminal commands and send them to DevChronicle

DEVCHRONICLE_ENDPOINT="${DEVCHRONICLE_ENDPOINT:-http://localhost:3030}"
DEVCHRONICLE_ENABLED="${DEVCHRONICLE_ENABLED:-1}"

# Function to send event to DevChronicle
devchronicle_send() {
    if [ "$DEVCHRONICLE_ENABLED" != "1" ]; then
        return
    fi

    local command="$1"
    local exit_code="$2"
    local duration="$3"
    local cwd="$4"

    # Skip empty commands and internal bash commands
    if [ -z "$command" ] || [[ "$command" =~ ^(history|cd|pwd|exit|clear)$ ]]; then
        return
    fi

    # Prepare JSON payload
    local payload=$(cat <<EOF
{
  "source": "terminal",
  "payload": {
    "command": $(echo "$command" | jq -Rs .),
    "exit_code": $exit_code,
    "duration_sec": $duration,
    "cwd": $(echo "$cwd" | jq -Rs .)
  }
}
EOF
)

    # Send to endpoint (non-blocking)
    curl -sS -X POST "$DEVCHRONICLE_ENDPOINT/ingest/terminal" \
        -H "Content-Type: application/json" \
        -d "$payload" > /dev/null 2>&1 &
}

# Detect shell type
if [ -n "$ZSH_VERSION" ]; then
    # Zsh hook
    devchronicle_preexec() {
        DEVCHRONICLE_CMD_START=$(date +%s.%N)
        DEVCHRONICLE_CMD="$1"
        DEVCHRONICLE_CWD="$PWD"
    }

    devchronicle_precmd() {
        if [ -n "$DEVCHRONICLE_CMD_START" ]; then
            local end_time=$(date +%s.%N)
            local duration=$(echo "$end_time - $DEVCHRONICLE_CMD_START" | bc)
            local exit_code=${?}
            
            devchronicle_send "$DEVCHRONICLE_CMD" "$exit_code" "$duration" "$DEVCHRONICLE_CWD"
            
            unset DEVCHRONICLE_CMD_START
            unset DEVCHRONICLE_CMD
            unset DEVCHRONICLE_CWD
        fi
    }

    # Add hooks to zsh
    autoload -Uz add-zsh-hook
    add-zsh-hook preexec devchronicle_preexec
    add-zsh-hook precmd devchronicle_precmd

elif [ -n "$BASH_VERSION" ]; then
    # Bash hook - store command before execution
    devchronicle_preexec() {
        # Skip if this is our own internal command
        if [[ "$BASH_COMMAND" =~ devchronicle_ ]]; then
            return
        fi
        DEVCHRONICLE_CMD_START=$(date +%s.%N)
        DEVCHRONICLE_CMD="$BASH_COMMAND"
        DEVCHRONICLE_CWD="$PWD"
    }

    devchronicle_postexec() {
        # Capture exit code immediately before anything else
        local exit_code=$?
        
        # Skip if no command was tracked
        if [ -z "$DEVCHRONICLE_CMD_START" ]; then
            return
        fi
        
        # Skip if this is our own internal command
        if [[ "$DEVCHRONICLE_CMD" =~ devchronicle_ ]]; then
            unset DEVCHRONICLE_CMD_START
            unset DEVCHRONICLE_CMD
            unset DEVCHRONICLE_CWD
            return
        fi
        
        local end_time=$(date +%s.%N)
        local duration=$(echo "$end_time - $DEVCHRONICLE_CMD_START" | bc)
        
        devchronicle_send "$DEVCHRONICLE_CMD" "$exit_code" "$duration" "$DEVCHRONICLE_CWD"
        
        unset DEVCHRONICLE_CMD_START
        unset DEVCHRONICLE_CMD
        unset DEVCHRONICLE_CWD
    }

    # Hook into bash using DEBUG trap (only for non-internal commands)
    trap 'devchronicle_preexec' DEBUG
    
    # Add to PROMPT_COMMAND to capture after execution
    if [ -z "$PROMPT_COMMAND" ]; then
        PROMPT_COMMAND="devchronicle_postexec"
    else
        PROMPT_COMMAND="devchronicle_postexec; $PROMPT_COMMAND"
    fi
fi

