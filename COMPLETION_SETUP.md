# Tab Completion Setup

## Quick Setup with tabtab

1. **Install tabtab globally** (if you don't have it):
   ```bash
   npm install -g tabtab
   ```

2. **Install completion for claudectl**:
   ```bash
   tabtab install --name claudectl --completer claudectl
   ```

3. **Restart your shell** or source your shell config:
   ```bash
   # For bash
   source ~/.bashrc
   
   # For zsh  
   source ~/.zshrc
   ```

4. **Test it**:
   ```bash
   claudectl <TAB>        # Shows: init, new, list, rm, completion
   claudectl r<TAB>       # Completes to: rm
   claudectl rm <TAB>     # Shows your session names
   claudectl rm mysession <TAB>  # Shows: --force, -f
   ```

## Manual Setup (Alternative)

If tabtab doesn't work, you can add this to your shell config:

### Bash (~/.bashrc)
```bash
_claudectl_completion() {
    local cur prev words cword
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    
    local completions=$(COMP_LINE="$COMP_LINE" COMP_POINT="$COMP_POINT" claudectl completion 2>/dev/null)
    
    if [[ -n "$completions" ]]; then
        COMPREPLY=( $(compgen -W "$completions" -- "$cur") )
    fi
}
complete -F _claudectl_completion claudectl
```

### Zsh (~/.zshrc)
```zsh
_claudectl() {
    local completions=$(COMP_LINE="$words" COMP_POINT="${#words}" claudectl completion 2>/dev/null)
    
    if [[ -n "$completions" ]]; then
        _describe 'claudectl' ${(f)completions}
    fi
}
compdef _claudectl claudectl
```

## Troubleshooting

If you see `error: unknown command 'completion'`, make sure you've rebuilt and reinstalled:
```bash
npm run build
npm link  # or npm install -g . if you prefer
```