# advprog8-subscriber

bnuuy


## relfeceton

> 7a. How many data your publisher program will send to the message broker in one run?

The publisher will send 5 packets of data, each containing a `user_created` message each with a `user_id` and `user_name`.
This is indicated in the `main` function, where it calls `p.send` 5 times.

> 7b. The url of: “amqp://guest:guest@localhost:5672” is the same as in the subscriber
program, what does it mean?

It means that the Publisher's message broker is the same as the Subscriber's.
So, they both are connected through the same message broker.