YT: https://www.youtube.com/watch?v=jgpVdJB2sKQ (Web DEv Simplified)


# 1. BASICS

```bash
redis-cli

SET name max
GET name # --> max

DEL name # --> 1
GET name # --> (nil)

EXISTS name # --> 0

KEYS * # --> 1) name

```


# 2. HASHSETS

```bash
HSET person name max
HGET person name # --> max

HSET person age 20
HGET person age # --> 20



```
