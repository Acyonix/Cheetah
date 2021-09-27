# Entry Point

Right so, off the bat I'm having a dilemma: I don't know exactly how I want this application to run.

Ok that's not exactly true, I know that I want it to run like something similar to existing graphical game engines like Unity - you start a new project, add entities and define how they behave, you have a scene viewer where you can actually see what it looks like, and you have a build button where the engine will bundle all of the different assets and scripts together to create the game. That's all well and good, but how exactly do I want it to WORK in Rust?

So the series I'm following is using an inheritance model off the bat - any new game would be a subclass of the engine itself, which works great in an object oriented paradigm but.... Rust isn't exactly an object oriented programming language. There is no inheritance per say, the closest thing Rust has is using Traits, and I'm not sure I particularly want to be doing that here. Then again, perhaps I do...

Ok so my question is: exactly how do I want the user to be interacting with the engine? When they start up a new project, what does it look like?

One starting point could be through scenes. A scene in Unity is simply a bundle of assets working together to create a level in a game, or a credits screen, or a menu screen - essentially it's a part of the game. It's kind of like web pages within a web site: the entire game application is the web site, and a scene is one of the web pages. In this case, there can only be one scene active at a time, but they can certainly link between themselves.

## The Hazel Method

So the way I understand how The Cherno does it is this:

1. The `main()` function is set inside the engine itself, consisting of the following code:

   ```C++
   int main(int argc, char** argv)
   {
       auto app = Hazel::CreateApplication();
       app->Run();
       delete app;
   }
   ```

2. When the user starts making a new game, what they're doing is simply inheriting from the `Hazel::Application` class, which inherits the `main()` function above, and runs that.
3. When the `main()` function runs, it creates an app, in Cherno's case of type Sandbox, and since the `main()` function is run within the Sandbox object, then the application is run as a Sandbox rather than as a generic Hazel application.

To me, this feels a bit strange. I can't exactly explain why, but I'll describe how I think this could work in Rust.

## The Cheetah Method

To start off with, I'll try keeping the `main()` method within the Sandbox. When a user wants to create a new application, they create a new `Application` object, which will be the overarching object that contains all of the state of the application, along with configuration options.

Ok, so the architecture so far looks like this:

- When creating a game, an `Application` object from the `cheetah::core` module is made.
- This `Application` object contains two things so far: A list of `Scene`s that exist in the game, and a separate `Scene` that is currently in effect.

As an aside, this lead to me learning about the Vector type and the Rc<> type in Rust, both useful to store a list of references to `Scene`s, however I ended up deciding that `active_scene` is actually a completely different `Scene` in memory than those in the `scenes` list, specifically because then it would be possible to make a clone of a listed `Scene`, and then adapt it for a specific level.

Also, I've been using a few terms as interchangeable so far, such as game/application and scene/window. I'll try and clarify this now - I'll use the terms application and scene more often, to try and reflect the code.
