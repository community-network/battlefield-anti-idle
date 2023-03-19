# Battlefield anti-idle

This tool will prevent you from getting kicked for idle for Battlefield 4, 1, 5 and 2042, you only have to run it and it will work even if the game is minimized.
It can also send messages on specified times based on what you set in the config.

download it here: https://github.com/community-network/battlefield-anti-idle/releases/latest

Config file

```bash
# It will autogenerate one of these files when you run the script when it doesn't exist already.

# It will send messages based on timeout when set to true
send_messages = false
# Message it will send
messages = [
    'test1',
    'test2',
    'test3',
]
# In which chat it has to send the messages (can be 'Public', 'Team' or 'Squad')
# The 'Public' chat can only be used by the server owner in Battlefield 2042!
chat_type = 'Public'
# When it will start sending messages, based on the UTC timezone
message_start_time_utc = '12:00'
# When it will stop sending messages, based on the UTC timezone
message_stop_time_utc = '23:00'
# Timeout used when sending messages
message_timeout_mins = 8
```
