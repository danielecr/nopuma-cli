# NoPuppetManager

Based on concept of adaptation of design, this project is composed of three main tools:

- define job steps
- validate procedure
- show existing procedures


> nopuma config --server=https://nopuma-service.domain.com

set the server target uri for communication and return a env variable like this:
export NOPUMASERVER=https://nopuma-service.domain.com

> nopuma config --server=https://nopuma-service.domain.com --store bash


> nopuma proc-define (interactive TUI)
```

/----------------------------\
| amzapi.cronreport.check | 1 |
\ ___________________________/



__________________________________________________________________________
n: new step | e: edit step | [0-9]: select step | v: switch view | s: save
```

Views are:
- tile of steps
- render graph of dependency

Edit mode (is a special view on edit of a selected node):
```
Name: amzapi.cronreport.check
Description: a step to check if a report is required
Policy: regular_q [v]
Delay: ms
expire: ms
reenqueue: ms
transition_to: [proc1.phase1] [proc1.phase2] _____[v] [add]
out_degree: 0 | 1 | n
check_hierarchy_mode: FAIL_PARENT | FAIL_CHILD | IGNORE
```

After 's' (save) it produce a nopuma-procedure.json that can be sent to nopuma for approvation.

The content of nopuma-procedure.json is:
```
[
	{
		"ttype": "amzapi.cronreport.check",
		"policy": {
			"type": "regular_q",
			"delay": 1,
			"expireAfter": 0,
			"reEnqueueAfter": 0
		},
		"transitions": {
			"transition_to": ["amzapi.cronreport.lsaccount"],
			"out_degree": "n",
			"check_hierarchy_mode": "FAIL_PARENT"
		},
		"description": "check which peridical report has to be done and spawn cronreport.lsaccount task"
	},
    ...
]
```
an array of step definition object. The command:

> nopuma [--merge-on nopuma-procedure-merge.json] proc-verify nopuma-procedure.json

returns conflict list (for policy and transitions), with the option
`--merge-on nopuma-procedure-merge.json` the service try to merge
missing `transition_to` and write it in file nopuma-procedure-merge.json

The command:

> nopuma proc-commit procname nopuma-procedure.json

where 'procname' is the name of the new procedure,
send procedure, that is accepted only if there are no conflict and user has credential for it.

> nopuma role-subscription
```

/----------------------------\
| amzapi.cronreport.check | 1 | producer
\ ___________________________/



________________________________________________________________
c/p: consumer/producer | [0-9]: select step | v: switch view
```

produce a `role-subscription.json` file

the command:

> nopuma role-subrequest role-subscrition.json PUBKEY

send the role-subscription.json for approvation, it returns a signed certificate that can be used by
the new client.

