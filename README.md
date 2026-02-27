### Crustchan v2

***IMPORTANT*** This repository is far from its first release.  Expect nothing to work here just yet.

## What is it?
Crustchan is an imageboard built for the cloud whose backend is in Rust.  It can be seen as a successor to the unfinished [crustchan](https://github.com/devhax-heavy-industry/crustchan).

## Planned differences from crustchan

* Use of Axum instead of Warp for the API/backend
* Bit of a more coherent API contract
* Use of an RDBMS (Postgres) instead of DynamoDB
* Use of Redis or some KV store (or maybe just read-replicas) to facilitate CQRS, to separate our DB writes from our DB reads
* Avoidance of AWS-specific infrastructure
* This includes maybe making use of RabbitMQ or similar for our events
* (potentially) Introduction of a time-series database to dig into ancient/archieved posts
* (plans to build) An adminstrative UI
* (plans to build) A user UI
* Bonus non-anonymous-only boards, mixed boards, with user registration done vie oauth2 (sign up with your google account, etc)
* (potentially) Rust-react SSR via [Tuono](https://tuono.dev)
* Image optimization of every image uploaded, hope you like some webp - maybe I'll wipe metadata here like geolocation information?
* (maybe down the line) automagic websocket or sse updates for threads/watched threads
* Moving away from AWS Free tier ( They only support a RDBMS for 1 year free anyway)
* Less focus on refined deployment - I might just deploy this to my personal server somewhere, maybe I'll wrap it up in a Portainer stack (Sorry Terraform I still love you)
* I plan to make use of AI Agents to assist in the building of thise software.  This is purely just for my own learning and will try to be steadfast in not creating slop / allowing nonsense through PRs.




## Support the author

Hire me: svajlenka.com Send me some coins:

    bitcoin: 1Lg3YBZvZG2Qan7acbECdJo7Wvoh8pt7E8
    ethereum: 0x4347a1AaE71f5f9DBCa602bB0A0bD856505726Bf
    monero: 87CYcYyWp8RHRGmqVw9XHBf54NGdwYZevLfk7i47FC