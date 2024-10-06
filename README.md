# What is it

Bee is a small program you can leave running on your server. It will monitor memory usage and alert you via email when it gets too high.

You can configure the alert threshold, memory polling interval and email alert settings.

## Supported operating systems:

FreeBSD, Linux.
  
TODO: OpenBSD, NetBSD.

## Configuration

**Polling interval:** Supply this with a command line argument. `bee -i 5` for every 5 seconds etc.

**Email alerting:** Copy the included `.env.example` to `.env` and set parameters as you want for notifications.