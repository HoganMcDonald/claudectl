# Testing Tab Completion Locally

## Method 1: Manual Shell Setup (Temporary)

Add this to your shell session to test completion:

### For Bash:
```bash
# Add this to test tab completion temporarily
_claudectl_completion() {
    local cur prev words cword
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    
    # Call claudectl with completion environment
    local completions=$(COMP_LINE="$COMP_LINE" COMP_POINT="$COMP_POINT" COMP_WORDS="$COMP_LINE" COMP_CWORD="$COMP_CWORD" claudectl 2>/dev/null)
    
    if [[ -n "$completions" ]]; then
        COMPREPLY=( $(compgen -W "$completions" -- "$cur") )
    fi
}
complete -F _claudectl_completion claudectl
```

### For Zsh:
```zsh
# Add this to test tab completion temporarily
_claudectl() {
    local context state state_descr line
    typeset -A opt_args
    
    local completions=$(COMP_LINE="$words" COMP_POINT="${#words}" claudectl 2>/dev/null)
    
    if [[ -n "$completions" ]]; then
        _describe 'claudectl' ${(f)completions}
    fi
}
compdef _claudectl claudectl
```

## Method 2: Using tabtab (Recommended)

Since we're using tabtab, you can also test by installing completion:

```bash
# Install completion using tabtab
npm install -g tabtab
tabtab install --name claudectl --completer claudectl
```

Then restart your shell and test:
- `claudectl <TAB>` - should show main commands
- `claudectl r<TAB>` - should complete to `rm`
- `claudectl rm <TAB>` - should show your session names
- `claudectl rm session1 <TAB>` - should show `--force` and `-f`

## Testing the Logic

You can also test the completion logic directly using our test script:
```bash
node test-completion.js
```

This shows what completions would be returned for various inputs.