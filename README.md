# logparser
Parse and publish logs

# What is it and why
I'm learning rust for a while and I'm using Logstash a lot for a work. So I decided to try reimplement logstash in rust.
It is not completed yet. But if someone wants to join and make fast log parser - welcome.

# Where is the code
please check the develop branch

# Plans
## Config
First of all we need to have some config. Let it be the standard logstash pipeline config. So we need to build something like computation graph of our parser from config.
- [ ] input
- [x] filter (we can read most of parts from filter module)
- [ ] output
- [x] read if statements and transform it to postfix notation for simpler execution

... and huge amount other things.
