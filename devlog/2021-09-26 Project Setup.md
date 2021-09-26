# Project Setup

So, today is the first day of working on the Cheetah engine, and it was all about project setup. In these logs I'll be trying to explain exactly what my game plan is at the end of each day, iterating over my workflow, describing what I changed in the day and making notes about what I would like to change or add in the future.

## Git Workflow

To start with, I'll mention my workflow using Git. Right from the outset, I want two branches:

- A `dev` branch to push to as often as needed, which will be the absolute most current version of this repository, but may not always work perfectly every time. The main purpose of this branch is simply to be the latest save of my work in case of any data loss on my local machine.
- A `main` branch which is essentially the stable version of this project. The goal is that at the end of the day I can merge `dev` with `main`, and it will simply work. The reason that I aim to have only one or so changes to `main` per day is so that it can keep up with these logs - I want one log to essentially correspond to one big change in `main`, so people can see exactly what state the project is in for each entry in this log.

## Project Layout

So far, I have two source files in this project:

- `cheetah.rs`: The main lib file for the Cheetah engine. This will contain or import all of the code for the engine itself, and would be what is exported to `crates.io` if this engine were to become large enough.
- `sandbox.rs`: An executable representing a game being made through Cheetah. This will not be a part of the engine itself, instead it's a representation of what it would look like any time the engine is used out in the world.

## Roadmap For Cheetah

Simply put, I have very little clue where to start! This is why, at least initially, I'll be following along with the general path that The Cherno has laid while making his Hazel engine. At some point, when I have more knowledge or more of an idea where I want to take this, I could branch out, but that will be something to think about in the future. However, I am pretty certain that at some points in his series he may make some design choices that I could disagree with, in which case I may try to find my own way of doing that particular feature. Again, something for the future.

Now it is worth noting that the Hazel engine is actually a C++ project. Because of this, I am fairly certain this project will have some parts that are much easier to set up, and others which could be a lot more challenging, than if I were to simply attempt to make a clone of the Hazel engine. For instance, I don't yet know if I want to try working with essentially bare-metal renderers when building out this engine like OpenGL or Vulcan, or take advantage of higher-level Rust libraries out there like Piston. Likely I'll start off using something like Piston, and only afterwards, when I have more of an idea of what I can accomplish, will I attempt to implement more and more optimizations.

Oh, and before I forget: at this point in time, Cheetah is going to be specifically targeted towards 2D game/application development. I'm not even going to consider 3D rendering at this point. This is an intentional decision from the outset because I know that, at least for now, if I were to build a game using Cheetah it would be a 2D one. Any 3D games I would simply use Unity (or Unreal Engine, but I'm much more familiar with Unity). Perhaps further down the path I could consider adapting Cheetah to implement 3D rendering, or even create a whole new engine specifically designed to work with 3D environments. Perhaps I'll name that one for it's cousin, the Leopard. Who knows.

Anyways, I'll leave it at that for now. Time to see where this goes!
