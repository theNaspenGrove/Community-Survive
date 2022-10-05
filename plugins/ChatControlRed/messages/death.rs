# ---------------------------------------------------------------------------------
# Death messages. Uses the same syntax as files in rules/ folder but operators
# are slightly different. For documentation and a quick tutorial, see:
# https://github.com/kangarko/ChatControl-Red/wiki/messages
# ---------------------------------------------------------------------------------

# They are read like a newspaper and each player only sees one message,
# that is the first one we can send to him.

group default
# You can also broadcast this over BungeeCord
then log {player} has died at {world} {x} {y} {z} by {cause}
then discord 764042558841290752 {original_message} ({x}, {y}, {z})
message:
# Or you can just show the one from Minecraft
- &e{original_message} &6({x}, {y}, {z})&r