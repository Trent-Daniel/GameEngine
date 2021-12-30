# What is a Game Engine #
It's a platform which you can use to create games. Can also be used to create simulations, animations, etc.

The goal is to transform data from one form into another. We read data from disk and transform it into something else before presenting the data on the screen. Usually, there is a way to interact with the data as well.

The purpose of a game engine is not to create data from nothing, instead it loads data from files. However, a game engine can be a platform to create these data files, often called assets. Usually assets get created in a format, usually custom to the game engine, that the game engine can understand.

# Game Engine Design #

## Entrypoint ##
When we launch the application or game made with the engine, what happens? What controls the main function? Is it something that the client control, or that the engine controls?

## Application Layer ##
A section of our code that deals with application lifecycle and events. What keeps the app open and rendering frames? What keeps time running forward? What manages I/O like from keyboard?

## Window Layer ##
A window exists only on Desktop platforms. It's the target for all of the graphics rendering. This doesn't really matter for mobile or console games.
    - Inputs
    - Events
    - Event Manager will handle inputs and other events
    - One idea is to design the event manager based on a messaging system with subscriptions etc.

## Rendering ##
Draws frames to the screen.

## Render API Abstraction ##
Need the engine to interface with OpenGL, or whatever other rendering API or graphics library we might use. This will make the engine as agnostic as possible

## Debugging Support ##
Building with scaffolding. Need good ways to debug while coding the engine or developing a game. Logging, profiling system, etc..

## Scripting ##
Support a scripting language that artists or content creators can use to create a game instead of having to write in Rust all the time

## Memory Systems ##

## Entity Component Systems ##
The way that the engine relates data.

## Physics ##

## File I/O, Virtual File System ##

## Build System ##
Building the game, converting assets into a format that is compatible with the engine. Allow the engine to grab assets from other applications like photoshop etc.
