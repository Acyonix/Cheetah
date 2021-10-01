# Event System

## What I Barely Understand

Ok so today's the start of working on the event system, and at this point I'm already a bit lost. I understand that, thinking object oriented for the moment, that there's an `Event` type, which has a bunch of subclasses that define the exact type of event, such as a `KeyEvent`, or a `MouseEvent`, and these different types have their own nuances with data that they would provide, along with some shared behaviour such as being able to say whether they're handled or not.

## Now The Rest

And now I'm basically lost. An `Event` is created somehow (this is one thing I need to figure out) and sent to an `EventDispatcher`, which then dispatches the `Event` to the engine itself, which can then handle the `Event`. I think that's what the gist of it is, but then I'm also just plain not sure! I'll have to do some more planning maybe before going too much further, before I do a lot of work that I didn't need to do.

## Benefits of Today

On the plus side, I did end up making a neat little wrapper module called `bitmap`. I looked online but I don't thing the existing crates were quite what I was looking for, so I implemented one myself. Essentially what it does is just wraps around a `u64`, so it's a bitmap that can store up to 64 flags. It's small, simple, gets the job done and is easy to add to for extra functionality as called for. Come to think of it, I might just straight up call it `bitmap64`, build it up a little, and see if I can try publishing it on crates.io. Then again, there are quite a few bitmap libraries already there... I dunno...

Gah, you know what, there's no crate called `bitmap64` yet so I'm just gonna go ahead and do it. Can't hurt and it's simple enough!

Anyway, I'm gonna call that a day for this project. I'll see how much planning I can do later on.
