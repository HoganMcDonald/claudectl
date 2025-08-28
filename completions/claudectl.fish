# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_claudectl_global_optspecs
	string join \n debug h/help
end

function __fish_claudectl_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_claudectl_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_claudectl_using_subcommand
	set -l cmd (__fish_claudectl_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c claudectl -n "__fish_claudectl_needs_command" -l debug -d 'Enable debug logging output'
complete -c claudectl -n "__fish_claudectl_needs_command" -s h -l help -d 'Print help'
complete -c claudectl -n "__fish_claudectl_needs_command" -f -a "init"
complete -c claudectl -n "__fish_claudectl_needs_command" -f -a "task"
complete -c claudectl -n "__fish_claudectl_needs_command" -f -a "list"
complete -c claudectl -n "__fish_claudectl_needs_command" -f -a "rm"
complete -c claudectl -n "__fish_claudectl_needs_command" -f -a "completions"
complete -c claudectl -n "__fish_claudectl_needs_command" -f -a "repair"
complete -c claudectl -n "__fish_claudectl_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c claudectl -n "__fish_claudectl_using_subcommand init" -l debug -d 'Enable debug logging output'
complete -c claudectl -n "__fish_claudectl_using_subcommand init" -s h -l help -d 'Print help'
complete -c claudectl -n "__fish_claudectl_using_subcommand task" -l debug -d 'Enable debug logging output'
complete -c claudectl -n "__fish_claudectl_using_subcommand task" -s h -l help -d 'Print help'
complete -c claudectl -n "__fish_claudectl_using_subcommand list" -l debug -d 'Enable debug logging output'
complete -c claudectl -n "__fish_claudectl_using_subcommand list" -s h -l help -d 'Print help'
complete -c claudectl -n "__fish_claudectl_using_subcommand rm" -l debug -d 'Enable debug logging output'
complete -c claudectl -n "__fish_claudectl_using_subcommand rm" -s h -l help -d 'Print help'
complete -c claudectl -n "__fish_claudectl_using_subcommand completions" -l verify
complete -c claudectl -n "__fish_claudectl_using_subcommand completions" -l debug -d 'Enable debug logging output'
complete -c claudectl -n "__fish_claudectl_using_subcommand completions" -s h -l help -d 'Print help'
complete -c claudectl -n "__fish_claudectl_using_subcommand repair" -l force
complete -c claudectl -n "__fish_claudectl_using_subcommand repair" -l debug -d 'Enable debug logging output'
complete -c claudectl -n "__fish_claudectl_using_subcommand repair" -s h -l help -d 'Print help'
complete -c claudectl -n "__fish_claudectl_using_subcommand help; and not __fish_seen_subcommand_from init task list rm completions repair help" -f -a "init"
complete -c claudectl -n "__fish_claudectl_using_subcommand help; and not __fish_seen_subcommand_from init task list rm completions repair help" -f -a "task"
complete -c claudectl -n "__fish_claudectl_using_subcommand help; and not __fish_seen_subcommand_from init task list rm completions repair help" -f -a "list"
complete -c claudectl -n "__fish_claudectl_using_subcommand help; and not __fish_seen_subcommand_from init task list rm completions repair help" -f -a "rm"
complete -c claudectl -n "__fish_claudectl_using_subcommand help; and not __fish_seen_subcommand_from init task list rm completions repair help" -f -a "completions"
complete -c claudectl -n "__fish_claudectl_using_subcommand help; and not __fish_seen_subcommand_from init task list rm completions repair help" -f -a "repair"
complete -c claudectl -n "__fish_claudectl_using_subcommand help; and not __fish_seen_subcommand_from init task list rm completions repair help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
