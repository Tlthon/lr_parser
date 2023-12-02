technique

target

progress
 + Abstract Syntax Tree

using technique
 + What jubd if cide
 + What is the current limitation
 + Example of current limitations

Project 2: Visually implement the bottom-up LR(1) parsing algorithm, so that the behavior and data structures of the algorithm, as it parses a string (with respect to a context-free grammar), is displayed in a visually appealling way.  Allow the user to step forwards and backwards in the parsing process.

In your presentation, discuss your progress to date, your current barriers (what stops you from making progress at the moment), and your planned next steps. It will also be good to link your progress to your overall plan to show whether you are on target or need to make more progress.

technical limitation:
 + when there's too much rule / too long input
 + Bad syntax + Contradiction
 + not visually appealling enough

ACCEPT -> S EOF
S -> if (S) then S
S -> 1 + S
S -> 1
S -> 0

ACCEPT -> S
E -> S + S
S -> S + 1
S -> 1