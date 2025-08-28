
use builtin;
use str;

set edit:completion:arg-completer[claudectl] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'claudectl'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'claudectl'= {
            cand --debug 'Enable debug logging output'
            cand -h 'Print help'
            cand --help 'Print help'
            cand init 'init'
            cand task 'task'
            cand list 'list'
            cand rm 'rm'
            cand completions 'completions'
            cand repair 'repair'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'claudectl;init'= {
            cand --debug 'Enable debug logging output'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'claudectl;task'= {
            cand --debug 'Enable debug logging output'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'claudectl;list'= {
            cand --debug 'Enable debug logging output'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'claudectl;rm'= {
            cand --debug 'Enable debug logging output'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'claudectl;completions'= {
            cand --verify 'verify'
            cand --debug 'Enable debug logging output'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'claudectl;repair'= {
            cand --force 'force'
            cand --debug 'Enable debug logging output'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'claudectl;help'= {
            cand init 'init'
            cand task 'task'
            cand list 'list'
            cand rm 'rm'
            cand completions 'completions'
            cand repair 'repair'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'claudectl;help;init'= {
        }
        &'claudectl;help;task'= {
        }
        &'claudectl;help;list'= {
        }
        &'claudectl;help;rm'= {
        }
        &'claudectl;help;completions'= {
        }
        &'claudectl;help;repair'= {
        }
        &'claudectl;help;help'= {
        }
    ]
    $completions[$command]
}
